use std::sync::Arc;
use wasm_bindgen::prelude::*; // #[wasm_bindgen(start)]
//use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::Renderer;
use yew::prelude::*;
use yew_router::prelude::*; // BrowserRouter, Switch, Routable, etc.

//use gloo_events::EventListener;
//use gloo_timers::future::TimeoutFuture;

use log::info;

use phazor_core::datasink::fake::FailThenOkSink;
use phazor_core::outbox::{service::OutboxService, store_mem::MemStore};
mod components;
use components::router::{Route, switch};

#[function_component(App)]
fn app() -> Html {
    // Singletons
    let svc = use_mut_ref(|| {
        let store = Arc::new(MemStore::default());
        let sink = Arc::new(FailThenOkSink::new(2)); // swap to real sink when ready
        Arc::new(OutboxService::new(store, sink))
    });

    // Background loop (with cleanup)
    use_effect_with((), {
        let svc = svc.clone();
        move |_| {
            let stop = std::rc::Rc::new(std::cell::Cell::new(false));
            let stop2 = stop.clone();
            wasm_bindgen_futures::spawn_local(async move {
                while !stop2.get() {
                    let _ = svc.borrow().drain_once().await;
                    gloo_timers::future::TimeoutFuture::new(2000).await;
                }
            });
            // cleanup on unmount
            move || stop.set(true)
        }
    });

    // Online/offline: kick a drain when you come back online
    use_effect_with((), {
        let svc = svc.clone();
        move |_| {
            let win = web_sys::window().expect("window");
            let listener = gloo_events::EventListener::new(&win, "online", move |_| {
                let svc = svc.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let _ = svc.borrow().drain_once().await;
                });
            });
            // cleanup
            move || drop(listener)
        }
    });

    info!("App rendered!");
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    // Console logging
    wasm_logger::init(wasm_logger::Config::default());
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let document = window().unwrap().document().unwrap();
    let root = document.get_element_by_id("root").expect("#root not found");

    Renderer::<App>::with_root(root).render();
}
