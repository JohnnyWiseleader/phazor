use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

use phazor_core::outbox::types::{Message, MessageKind};
use crate::OutboxContext; // import the context type from lib.rs

#[function_component(Home)]
pub fn home() -> Html {
    let ctx = use_context::<OutboxContext>().expect("OutboxContext not found");
    let status = use_state(|| String::from("idle"));

    let on_enqueue = {
        let ctx = ctx.0.clone();
        let status = status.clone();
        Callback::from(move |_| {
            let ctx = ctx.clone();
            let status = status.clone();
            spawn_local(async move {
                let msg = Message::new(
                    MessageKind::Create { collection: "todos".into() },
                    serde_json::json!({ "title": "Buy hay bales", "done": false }),
                );
                match ctx.enqueue(msg).await {
                    Ok(_) => status.set("enqueued".into()),
                    Err(e) => status.set(format!("enqueue error: {e}")),
                }
            });
        })
    };

    html! {
        <>
            <h1>{ "Hello from home!" }</h1>
            <p>{ "Welcome to Phazor" }</p>

            <button onclick={on_enqueue}>{ "Add sample todo" }</button>
            <span style="margin-left:8px">{ (*status).clone() }</span>
        </>
    }
}
