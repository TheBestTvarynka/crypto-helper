use web_sys::MouseEvent;
use yew::{classes, function_component, html, use_state, Callback, Html, Properties};
use yew_notifications::{use_notification, Notification, NotificationType};

use super::jwt::Jwt;
use super::signature::JwtSignatureAlgorithm;
use crate::check_symmetric_key;
use crate::common::{build_simple_input, build_simple_output, BytesFormat};

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
        JwtSignatureAlgorithm::None => {
            html! {
                <span>{"none signature algorithm doesn't need any key."}</span>
            }
        }
        JwtSignatureAlgorithm::Hs256(key) => build_simple_input(
            key.clone(),
            "HMAC SHA256 hex-encoded key".into(),
            Callback::from(move |key| {
                set_signature_algo.emit(JwtSignatureAlgorithm::Hs256(key));
            }),
        ),
        JwtSignatureAlgorithm::Hs512(key) => build_simple_input(
            key.clone(),
            "HMAC SHA512 hex-encoded key".into(),
            Callback::from(move |key| {
                set_signature_algo.emit(JwtSignatureAlgorithm::Hs512(key));
            }),
        ),
        JwtSignatureAlgorithm::Unsupported(algo_name) => {
            log::error!("Unsupported signature algo: {:?}", algo_name);

            html! {}
        }
    }
}

fn calculate_signature(jwt: &Jwt, spawn_notification: Callback<Notification>) -> Option<Vec<u8>> {
    let data_to_sign = base64::encode(format!("{}.{}", jwt.parsed_header, jwt.parsed_payload).as_bytes());

    match &jwt.signature_algorithm {
        JwtSignatureAlgorithm::None => Some(Vec::new()),
        JwtSignatureAlgorithm::Hs256(key) => {
            let key = check_symmetric_key!(
                key: key,
                len_hint: jwt.signature_algorithm.key_len_hint(),
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification
            );

            Some(hmac_sha256::HMAC::mac(data_to_sign.as_bytes(), &key).to_vec())
        }
        JwtSignatureAlgorithm::Hs512(key) => {
            let key = check_symmetric_key!(
                key: key,
                len_hint: jwt.signature_algorithm.key_len_hint(),
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification
            );

            Some(hmac_sha512::HMAC::mac(data_to_sign.as_bytes(), &key).to_vec())
        }
        JwtSignatureAlgorithm::Unsupported(algo_name) => {
            spawn_notification.emit(Notification::from_description_and_type(
                NotificationType::Warn,
                format!("Unsupported signature algorithm: {}.", algo_name,),
            ));

            None
        }
    }
}

fn validate_signature(jwt: &Jwt, spawn_notification: Callback<Notification>) -> Option<bool> {
    let data_to_sign = format!(
        "{}.{}",
        base64::encode(jwt.parsed_header.as_bytes()),
        base64::encode(jwt.parsed_payload.as_bytes())
    );

    let calculated_signature = match &jwt.signature_algorithm {
        JwtSignatureAlgorithm::None => Vec::new(),
        JwtSignatureAlgorithm::Hs256(key) => {
            let key = check_symmetric_key!(
                key: key,
                len_hint: jwt.signature_algorithm.key_len_hint(),
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification
            );

            hmac_sha256::HMAC::mac(data_to_sign.as_bytes(), &key).to_vec()
        }
        JwtSignatureAlgorithm::Hs512(key) => {
            let key = check_symmetric_key!(
                key: key,
                len_hint: jwt.signature_algorithm.key_len_hint(),
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification
            );

            hmac_sha512::HMAC::mac(data_to_sign.as_bytes(), &key).to_vec()
        }
        JwtSignatureAlgorithm::Unsupported(algo_name) => {
            spawn_notification.emit(Notification::from_description_and_type(
                NotificationType::Warn,
                format!("Unsupported signature algorithm: {}.", algo_name,),
            ));

            return None;
        }
    };

    Some(jwt.signature == calculated_signature)
}

fn generate_jwt(jwt: &Jwt, spawn_notification: Callback<Notification>) -> Option<Vec<u8>> {
    let signature = calculate_signature(jwt, spawn_notification)?;

    let header = base64::encode_config(jwt.parsed_header.as_bytes(), base64::URL_SAFE_NO_PAD);
    let payload = base64::encode_config(jwt.parsed_payload.as_bytes(), base64::URL_SAFE_NO_PAD);
    let signature = base64::encode_config(signature, base64::URL_SAFE_NO_PAD);

    let jwt = format!("{}.{}.{}", header, payload, signature);

    Some(jwt.as_bytes().to_vec())
}

#[function_component(JwtUtils)]
pub fn jwt_utils(props: &JwtUtilsProps) -> Html {
    let data = use_state(|| None);

    let data_setter = data.setter();
    let jwt = props.jwt.clone();
    let notifications = use_notification::<Notification>();
    let recalculate = Callback::from(move |_event: MouseEvent| {
        let notifications = notifications.clone();
        let signature = calculate_signature(
            &jwt,
            Callback::from(move |notification| notifications.spawn(notification)),
        );
        data_setter.set(signature);
    });

    let data_setter = data.setter();
    let jwt = props.jwt.clone();
    let notifications = use_notification::<Notification>();
    let validate = Callback::from(move |_event: MouseEvent| {
        let notifications = notifications.clone();
        data_setter.set(
            validate_signature(
                &jwt,
                Callback::from(move |notification| notifications.spawn(notification)),
            )
            .map(|v| vec![v as u8]),
        );
    });

    let data_setter = data.setter();
    let jwt = props.jwt.clone();
    let notifications = use_notification::<Notification>();
    let generate = Callback::from(move |_event: MouseEvent| {
        let notifications = notifications.clone();
        data_setter.set(generate_jwt(
            &jwt,
            Callback::from(move |notification| notifications.spawn(notification)),
        ));
    });

    let jwt = props.jwt.clone();
    let set_jwt = props.set_jwt.clone();

    let notifications = use_notification::<Notification>();

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
                <button class={classes!("jwt-util-button")} onclick={generate}>{"Regenerate JWT"}</button>
            </div>
            {if let Some(data) = (*data).as_ref() {
                build_simple_output((*data).clone(),  BytesFormat::Hex, Callback::from(move |notification| notifications.spawn(notification)))
            } else {
                html! {}
            }}
        </div>
    }
}
