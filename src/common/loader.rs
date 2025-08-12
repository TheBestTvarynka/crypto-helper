use yew::{Html, function_component, html};

#[function_component(Loader)]
pub fn loader() -> Html {
    html! {
        <div class="loader-wrapper">
            <div class="loader" />
        </div>
    }
}
