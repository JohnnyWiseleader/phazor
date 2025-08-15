use log::debug;
use serde_json::Value;
use serde_json::to_string_pretty;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::OutboxContext; // this is the type defined in lib.rs from the façade

#[derive(Clone, PartialEq)]
pub struct EnqueueCreate {
    pub collection: String,
    pub payload: Value,
}

/// Returns a callback you can stick on a button `onclick` that enqueues
/// a Generic “enqueue” hook (collection + JSON)
#[hook]
pub fn use_outbox_enqueue() -> Callback<EnqueueCreate> {
    let ctx = use_context::<OutboxContext>().expect("OutboxContext not provided");
    Callback::from(move |req: EnqueueCreate| {
        let api = ctx.0.clone();
        spawn_local(async move {
            debug!( // log collection and payload to the browser console 
                "UI -> enqueue: collection={}, payload=\n{}",
                req.collection,
                to_string_pretty(&req.payload).unwrap()
            );
            let _ = api.enqueue_create(&req.collection, req.payload).await;
        });
    })
}
