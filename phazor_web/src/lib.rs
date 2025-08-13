use log::info;
use wasm_bindgen::prelude::*;
use wasm_logger;
use web_sys::window;
use yew::Renderer;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::router::{Route, switch};

#[function_component(App)]
pub fn app() -> Html {
    info!("App rendered!");
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());

    let document = window().unwrap().document().unwrap();
    let root = document.get_element_by_id("root").unwrap();

    Renderer::<App>::with_root(root).render();
}
