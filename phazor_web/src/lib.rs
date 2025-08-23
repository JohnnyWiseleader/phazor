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

// top of file (dev-only tuning)
const DRAIN_EVERY_MS: u32 = 2_000; // set to 2_000 for normal, 60_000 for debug

#[derive(Clone)]
pub struct OutboxContext(pub std::sync::Arc<Outbox>);

impl PartialEq for OutboxContext {
    fn eq(&self, other: &Self) -> bool { std::sync::Arc::ptr_eq(&self.0, &other.0) }
}

#[function_component(App)]
fn app() -> Html {
    let api = use_mut_ref(|| {
        cfg_if! {
            if #[cfg(all(target_arch="wasm32", feature="idb-store", feature="rest-http-wasm"))] {
                Arc::new(Outbox::dev_idb_http("http://localhost:3000", "phazor_outbox"))
            } else if #[cfg(all(target_arch="wasm32", feature="rest-http-wasm"))] {
                Arc::new(Outbox::dev_mem_http("http://localhost:3000"))
            } else if #[cfg(all(target_arch="wasm32", feature="fake"))] {
                Arc::new(Outbox::dev_mem_fake(2))
            } else if #[cfg(not(target_arch="wasm32"))] {
                compile_error!("phazor_web must be built for wasm32 (use trunk serve/cargo build --target wasm32-unknown-unknown).");
            } else {
                compile_error!("On wasm32, enable one of: idb-store+rest-http-wasm, rest-http-wasm, or fake.");
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
                    // This drains the queue every DRAIN_EVERY_MS interval
                    // After this fires data in the IndexedDB will be empty
                    gloo_timers::future::TimeoutFuture::new(DRAIN_EVERY_MS).await;
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
