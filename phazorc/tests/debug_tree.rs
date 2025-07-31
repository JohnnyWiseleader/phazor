use tree_sitter::{Parser, TreeCursor};
use phazorc::parser::language;

fn print_tree(source: &str) {
    let mut parser = Parser::new();
    parser.set_language(language()).expect("Failed to load phz language");
    let tree = parser.parse(source, None).expect("Parse failed");
    let mut cursor = tree.root_node().walk();

    fn walk(cursor: &mut TreeCursor, source: &str, indent: usize) {
        loop {
            let node = cursor.node();
            let text = node.utf8_text(source.as_bytes()).unwrap_or("<invalid>");
            println!(
                "{:indent$}- {}: `{}`",
                "",
                node.kind(),
                text.trim().replace("\n", "\\n"),
                indent = indent * 2
            );

            if cursor.goto_first_child() {
                walk(cursor, source, indent + 1);
                cursor.goto_parent();
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    walk(&mut cursor, source, 0);
}

#[test]
fn debug_sample_phz_tree() {
    println!("\n\n\n### Begin Walk Of Tree ###\n\n");
    let source = r#"
# A comment
@route "/hello"
@props name, age
def hello():
    <h1>Hello, {name}!</h1>
    <p>{age} years of wisdom!</p>
"#;
    print_tree(source);
}
