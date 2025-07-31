use phazorc::generator::{update_mod_rs, write_component, write_router};
use phazorc::parser::{extract_view, language};
use std::fs;
use std::path::PathBuf;
use tree_sitter::Parser;

fn main() {
    let input_dir = PathBuf::from("views");
    let output_dir = PathBuf::from("../phazor_frontend/src/generated");
    // let output_dir = PathBuf::from("../phazorc/staging");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    if !input_dir.exists() {
        eprintln!("Missing views/ directory");
        return;
    }

    let mut views = Vec::new();

    // Initialize tree-sitter parser
    let language = language(); // from parser.rs
    let mut parser = Parser::new();
    parser
        .set_language(language)
        .expect("Error loading tree-sitter language");

    for entry in fs::read_dir(&input_dir).expect("Failed to read views directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("phz") {
                let source = fs::read_to_string(&path).expect("Failed to read .phz file");

                if let Some(view) = extract_view(&source) {
                    println!("Parsed view: {:?}", view.name);
                    update_mod_rs(&output_dir, &view.name);
                    write_component(&view, &output_dir);
                    views.push(view);
                } else {
                    eprintln!("Failed to parse file: {}", path.display());
                }
            }
        }
    }
    write_router(&views, &output_dir);
    update_mod_rs(&output_dir, &"router".to_string());
}
