use web_sys::MouseEvent;
use yew::{classes, function_component, html, use_state, Callback, Html, Properties};

use crate::common::{build_simple_input, build_simple_output};

use super::jwt::{Jwt, JwtSignatureAlgorithm};

#[derive(PartialEq, Properties)]
pub struct JwtUtilsProps {
    pub jwt: Jwt,
    pub set_jwt: Callback<Jwt>,
}

fn get_input_component(
    signature_algo: &JwtSignatureAlgorithm,
    set_signature_algo: Callback<JwtSignatureAlgorithm>,
) -> Html {
    match signature_algo {
        JwtSignatureAlgorithm::Hs256(key) => build_simple_input(
            key.clone(),
            "HMAC SHA256 hex-encoded key".into(),
            Callback::from(move |key| {
                set_signature_algo.emit(JwtSignatureAlgorithm::Hs256(key));
            }),
        ),
        _ => html! {},
    }
}

fn calculate_signature(jwt: &Jwt) -> Vec<u8> {
    let data_to_sign =
        base64::encode(format!("{}.{}", jwt.parsed_header, jwt.parsed_payload).as_bytes());

    match &jwt.signature_algorithm {
        JwtSignatureAlgorithm::Hs256(key) => {
            let key = if let Ok(key) = hex::decode(key) {
                key
            } else {
                log::error!("invalid HMAC SHA256 key: {}", key);
                return Default::default();
            };

            hmac_sha256::HMAC::mac(data_to_sign.as_bytes(), &key).to_vec()
        }
        _ => Default::default(),
    }
}

fn validate_signature(jwt: &Jwt) -> bool {
    let data_to_sign =
        base64::encode(format!("{}.{}", jwt.parsed_header, jwt.parsed_payload).as_bytes());

    let calculated_signature = match &jwt.signature_algorithm {
        JwtSignatureAlgorithm::Hs256(key) => {
            let key = if let Ok(key) = hex::decode(key) {
                key
            } else {
                log::error!("invalid HMAC SHA256 key: {}", key);
                return false;
            };

            hmac_sha256::HMAC::mac(data_to_sign.as_bytes(), &key).to_vec()
        }
        _ => return false,
    };

    jwt.signature == calculated_signature
}

#[function_component(JwtUtils)]
pub fn jwt_utils(props: &JwtUtilsProps) -> Html {
    let data = use_state(Vec::<u8>::new);

    let data_setter = data.setter();
    let jwt = props.jwt.clone();
    let recalculate = Callback::from(move |_event: MouseEvent| {
        let signature = calculate_signature(&jwt);
        data_setter.set(signature);
    });

    let data_setter = data.setter();
    let jwt = props.jwt.clone();
    let validate = Callback::from(move |_event: MouseEvent| {
        data_setter.set(vec![validate_signature(&jwt) as u8]);
    });

    let jwt = props.jwt.clone();
    let set_jwt = props.set_jwt.clone();

    html! {
        <div class={classes!("vertical")}>
            {get_input_component(&props.jwt.signature_algorithm, Callback::from(move |signature_algo| {
                let mut new_jwt = jwt.clone();
                new_jwt.signature_algorithm = signature_algo;

                set_jwt.emit(new_jwt);
            }))}
            <div class={classes!("horizontal")}>
                <button class={classes!("jwt-util-button")} onclick={validate}>{"Validate signature"}</button>
                <button class={classes!("jwt-util-button")} onclick={recalculate}>{"Recalculate signature"}</button>
                <button class={classes!("jwt-util-button")}>{"Regenerate JWT"}</button>
            </div>
            {if !(*data).is_empty() {
                build_simple_output((*data).as_ref(), Callback::from(|_| {}))
            } else { html! {} }}
        </div>
    }
}
