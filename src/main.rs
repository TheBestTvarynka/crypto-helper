mod about;
mod common;
mod crypto_helper;
mod footer;
mod header;
mod notification;
mod utils;

use yew::{classes, function_component, html, Html};
use yew_router::{Routable, BrowserRouter, Switch};

use about::About;
use crypto_helper::CryptoHelper;
use footer::footer;
use header::header;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/crypto-helper")]
    CryptoHelper,
    #[at("/jwt")]
    Jwt,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn _switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <CryptoHelper /> },
        Route::CryptoHelper => html! { <CryptoHelper /> },
        Route::Jwt => html!{ <span>{"Jw(t/e)"}</span> },
        Route::About => html!{ <span>{"About"}</span> },
        Route::NotFound => html!{ <span>{"Error: not found"}</span> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class={classes!("body")}>
            {header()}
            // <BrowserRouter>
            //     <Switch<Route> render={switch} />
            // </BrowserRouter>
            // <CryptoHelper />
            <About />
            {footer()}
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
