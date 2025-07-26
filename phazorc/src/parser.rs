use crate::ast::View;

pub fn parse_phz(source: &str) -> Option<View> {
    let lines: Vec<&str> = source.lines().collect();

    let mut route = String::new();
    let mut name = String::new();
    let mut html = String::new();

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("@route") {
            if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed.rfind('"') {
                    route = trimmed[start + 1..end].to_string();
                }
            }
        } else if trimmed.starts_with("def ") {
            if let Some(paren) = trimmed.find('(') {
                name = trimmed[4..paren].trim().to_string();
            }
        } else if trimmed.starts_with('<') {
            html = lines[i..].join("\n");
            break;
        }
    }

    if !name.is_empty() && !route.is_empty() && !html.is_empty() {
        Some(View { name, route, html })
    } else {
        None
    }
}
