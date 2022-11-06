mod common;
mod crypto_helper;
mod footer;
mod header;
mod notification;

use crypto_helper::CryptoHelper;
use footer::Footer;
use header::Header;

use yew::{classes, html, Component, Context, Html};

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("body")}>
                <Header />
                <CryptoHelper />
                <Footer />
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
