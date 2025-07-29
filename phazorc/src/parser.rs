use crate::ast::View;
use tree_sitter::{Language, Parser};

// Link to the generated parser in parser.c
unsafe extern "C" {
    fn tree_sitter_phz() -> Language;
}

// Return the Tree-sitter language
pub fn language() -> Language {
    unsafe { tree_sitter_phz() }
}

/// Parses a `.phz` string into a `View` struct.
/// 
pub fn extract_view(source: &str) -> Option<View> {
    let mut parser = Parser::new();
    parser.set_language(language()).ok()?; // load phz grammar

    let tree = parser.parse(source, None)?;
    let root_node = tree.root_node();

    // Find the first `statement` node
    for i in 0..root_node.child_count() {
        let node = root_node.child(i)?;
        if node.kind() == "statement" {
            let mut route = String::new();
            let mut name = String::new();
            let mut html = String::new();

            for j in 0..node.child_count() {
                let child = node.child(j)?;
                match child.kind() {
                    "string" => {
                        let text = child.utf8_text(source.as_bytes()).ok()?;
                        route = text.trim_matches('"').to_string(); // Remove quotes
                    }
                    "identifier" => {
                        name = child.utf8_text(source.as_bytes()).ok()?.to_string();
                    }
                    "html_block" => {
                        html = child.utf8_text(source.as_bytes()).ok()?.to_string();
                    }
                    _ => {}
                }
            }

            return Some(View {
                name,
                route,
                html,
            });
        }
    }

    None
}
