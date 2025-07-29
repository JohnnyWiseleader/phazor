// use std::env;
use std::path::PathBuf;

fn main() {
    let grammar_dir = PathBuf::from("../tree-sitter-phz/src");

    println!("cargo:rerun-if-changed={}", grammar_dir.display());

    cc::Build::new()
        .include(&grammar_dir)
        .file(grammar_dir.join("parser.c"))
        .compile("tree-sitter-phz");

    // Optional: if you have a scanner.c
    let scanner_path = grammar_dir.join("scanner.c");
    if scanner_path.exists() {
        cc::Build::new()
            .include(&grammar_dir)
            .file(scanner_path)
            .compile("tree-sitter-phz-scanner");
    }
}
