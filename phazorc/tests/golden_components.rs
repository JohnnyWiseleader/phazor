use std::fs;
use std::path::PathBuf;
use phazorc::parser::extract_view;
use phazorc::generator::{write_component, write_router};
use pretty_assertions::assert_eq;
use tempfile::tempdir;

#[test]
fn component_views_match_expected() {
    let views_dir = PathBuf::from("views");

    for entry in fs::read_dir(&views_dir).expect("Failed to read views directory") {
            if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("phz") {
                let view_source = fs::read_to_string(&path).expect("Failed to read .phz file");

                // generate into temp dir
                let tmp = tempdir().expect("tempdir");
                let out_dir = tmp.path();

                // load .phz
                if let Some(view) = extract_view(&view_source) {
                    write_component(&view, &out_dir);
                    let component_rs =  view.name + ".rs";

                    let actual = fs::read_to_string(out_dir.join(&component_rs)).expect("read generated");
                    let expected = read_fixture(&("expected/".to_owned() + &component_rs));

                    assert_eq!(normalize(&actual), normalize(&expected));
                } else {
                    eprintln!("Failed to parse file: {}", path.display());
                }
            }
        }
    }
}

#[test]
fn router_matches_expected() {
    let views_dir = PathBuf::from("views");
    let mut views = Vec::new();

    for entry in fs::read_dir(&views_dir).expect("Failed to read views directory") {
            if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("phz") {
                let view_source = fs::read_to_string(&path).expect("Failed to read .phz file");

                // load .phz
                if let Some(view) = extract_view(&view_source) {
                    views.push(view);
                } else {
                    eprintln!("Failed to parse file: {}", path.display());
                }
            }
        }
    }

    let tmp = tempdir().expect("tempdir");
    let out_dir = tmp.path();
    write_router(&views, out_dir);

    let actual = fs::read_to_string(out_dir.join("router.rs")).expect("read router.rs");
    let expected = read_fixture("expected/router.rs");

    assert_eq!(normalize(&actual), normalize(&expected));
}

fn fixtures_dir() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests/fixtures");
    p
}

fn normalize(s: &str) -> String {
    // Normalize line endings & trim trailing spaces per line,
    // so formatting/OS differences don't cause false negatives.
    s.replace("\r\n", "\n")
        .lines()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}

fn read_fixture(rel: &str) -> String {
    let mut p = fixtures_dir();
    p.push(rel);
    fs::read_to_string(p).expect("read fixture")
}
