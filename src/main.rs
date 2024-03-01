#[macro_use]
extern crate log;

mod about;
mod asn1;
mod common;
mod crypto_helper;
mod footer;
mod header;
mod jwt;
mod not_found;
mod url_query_params;
mod utils;

use about::About;
use asn1::Asn1ParserPage;
use crypto_helper::CryptoHelper;
use footer::footer;
use header::Header;
use jwt::Jwt;
use not_found::not_found;
use yew::{function_component, html, Html};
use yew_notifications::{Notification, NotificationFactory, NotificationsProvider};
use yew_router::{BrowserRouter, Routable, Switch};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/asn1")]
    Asn1Parser,
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
        Route::Asn1Parser => html! { <Asn1ParserPage /> },
        Route::CryptoHelper => html! { <CryptoHelper /> },
        Route::Jwt => html! { <Jwt /> },
        Route::About => html! { <About /> },
        Route::NotFound => not_found(),
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let component_creator = NotificationFactory;

    html! {
        <BrowserRouter>
            <NotificationsProvider<Notification, NotificationFactory> {component_creator}>
                <div class="body">
                    <Header />
                    <Switch<Route> render={switch} />
                    {footer()}
                </div>
            </NotificationsProvider<Notification, NotificationFactory>>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
