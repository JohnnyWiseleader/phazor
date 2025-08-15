use crate::hooks::outbox_ops::use_outbox_ops;
use web_sys::MouseEvent;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let cb = use_outbox_ops(); // Callback<()>
    let onclick: Callback<MouseEvent> = cb.reform(|_| ()); // ignore the event

    html! {
        <>
            <h1>{ "Hello from home!" }</h1>
            <p>{ "Welcome to Phazor" }</p>
            <button {onclick}>{ "Add sample todo" }</button>
        </>
    }
}
