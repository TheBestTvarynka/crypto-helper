mod utils;

use web_sys::HtmlInputElement;
use yew::{classes, html, Html, function_component, use_state, Callback, TargetCast};

use crate::jwt::utils::Utils;

// eyJhbGciOiJIUzI1NiJ9.eyJSb2xlIjoiQWRtaW4iLCJJc3N1ZXIiOiJJc3N1ZXIiLCJVc2VybmFtZSI6IkphdmFJblVzZSIsImV4cCI6MTY3MDAwNDI1NCwiaWF0IjoxNjcwMDA0MjU0fQ.ZGsN42vr-bM4uxXowtlNl7xRerkdKu6i29VS8DFQ4Tw

#[function_component(Jwt)]
pub fn jwt() -> Html {
    let jwt = use_state(|| String::new());

    let jwt_setter = jwt.setter();
    let oninput = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        jwt_setter.set(input.value());
    });

    html! {
        <article class={classes!("vertical")}>
            <textarea
                rows="5"
                placeholder={"base64 encoded JWT(JWE)"}
                class={classes!("base-input")}
                value={(*jwt).clone()}
                {oninput}
            />
            <button>{"Process"}</button>
            <div class={classes!("jwt-page")}>
                <div>
                    <span class={classes!("jwt-header")}>{"header"}</span>
                    <span class={classes!("jwt-dot")}>{"."}</span>
                    <span class={classes!("jwt-payload")}>{"payload"}</span>
                    <span class={classes!("jwt-dot")}>{"."}</span>
                    <span class={classes!("jwt-signature")}>{"signature"}</span>
                </div>
                <div class={classes!("vertical")}>
                    <div class={classes!("vertical")}>
                        <span class={classes!("jwt-header")}>{"Header"}</span>
                        <textarea rows="4" class={classes!("base-input")} />
                    </div>
                    <div class={classes!("vertical")}>
                        <span class={classes!("jwt-payload")}>{"Payload"}</span>
                        <textarea rows="6" class={classes!("base-input")} />
                    </div>
                    <div class={classes!("vertical")}>
                        <span class={classes!("jwt-signature")}>{"Signature"}</span>
                        <textarea rows="2" class={classes!("base-input")} />
                    </div>
                </div>
            </div>
            <div class={classes!("container")}>
                <Utils />
            </div>
        </article>
    }
}
