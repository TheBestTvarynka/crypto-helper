use yew::{html, Html};

pub fn header() -> Html {
    html! {
        <header>
            <a href="/">{"Crypto helper"}</a>
            <a href="/jwt">{"JWT/JWE"}</a>
            <a href="/about">{"About"}</a>
        </header>
    }
}
