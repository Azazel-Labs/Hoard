
use super::*;

fn json(sel: &str, doc: &str) -> Vec<String> {
    let v: Value = serde_json::from_str(doc).unwrap();
    Sel::parse(sel, Syntax::Json).unwrap().strings(Node::Json(&v))
}

#[test]
fn paths_find_values() {
    let doc = r##"{"a": {"name": "N", "minSize": "1 m", "Tags": ["x", "y"], "flag": true},
                      "meta": [{"key": "scanArea", "value": "2x2 m"}, {"key": "Height", "value": "2cm"}],
                      "list": [{"uri": "a.jpg"}, {"uri": "b.jpg"}], "color": "#fff", "cats": ["surface_x", "soil"]}"##;
    assert_eq!(json("a.name", doc), ["N"]);
    assert_eq!(json("a.tags", doc), ["x", "y"], "keys ignore case when no key matches exactly");
    assert_eq!(json("a.*size*", doc), ["1 m"]);
    assert_eq!(json("a.*!(name,*size*)[array]", doc), ["x", "y"]);
    assert_eq!(json("meta[key=scanarea].value", doc), ["2x2 m"]);
    assert_eq!(json("meta[key=width|height].value", doc), ["2cm"]);
    assert_eq!(json("list.uri", doc), ["a.jpg", "b.jpg"], "a key steps into each item of an array");
    assert_eq!(json("list[].uri", doc), ["a.jpg", "b.jpg"]);
    assert_eq!(json("color !~ #*", doc), Vec::<String>::new());
    assert_eq!(json("cats ~ surface*", doc), ["surface_x"]);
    assert_eq!(json("nothing.here", doc), Vec::<String>::new());

    let v: Value = serde_json::from_str(doc).unwrap();
    let holds = |s: &str| Sel::parse(s, Syntax::Json).unwrap().holds(Node::Json(&v));
    assert!(holds("a.flag") && holds("cats ~ surface*") && !holds("cats ~ rock*") && !holds("a.missing"));
}

#[test]
fn bad_selectors_say_why() {
    for bad in ["a..b", "a[key=]", "a[wat]", "a[", "name!(x)", "a.b)"] {
        assert!(Sel::parse(bad, Syntax::Json).is_err(), "{bad}");
    }
    assert!(Sel::parse("(unclosed", Syntax::Text).is_err());
    let e = Format::parse("name = \"X\"\nfiles = [\"*.json\"]\n[fields]\ntitel = \"name\"").unwrap_err();
    assert!(format!("{e:#}").contains("titel"), "{e:#}");
}

#[test]
fn text_patterns_take_their_first_group() {
    let doc = Doc::Text("* title:\tRound thing\n* author:\tSomeone (https://example.com/someone)\n".to_owned());
    let p = |s: &str| Sel::parse(s, Syntax::Text).unwrap().strings(doc.root());
    assert_eq!(p(r"^\* title:[ \t]*(.+?)[ \t]*$"), ["Round thing"]);
    assert_eq!(p(r"^\* author:[ \t]*(.+?)(?:[ \t]+\(https?://[^)]*\))?[ \t]*$"), ["Someone"]);
    assert_eq!(p(r"^\* source:[ \t]*(\S+)"), Vec::<String>::new());
}

#[test]
fn globs() {
    assert!(glob("*.json", "a.json") && glob("license.txt", "license.txt") && glob("*min*size*", "minsize"));
    assert!(!glob("*.json", "a.json5") && !glob("a?c", "ac") && glob("a*b*c", "aXbYbc"));
    let f = Format::parse("name = \"X\"\nfiles = [\"*.json\", \"license.txt\"]").unwrap();
    assert!(f.wants("A/B.JSON") && f.wants("License.txt") && !f.wants("a/licenses.txt"));
    assert_eq!(f.like_patterns(), ["%.json", "%/license.txt", "license.txt"]);
}

/// Every file in `formats/` is compiled in and reads.
#[test]
fn every_format_is_built_in() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("formats");
    let mut files: Vec<String> = std::fs::read_dir(&dir)
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| Path::new(n).extension().is_some_and(|e| e == "toml"))
        .collect();
    files.sort();
    let built: Vec<&str> = BUILT_IN.iter().map(|(f, _)| *f).collect();
    assert_eq!(files, built, "add new formats to BUILT_IN in store_formats.rs, in name order");
    for (file, text) in BUILT_IN {
        if let Err(e) = Format::parse(text) {
            panic!("formats/{file}: {e:#}");
        }
    }
}

#[test]
fn slugs() {
    assert_eq!(slug("Quixel Bridge"), "quixel-bridge");
    assert_eq!(slug("  Itch.io (assets)!"), "itch-io-assets");
    assert_eq!(slug("Megascans"), "megascans");
    assert_eq!(slug("ÆØ"), "");
}

/// Each sample in `formats/samples/<slug of the format's name>/` reads as that format, and gives what its `.expected.json` says
/// (only the keys it lists are compared).
#[test]
fn samples_read_as_expected() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("formats/samples");
    let mut n = 0;
    for format_dir in std::fs::read_dir(&dir).unwrap().flatten() {
        let format = format_dir.file_name().to_string_lossy().into_owned();
        for file in std::fs::read_dir(format_dir.path()).unwrap().flatten() {
            let name = file.file_name().to_string_lossy().into_owned();
            if name.ends_with(".expected.json") {
                continue;
            }
            let text = std::fs::read_to_string(file.path()).unwrap();
            let expected_path = file.path().with_file_name(format!("{name}.expected.json"));
            let expected: Value = serde_json::from_str(
                &std::fs::read_to_string(&expected_path)
                    .unwrap_or_else(|_| panic!("{} has no {name}.expected.json beside it", file.path().display())),
            )
            .unwrap();
            let got = crate::sidecar::read(&name, &text)
                .unwrap_or_else(|| panic!("samples/{format}/{name} isn't read as any format"));
            let got = serde_json::to_value(&got).unwrap();
            let pretty = serde_json::to_string_pretty(&got).unwrap();
            assert!(got["format"].as_str().is_some_and(|f| slug(f) == format), "{pretty}");
            for (k, v) in expected.as_object().unwrap() {
                assert_eq!(&got[k], v, "samples/{format}/{name}: {k}\nread as:\n{pretty}");
            }
            n += 1;
        }
    }
    assert!(n >= 2);
}
