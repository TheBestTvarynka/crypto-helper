use yew::{function_component, html, Html};
use yew_router::prelude::Link;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <Link<Route> to={Route::CryptoHelper}>{"Crypto helper"}</Link<Route>>
            <Link<Route> to={Route::Jwt}>{"JWT debugger"}</Link<Route>>
            <Link<Route> to={Route::Asn1Parser}>{"Asn1 debugger (beta)"}</Link<Route>>
            <Link<Route> to={Route::Diff}>{"Diff (beta)"}</Link<Route>>
            <Link<Route> to={Route::About}>{"About"}</Link<Route>>
        </header>
    }
}
