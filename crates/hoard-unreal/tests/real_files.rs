//! Checks against real packages, when `HOARD_UASSET_DIR` points at some (they aren't redistributable, so none are
//! checked in). Every export of every versioned editor package must parse to its terminating `None` inside the
//! export, and every main asset must convert or report why not.

#![expect(clippy::unwrap_used, reason = "test helpers")]

use std::path::{Path, PathBuf};

use hoard_unreal::convert::{self, Outcome};
use hoard_unreal::props;

fn packages(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(d).unwrap().flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "uasset" || x == "umap") {
                files.push(p);
            }
        }
    }
    files.sort();
    files
}

#[test]
fn properties_and_conversion() {
    let Some(dir) = std::env::var_os("HOARD_UASSET_DIR") else { return };
    let out = std::env::var_os("HOARD_UASSET_OUT").map(PathBuf::from);
    let (mut exports, mut bad) = (0, Vec::new());
    for f in packages(Path::new(&dir)) {
        let data = std::fs::read(&f).unwrap();
        let Ok(pkg) = hoard_unreal::parse(&data) else { continue };
        if pkg.cooked {
            // Exports live in a separate .uexp file, and editor data is stripped anyway.
            assert!(matches!(convert::convert(&data, &pkg), Outcome::Unsupported(_)));
            continue;
        }
        for e in &pkg.exports {
            exports += 1;
            match props::read_export(&data, &pkg, e) {
                Ok(p) => {
                    let end = usize::try_from(e.serial_offset + e.serial_size).unwrap();
                    if p.end > end {
                        bad.push(format!("{}: {} ran past its export", f.display(), e.name));
                    }
                    let unread: Vec<&String> =
                        p.values.iter().filter(|(_, v)| v.get("unread").is_some()).map(|(k, _)| k).collect();
                    if !unread.is_empty() {
                        eprintln!("     {} {}: skipped {unread:?}", f.display(), e.name);
                    }
                }
                Err(err) => bad.push(format!("{}: {}: {err:#}", f.display(), e.name)),
            }
        }
        let outcome = convert::convert(&data, &pkg);
        let class = pkg.class.as_deref().unwrap_or("?");
        match &outcome {
            Outcome::Converted { file, notes } => {
                eprintln!("conv {} [{class}] -> .{} {} bytes {notes:?}", f.display(), file.ext, file.bytes.len());
                if let Some(out) = &out {
                    std::fs::create_dir_all(out).unwrap();
                    let name = format!("{}.{}", f.file_stem().unwrap().to_string_lossy(), file.ext);
                    std::fs::write(out.join(name), &file.bytes).unwrap();
                }
            }
            Outcome::NeedsEditor(why) => eprintln!("edit {} [{class}] {why}", f.display()),
            Outcome::Unsupported(why) => eprintln!("skip {} [{class}] {why}", f.display()),
            Outcome::Failed(why) => bad.push(format!("{} [{class}] convert: {why}", f.display())),
        }
    }
    eprintln!("{exports} exports read");
    for b in &bad {
        eprintln!("BAD  {b}");
    }
    assert!(bad.is_empty(), "{} problems", bad.len());
}
