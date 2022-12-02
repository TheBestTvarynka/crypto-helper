use yew::{classes, function_component, html, Html, Properties};

use crate::utils::gen_onclick;

use super::Jwt;

#[derive(PartialEq, Eq, Properties)]
pub struct JwtEditorProps {
    pub jwt: Jwt,
}

#[function_component(JwtEditor)]
pub fn jwt_editor(props: &JwtEditorProps) -> Html {
    let header = props.jwt.parsed_header.clone();
    let payload = props.jwt.parsed_payload.clone();
    let signature = props.jwt.parsed_signature.clone();

    html! {
        <div class={classes!("vertical")}>
            <div class={classes!("vertical")}>
                <span class={classes!("jwt-header")} onclick={gen_onclick(header.clone())}>{"Header"}</span>
                <textarea rows="4" class={classes!("base-input")} value={header} />
            </div>
            <div class={classes!("vertical")}>
                <span class={classes!("jwt-payload")} onclick={gen_onclick(payload.clone())}>{"Payload"}</span>
                <textarea rows="6" class={classes!("base-input")} value={payload} />
            </div>
            <div class={classes!("vertical")}>
                <span class={classes!("jwt-signature")} onclick={gen_onclick(signature.clone())}>{"Signature"}</span>
                <textarea rows="2" class={classes!("base-input")} value={signature} />
            </div>
        </div>
    }
}
