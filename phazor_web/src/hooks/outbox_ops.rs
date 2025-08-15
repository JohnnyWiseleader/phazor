use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde_json::json;
use crate::OutboxContext; // - this is the type defined in lib.rs from the façade

/// Returns a callback you can stick on a button `onclick` that enqueues a todo
/// and kicks a one-shot drain.
#[hook]
pub fn use_outbox_ops() -> Callback<()> {
    let ctx = use_context::<OutboxContext>().expect("OutboxContext not provided");
    let svc = ctx.0.clone(); // Arc<Outbox<...>>

    Callback::from(move |_| {
        let svc = svc.clone();
        spawn_local(async move {
            // Use the façade’s helpers:
            let _ = svc.enqueue_create("todos", json!({ "title": "Buy hay bales", "done": false })).await;
            let _ = svc.drain_once().await; // optional: nudge delivery immediately
        });
    })
}
