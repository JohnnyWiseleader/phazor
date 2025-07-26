use yew::prelude::*;
use yew_router::prelude::*;

mod generated; // <- this will point to generated components

#[derive(Routable, Clone, PartialEq, Eq, Debug)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <generated::home::Home /> },
        Route::About => html! { <generated::about::About /> },
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
