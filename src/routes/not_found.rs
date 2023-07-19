use yew::{html, Html};

pub fn not_found() -> Html {
    html! {
        <span>{"Error: not found"}</span>
    }
}
