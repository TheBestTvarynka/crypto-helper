use yew::{function_component, Html, html, classes};

#[function_component(Utils)]
pub fn utils() -> Html {
    html! {
        <div class={classes!("vertical")}>
            <textarea rows="2" class={classes!("base-input")} />
            <div class={classes!("horizontal")}>
                <button>{"Validate signature"}</button>
                <button>{"Generate signature"}</button>
                <button>{"Generate new JWT"}</button>
            </div>
        </div>
    }
}