mod common;
mod crypto_helper;
mod footer;
mod header;
mod notification;
mod utils;

use yew::{classes, function_component, html, Html};

use crypto_helper::CryptoHelper;
use footer::footer;
use header::header;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class={classes!("body")}>
            {header()}
            <CryptoHelper />
            {footer()}
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
