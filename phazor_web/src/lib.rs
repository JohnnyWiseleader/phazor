use std::sync::Arc;
use wasm_bindgen::prelude::*; // #[wasm_bindgen(start)]
use web_sys::window;
use yew::Renderer;
use yew::context::ContextProvider;
use yew::prelude::*;
use yew_router::prelude::*; // BrowserRouter, Switch, Routable, etc.
use log::info;

use phazor_core::datasink::fake::FailThenOkSink; // using the fake sink for now
use phazor_core::outbox::{service::OutboxService, store_mem::MemStore};
mod components;
use components::router::{Route, switch};

// context type so pages can access the service 
#[derive(Clone)]
pub struct OutboxContext(pub Arc<OutboxService<MemStore, FailThenOkSink>>);

impl PartialEq for OutboxContext {
    fn eq(&self, other: &Self) -> bool {
        // true if both Arcs point to the same service instance
        Arc::ptr_eq(&self.0, &other.0)
    }
}

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

    // Provide the service to pages
    let ctx = OutboxContext(svc.borrow().clone());
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
    // Console logging
    wasm_logger::init(wasm_logger::Config::default());

    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once(); // nice errors during dev only

    let document = window().unwrap().document().unwrap();
    let root = document.get_element_by_id("root").expect("#root not found");

    Renderer::<App>::with_root(root).render();
}
