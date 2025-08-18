use std::sync::Arc;
use wasm_bindgen::prelude::*;
use web_sys::window;
use yew::prelude::*;
use yew::Renderer;
use yew::context::ContextProvider;
use yew_router::prelude::*;
use log::info;
use cfg_if::cfg_if;

mod components;
mod hooks;

use phazor_core::outbox::Outbox; // Outbox via re-export
use components::router::{Route, switch};

#[derive(Clone)]
pub struct OutboxContext(pub std::sync::Arc<Outbox>);

impl PartialEq for OutboxContext {
    fn eq(&self, other: &Self) -> bool { std::sync::Arc::ptr_eq(&self.0, &other.0) }
}

#[function_component(App)]
fn app() -> Html {
    let api = use_mut_ref(|| {
        cfg_if! {
            if #[cfg(all(target_arch = "wasm32", feature = "rexie-sink"))] {
                Arc::new(Outbox::dev_mem_rexie("phazor_dev"))
            } else if #[cfg(feature = "fake")] {
                Arc::new(Outbox::dev_mem_fake(2)) // e.g. fail twice then OK
            } else {
                compile_error!("Enable either the `fake` or `rexie-sink` feature for phazor_web.");
            }
        }
    });

    // Background loop (with cleanup)
    use_effect_with((), {
        let api = api.clone();
        move |_| {
            let stop = std::rc::Rc::new(std::cell::Cell::new(false));
            let stop2 = stop.clone();
            wasm_bindgen_futures::spawn_local(async move {
                while !stop2.get() {
                    let _ = api.borrow().drain_once().await; // façade forwards to service
                    gloo_timers::future::TimeoutFuture::new(2000).await;
                }
            });
            move || stop.set(true)
        }
    });

    // Kick a drain when the browser comes online
    use_effect_with((), {
        let api = api.clone();
        move |_| {
            let win = web_sys::window().expect("window");
            let listener = gloo_events::EventListener::new(&win, "online", move |_| {
                let api = api.clone();
                wasm_bindgen_futures::spawn_local(async move { let _ = api.borrow().drain_once().await; });
            });
            move || drop(listener)
        }
    });

    // Provide the outbox service to pages
    let ctx = OutboxContext(api.borrow().clone());
    info!("App rendered!");
    html! {
        <ContextProvider<OutboxContext> context={ctx}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<OutboxContext>>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    // Show debug logs in Chrome DevTools
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::info!("Outbox sink = {}", phazor_core::outbox::Outbox::sink_name());    

    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once(); // nice errors during dev only

    let document = window().unwrap().document().unwrap();
    let root = document.get_element_by_id("root").expect("#root not found");
    Renderer::<App>::with_root(root).render();
}
