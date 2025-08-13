
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components;

#[derive(Routable, Clone, PartialEq, Eq, Debug)]
pub enum Route {
    #[at("/hello/:name/:age")]
    Hello { name: String, age: String },
    #[at("/about")]
    About,
    #[at("/info")]
    Info,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Hello { name, age } => html! { <components::hello::Hello name={name} age={age} /> },
        Route::About => html! { <components::about::About /> },
        Route::Info => html! { <components::info::Info /> },
        Route::Home => html! { <components::home::Home /> },
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
    }
}
