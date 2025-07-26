use crate::ast::View;
use convert_case::{Case, Casing};
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn write_component(view: &View, output_dir: &Path) {
    let struct_name = view.name.to_case(Case::Pascal);

    let component_code = format!(
        r#"
use yew::prelude::*;

#[function_component({struct_name})]
pub fn {name}() -> Html {{
    html! {{
        {html}
    }}
}}
"#,
        struct_name = struct_name,
        name = view.name,
        html = format!("{{ r#\"{}\"# }}", view.html.trim())
    );

    let filename = output_dir.join(format!("{}.rs", view.name));
    fs::write(&filename, component_code).expect("Failed to write component file");
    println!("Generated component: {}", filename.display());

    update_mod_rs(output_dir, &view.name);
}

pub fn write_router(views: &[View], output_dir: &Path) {
    let mut enum_variants = Vec::new();
    let mut match_arms = Vec::new();

    for view in views {
        let variant = view.name.to_case(Case::Pascal);
        if view.name.to_lowercase() == "home" {
            // Add alias for root path
            enum_variants.push(format!("    #[at(\"/\")]"));
        }
        enum_variants.push(format!("    #[at(\"{}\")]\n    {},", view.route, variant));

        match_arms.push(format!(
            "        Route::{} => html! {{ <generated::{}::{} /> }},",
            variant, view.name, variant
        ));
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

fn update_mod_rs(output_dir: &Path, view_name: &str) {
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
