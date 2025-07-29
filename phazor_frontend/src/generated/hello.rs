
use yew::prelude::*;

#[function_component(Hello)]
pub fn hello() -> Html {
    html! {
    <>
        <h1>{"Hello, {name}!"}</h1>
        <p>{"{age} years of wisdom!"}</p>
    </>
    }
}
