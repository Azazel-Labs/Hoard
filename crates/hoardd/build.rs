//! Checks the Fluent catalogs in `locales/` (each has every English message, and no variable the English doesn't pass)
//! and embeds them, so a broken translation fails the build instead of a request.
use std::{env, fmt::Write as _, fs, path::PathBuf};

#[path = "../../localization/catalog.rs"]
mod catalog;

fn main() -> catalog::Result<()> {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let locales = manifest.join("../../locales");
    println!("cargo:rerun-if-changed={}", locales.display());
    println!("cargo:rerun-if-changed={}", manifest.join("../../localization/catalog.rs").display());
    let catalogs = catalog::load(&locales)?;
    let mut code = String::from("static CATALOGS: &[(&str, &str)] = &[\n");
    for (locale, path) in catalog::files(&locales)? {
        println!("cargo:rerun-if-changed={}", path.display());
        let path = fs::canonicalize(&path)?;
        writeln!(code, "    ({locale:?}, include_str!({:?})),", path.display().to_string())?;
    }
    code.push_str("];\nstatic LANGUAGES: &[&str] = &[");
    for locale in catalogs.keys() {
        write!(code, "{locale:?}, ")?;
    }
    code.push_str("];\n");
    fs::write(PathBuf::from(env::var("OUT_DIR")?).join("catalogs.rs"), code)?;
    Ok(())
}
