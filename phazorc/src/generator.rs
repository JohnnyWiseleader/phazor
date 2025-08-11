use crate::ast::View;
use convert_case::{Case, Casing};
use regex::Regex;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn write_component(view: &View, output_dir: &Path) {
    let struct_name = &view.name.to_case(Case::Pascal);
    let name = &view.name;
    let html = to_html_expr_lines(view);

    // format componenet code for output
    let component_code = if view.props.is_empty() {
        format!(
            r#"
use yew::prelude::*;

#[function_component({struct_name})]
pub fn {name}() -> Html {{
    html! {{
    <>
{html}
    </>
    }}
}}
"#,
            struct_name = struct_name,
            name = name,
            html = html
        )
    } else {
        let props = view
            .props
            .iter()
            .map(|p| format!("    pub {}: String,", p))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            r#"use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct {struct_name}Props {{
{props}
}}

#[function_component({struct_name})]
pub fn {name}(props: &{struct_name}Props) -> Html {{
    html! {{
    <>
{html}
    </>
    }}
}}
"#,
            struct_name = struct_name,
            name = name,
            html = html,
            props = props
        )
    };

    let filename = output_dir.join(format!("{}.rs", view.name));
    fs::write(&filename, component_code).expect("Failed to write component file");
    println!("Generated component: {}", filename.display());
}

pub fn write_router(views: &[View], output_dir: &Path) {
    let mut enum_variants = Vec::new();
    let mut match_arms = Vec::new();

    for view in views {
        let variant = view.name.to_case(Case::Pascal);
        let route_name = view.name.to_lowercase();
        if route_name == "home" {
            // Add alias for root path
            enum_variants.push(format!("    #[at(\"/\")]\n    Home,"));
        } else {
            let mut at_props = String::from("");
            let mut vars_names = String::from("");
            let mut match_pat = String::from("");
            let mut switch_props = String::from("");
            for (i, p) in view.props.iter().enumerate() {
                if i == 0 { // first element in the loop 
                    vars_names.push_str(" { ");
                    match_pat.push_str(" { ");
                }
                at_props.push('/');
                at_props.push(':');
                at_props.push_str(p);
                vars_names.push_str(p);
                match_pat.push_str(p);
                vars_names.push_str(": String, "); // all vars are of type string for now
                match_pat.push_str(", ");
                switch_props.push_str(p);
                switch_props.push_str("={");
                switch_props.push_str(p);
                switch_props.push_str("} ");
                if i == view.props.len() - 1 { // last element in the loop
                    vars_names.pop(); // remove the last comma and space
                    vars_names.pop();
                    vars_names.push_str(" }"); // add closing brace
                    match_pat.pop(); // remove the last comma and space
                    match_pat.pop();
                    match_pat.push_str(" }"); // add closing brace
                }
            }
            enum_variants.push(format!(
                "    #[at(\"/{route_name}{at_props}\")]\n    {variant}{vars_names},"
            ));
            match_arms.push(format!(
                "        Route::{}{} => html! {{ <generated::{}::{} {}/> }},",
                variant, match_pat, route_name, variant, switch_props
            ));
        }
    }

    let router_code = format!(
        r#"
use yew::prelude::*;
use yew_router::prelude::*;
use crate::generated;

#[derive(Routable, Clone, PartialEq, Eq, Debug)]
pub enum Route {{
{variants}
    #[not_found]
    #[at("/404")]
    NotFound,
}}

pub fn switch(route: Route) -> Html {{
    match route {{
{matches}
        Route::Home => html! {{ <generated::home::Home /> }},
        Route::NotFound => html! {{ <h1>{{ "404 Not Found" }}</h1> }},
    }}
}}
"#,
        variants = enum_variants.join("\n"),
        matches = match_arms.join("\n")
    );

    let filename = output_dir.join("router.rs");
    fs::write(&filename, router_code).expect("Failed to write router.rs");
    println!("Generated router: {}", filename.display());
}

pub fn update_mod_rs(output_dir: &Path, view_name: &String) {
    let mod_rs_path = output_dir.join("mod.rs");
    let mod_line = format!("pub mod {};", view_name);

    let current_contents = fs::read_to_string(&mod_rs_path).unwrap_or_default();
    if !current_contents.contains(&mod_line) {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&mod_rs_path)
            .expect("Failed to open or create mod.rs");
        writeln!(file, "{}", mod_line).expect("Failed to write to mod.rs");
    }
}

// Converts raw HTML with `{prop}` placeholders into Yew-friendly
// HTML with prop interpolation.
fn to_html_expr_lines(view: &View) -> String {
    let html = view.html.trim();
    let props = &view.props;
    let prop_pattern = Regex::new(r"\{(\w+)\}").unwrap();
    let tag_pattern = Regex::new(r"^(<[^>]+>)(.*)(</[^>]+>)$").unwrap();

    html.lines()
        .map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return String::new();
            }

            if let Some(caps) = tag_pattern.captures(trimmed) {
                let open_tag = &caps[1];
                let inner = &caps[2];
                let close_tag = &caps[3];

                // If the inner content includes placeholders, use format!
                if prop_pattern.is_match(inner) {
                    let mut format_string = String::new();
                    let mut format_args = Vec::new();
                    let mut last_index = 0;

                    for caps in prop_pattern.captures_iter(inner) {
                        if let Some(m) = caps.get(0) {
                            let prop_name = &caps[1];
                            if props.contains(&prop_name.to_string()) {
                                format_string.push_str(&inner[last_index..m.start()]);
                                format_string.push_str("{}");
                                format_args.push(format!("props.{}", prop_name));
                                last_index = m.end();
                            }
                        }
                    }

                    format_string.push_str(&inner[last_index..]);

                    format!(
                        "        {}{{ format!(\"{}\"{}) }}{}",
                        open_tag,
                        format_string,
                        format_args
                            .iter()
                            .map(|arg| format!(", {}", arg))
                            .collect::<String>(),
                        close_tag
                    )
                } else {
                    // Static inner text
                    format!("        {}{{\"{}\"}}{}", open_tag, inner, close_tag)
                }
            } else {
                // Line doesn't match expected tag format — fallback
                format!("        {{\"{}\"}}", trimmed)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
