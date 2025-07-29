#![allow(dead_code)]    // not used but archived for possible future use
use tree_sitter::{Node, TreeCursor};

pub fn extract_routes(source: &str, root: Node) {
    let mut cursor = root.walk();

    fn walk_tree(source: &str, cursor: &mut TreeCursor) {
        loop {
            let node = cursor.node();

            if node.kind() == "statement" {
                // let mut child_cursor = node.walk();
                let mut route = "";
                let mut name = "";
                let mut html = "";

                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();
                    match child.kind() {
                        "string" => route = child.utf8_text(source.as_bytes()).unwrap_or(""),
                        "identifier" => name = child.utf8_text(source.as_bytes()).unwrap_or(""),
                        "html_block" => html = child.utf8_text(source.as_bytes()).unwrap_or(""),
                        _ => {}
                    }
                }

                println!("Route: {}\nName: {}\nHTML: {}\n", route, name, html);
            }

            if cursor.goto_first_child() {
                walk_tree(source, cursor);
                cursor.goto_parent();
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    walk_tree(source, &mut cursor);
}
