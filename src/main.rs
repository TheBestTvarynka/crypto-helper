mod about;
mod common;
mod crypto_helper;
mod footer;
mod header;
mod jwt;
mod not_found;
mod notification;
mod utils;

use about::About;
use crypto_helper::CryptoHelper;
use footer::footer;
use header::Header;
use jwt::jwt;
use not_found::not_found;
use yew::{classes, function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};

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

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <CryptoHelper /> },
        Route::CryptoHelper => html! { <CryptoHelper /> },
        Route::Jwt => jwt(),
        Route::About => html! { <About /> },
        Route::NotFound => not_found(),
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class={classes!("body")}>
                <Header />
                <Switch<Route> render={switch} />
                {footer()}
            </div>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
