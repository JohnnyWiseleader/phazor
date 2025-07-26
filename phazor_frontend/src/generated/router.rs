
use yew::prelude::*;
use yew_router::prelude::*;
use crate::generated;

#[derive(Routable, Clone, PartialEq, Eq, Debug)]
pub enum Route {
    #[at("/about")]
    About,
    #[at("/")]
    #[at("/home")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::About => html! { <generated::about::About /> },
        Route::Home => html! { <generated::home::Home /> },
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
    }
}
