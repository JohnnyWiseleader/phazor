use phazorc::generator::{write_component, write_router};
use phazorc::parser::parse_phz;
use std::fs;
use std::path::PathBuf;

fn main() {
    let input_dir = PathBuf::from("views");
    let output_dir = PathBuf::from("../phazor_frontend/src/generated");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");
    let mut views = Vec::new();

    if !input_dir.exists() {
        eprintln!("Missing views/ directory");
        return;
    }

    for entry in fs::read_dir(&input_dir).expect("Failed to read views directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("phz") {
                let source = fs::read_to_string(&path).expect("Failed to read .phz file");
                match parse_phz(&source) {
                    Some(view) => {
                        println!("Parsed view: {:?}", view.name);
                        write_component(&view, &output_dir);
                        views.push(view);
                    }
                    None => eprintln!("Failed to parse file: {}", path.display()),
                }
            }
        }
    }
    write_router(&views, &output_dir);
}
