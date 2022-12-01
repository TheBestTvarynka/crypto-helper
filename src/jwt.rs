use web_sys::HtmlInputElement;
use yew::{classes, html, Html, function_component, use_state, Callback, TargetCast};

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
        </article>
    }
}
