use yew::{classes, function_component, html, Callback, Html, Properties};

use crate::common::build_simple_input;

use super::jwt::{Jwt, JwtSignatureAlgorithm};

#[derive(PartialEq, Eq, Properties)]
pub struct JwtUtilsProps {
    pub jwt: Jwt,
}

fn get_input_component(sig_algo: &JwtSignatureAlgorithm) -> Html {
    match sig_algo {
        JwtSignatureAlgorithm::Hs256(key) => build_simple_input(
            key.clone(),
            "HMAC SHA256 hex-encoded key".into(),
            Callback::from(|_| {}),
        ),
        _ => html! {},
    }
}

#[function_component(JwtUtils)]
pub fn jwt_utils(props: &JwtUtilsProps) -> Html {
    html! {
        <div class={classes!("vertical")}>
            {get_input_component(&props.jwt.signature_algorithm)}
            <div class={classes!("horizontal")}>
                <button class={classes!("jwt-util-button")}>{"Validate signature"}</button>
                <button class={classes!("jwt-util-button")}>{"Recalculate signature"}</button>
                <button class={classes!("jwt-util-button")}>{"Regenerate JWT"}</button>
            </div>
        </div>
    }
}
