use yew::prelude::*;
use web_sys::HtmlInputElement;
// use wasm_bindgen::JsCast;
use serde_json::json;
use crate::hooks::outbox_ops::{use_outbox_enqueue, EnqueueCreate};

#[function_component(Home)]
pub fn home() -> Html {
    let title = use_state(|| String::new());
    let oninput = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };

    let enqueue = use_outbox_enqueue();
    let onclick = {
        let title = title.clone();
        Callback::from(move |_| {
            enqueue.emit(EnqueueCreate {
                collection: "todos".into(),
                payload: json!({ "title": (*title).clone(), "done": false }),
            });
        })
    };

    html! {
        <>
          <h1>{ "Hello from home!" }</h1>
          <input value={(*title).clone()} {oninput} placeholder="todo title" />
          <button {onclick}>{ "Add todo" }</button>
        </>
    }
}

