use yew::{Html, html};

pub fn not_found() -> Html {
    html! {
        <span>{"Error: not found"}</span>
    }
}
