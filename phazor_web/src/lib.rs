use std::sync::Arc;
use wasm_bindgen::prelude::*;
use web_sys::window;
use yew::prelude::*;
use yew::Renderer;
use yew::context::ContextProvider;
use yew_router::prelude::*;
use log::info;

mod components;
mod hooks;

use phazor_core::datasink::fake::FailThenOkSink; // using the fake sink for now
use phazor_core::outbox::{Outbox, store_mem::MemStore}; // - Outbox via re-export
use components::router::{Route, switch};

#[derive(Clone)]
pub struct OutboxContext(pub Arc<Outbox<MemStore, FailThenOkSink>>);

impl PartialEq for OutboxContext {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

#[function_component(App)]
fn app() -> Html {
    // Singletons
    let svc = use_mut_ref(|| {
        let store = Arc::new(MemStore::default());
        let sink  = Arc::new(FailThenOkSink::new(2));
        Arc::new(Outbox::new(store, sink)) // - façade, not raw service
    });

    // Background loop (with cleanup)
    use_effect_with((), {
        let svc = svc.clone();
        move |_| {
            let stop = std::rc::Rc::new(std::cell::Cell::new(false));
            let stop2 = stop.clone();
            wasm_bindgen_futures::spawn_local(async move {
                while !stop2.get() {
                    let _ = svc.borrow().drain_once().await; // façade forwards to service
                    gloo_timers::future::TimeoutFuture::new(2000).await;
                }
            });
            move || stop.set(true)
        }
    });

    // Kick a drain when the browser comes online
    use_effect_with((), {
        let svc = svc.clone();
        move |_| {
            let win = web_sys::window().expect("window");
            let listener = gloo_events::EventListener::new(&win, "online", move |_| {
                let svc = svc.clone();
                wasm_bindgen_futures::spawn_local(async move { let _ = svc.borrow().drain_once().await; });
            });
            move || drop(listener)
        }
    });

    // Provide the outbox service to pages
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
