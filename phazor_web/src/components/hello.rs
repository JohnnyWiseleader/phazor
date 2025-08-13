use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HelloProps {
    pub name: String,
    pub age: String,
}

#[function_component(Hello)]
pub fn hello(props: &HelloProps) -> Html {
    html! {
    <>
        <h1>{ format!("Hello, {}!", props.name) }</h1>
        <p>{ format!("{} years of wisdom!", props.age) }</p>
    </>
    }
}
