use picky::hash::HashAlgorithm;
use picky::key::{PrivateKey, PublicKey};
use picky::signature::SignatureAlgorithm;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::{classes, function_component, html, use_state, Callback, Html, Properties, TargetCast};
use yew_notifications::{use_notification, Notification, NotificationType};

use super::jwt::Jwt;
use super::signature::JwtSignatureAlgorithm;
use crate::{check_asymmetric_key, check_symmetric_key, sign, verify};
use crate::common::{build_simple_input, build_simple_output, BytesFormat};

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
        JwtSignatureAlgorithm::Hs384(key) => build_simple_input(
            key.clone(),
            "HMAC SHA384 hex-encoded key".into(),
            Callback::from(move |key| {
                set_signature_algo.emit(JwtSignatureAlgorithm::Hs384(key));
            }),
        ),
        JwtSignatureAlgorithm::Hs512(key) => build_simple_input(
            key.clone(),
            "HMAC SHA512 hex-encoded key".into(),
            Callback::from(move |key| {
                set_signature_algo.emit(JwtSignatureAlgorithm::Hs512(key));
            }),
        ),
        JwtSignatureAlgorithm::Rs256(key) => {
            let oninput = Callback::from(move |event: html::oninput::Event| {
                let input: HtmlInputElement = event.target_unchecked_into();
                set_signature_algo.emit(JwtSignatureAlgorithm::Rs256(input.value()));
            });

            html! {
                <textarea
                    rows="4"
                    placeholder={"RSA private/public key in PEM (-----BEGIN RSA PRIVATE/PUBLIC KEY-----)"}
                    class={classes!("base-input")}
                    value={key.clone()}
                    {oninput}
                />
            }
        }
        JwtSignatureAlgorithm::Rs384(key) => {
            let oninput = Callback::from(move |event: html::oninput::Event| {
                let input: HtmlInputElement = event.target_unchecked_into();
                set_signature_algo.emit(JwtSignatureAlgorithm::Rs384(input.value()));
            });

            html! {
                <textarea
                    rows="4"
                    placeholder={"RSA private/public key in PEM (-----BEGIN RSA PRIVATE/PUBLIC KEY-----)"}
                    class={classes!("base-input")}
                    value={key.clone()}
                    {oninput}
                />
            }
        }
        JwtSignatureAlgorithm::Rs512(key) => {
            let oninput = Callback::from(move |event: html::oninput::Event| {
                let input: HtmlInputElement = event.target_unchecked_into();
                set_signature_algo.emit(JwtSignatureAlgorithm::Rs512(input.value()));
            });

            html! {
                <textarea
                    rows="4"
                    placeholder={"RSA private/public key in PEM (-----BEGIN RSA PRIVATE/PUBLIC KEY-----)"}
                    class={classes!("base-input")}
                    value={key.clone()}
                    {oninput}
                />
            }
        }
        JwtSignatureAlgorithm::Unsupported(algo_name) => {
            log::error!("Unsupported signature algo: {:?}", algo_name);

            if !algo_name.is_empty() {
                html! {
                    <span>{format!("Unsupported signature algo: {}", algo_name)}</span>
                }
            } else {
                html! {}
            }
        }
    }
}

fn calculate_signature(jwt: &Jwt, spawn_notification: Callback<Notification>) -> Option<Vec<u8>> {
    let data_to_sign = format!(
        "{}.{}",
        base64::encode(jwt.parsed_header.as_bytes()),
        base64::encode(jwt.parsed_payload.as_bytes())
    );

    match &jwt.signature_algorithm {
        JwtSignatureAlgorithm::None => Some(Vec::new()),
        JwtSignatureAlgorithm::Hs256(key) => {
            let key = check_symmetric_key!(
                key: key,
                len_hint: jwt.signature_algorithm.key_len_hint(),
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification
            );

            Some(hmac_sha256::HMAC::mac(data_to_sign.as_bytes(), key).to_vec())
        }
        JwtSignatureAlgorithm::Hs384(key) => {
            let key = check_symmetric_key!(
                key: key,
                len_hint: jwt.signature_algorithm.key_len_hint(),
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification
            );

            Some(hmac_sha512::sha384::HMAC::mac(data_to_sign.as_bytes(), key).to_vec())
        }
        JwtSignatureAlgorithm::Hs512(key) => {
            let key = check_symmetric_key!(
                key: key,
                len_hint: jwt.signature_algorithm.key_len_hint(),
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification
            );

            Some(hmac_sha512::HMAC::mac(data_to_sign.as_bytes(), key).to_vec())
        }
        JwtSignatureAlgorithm::Rs256(key) => {
            let private_key = match PrivateKey::from_pem_str(key) {
                Ok(key) => key,
                Err(error) => {
                    spawn_notification.emit(Notification::new(
                        NotificationType::Error,
                        "Invalid private RSA key",
                        error.to_string(),
                    ));

                    return None;
                }
            };

            match SignatureAlgorithm::RsaPkcs1v15(HashAlgorithm::SHA2_256).sign(data_to_sign.as_bytes(), &private_key) {
                Ok(signature) => Some(signature),
                Err(error) => {
                    spawn_notification.emit(Notification::new(
                        NotificationType::Error,
                        "Can not generate RS256 signature",
                        error.to_string(),
                    ));

                    None
                }
            }
        }
        JwtSignatureAlgorithm::Rs384(key) => {
            let private_key = match PrivateKey::from_pem_str(key) {
                Ok(key) => key,
                Err(error) => {
                    spawn_notification.emit(Notification::new(
                        NotificationType::Error,
                        "Invalid private RSA key",
                        error.to_string(),
                    ));

                    return None;
                }
            };

            match SignatureAlgorithm::RsaPkcs1v15(HashAlgorithm::SHA2_384).sign(data_to_sign.as_bytes(), &private_key) {
                Ok(signature) => Some(signature),
                Err(error) => {
                    spawn_notification.emit(Notification::new(
                        NotificationType::Error,
                        "Can not generate RS384 signature",
                        error.to_string(),
                    ));

                    None
                }
            }
        }
        JwtSignatureAlgorithm::Rs512(key) => {
            let private_key = check_asymmetric_key!(
                key: key,
                name: jwt.signature_algorithm.to_string(),
                notificator: &spawn_notification,
                key_kind: PrivateKey,
                key_kind_as_str: "private"
            );

            sign!(
                signature_algo: SignatureAlgorithm::RsaPkcs1v15,
                hash_algo: HashAlgorithm::SHA2_512,
                name: jwt.signature_algorithm.to_string(),
                private_key: &private_key,
                data_to_sign: data_to_sign.as_bytes(),
                notificator: &spawn_notification
            )

            // match SignatureAlgorithm::RsaPkcs1v15(HashAlgorithm::SHA2_512).sign(data_to_sign.as_bytes(), &private_key) {
            //     Ok(signature) => Some(signature),
            //     Err(error) => {
            //         spawn_notification.emit(Notification::new(
            //             NotificationType::Error,
            //             "Can not generate RS512 signature",
            //             error.to_string(),
            //         ));
            //
            //         None
            //     }
            // }
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

            hmac_sha256::HMAC::mac(data_to_sign.as_bytes(), key).to_vec()
        }
        JwtSignatureAlgorithm::Hs384(key) => {
            let key = check_symmetric_key!(
                key: key,
                len_hint: jwt.signature_algorithm.key_len_hint(),
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification
            );

            hmac_sha512::sha384::HMAC::mac(data_to_sign.as_bytes(), key).to_vec()
        }
        JwtSignatureAlgorithm::Hs512(key) => {
            let key = check_symmetric_key!(
                key: key,
                len_hint: jwt.signature_algorithm.key_len_hint(),
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification
            );

            hmac_sha512::HMAC::mac(data_to_sign.as_bytes(), key).to_vec()
        }
        JwtSignatureAlgorithm::Rs256(key) => {
            let public_key = match PublicKey::from_pem_str(key) {
                Ok(key) => key,
                Err(error) => {
                    spawn_notification.emit(Notification::new(
                        NotificationType::Error,
                        "Invalid public RSA key",
                        error.to_string(),
                    ));

                    return None;
                }
            };

            log::debug!("data_to_sign: {:?}", data_to_sign.as_bytes());
            log::debug!("signature: {:?}", jwt.signature);

            match SignatureAlgorithm::RsaPkcs1v15(HashAlgorithm::SHA2_256).verify(
                &public_key,
                data_to_sign.as_bytes(),
                &jwt.signature,
            ) {
                Ok(_) => return Some(true),
                Err(error) => {
                    spawn_notification.emit(Notification::from_description_and_type(
                        NotificationType::Error,
                        error.to_string(),
                    ));

                    return Some(false);
                }
            }
        }
        JwtSignatureAlgorithm::Rs384(key) => {
            let public_key = match PublicKey::from_pem_str(key) {
                Ok(key) => key,
                Err(error) => {
                    spawn_notification.emit(Notification::new(
                        NotificationType::Error,
                        "Invalid public RSA key",
                        error.to_string(),
                    ));

                    return None;
                }
            };

            log::debug!("data_to_sign: {:?}", data_to_sign.as_bytes());
            log::debug!("signature: {:?}", jwt.signature);

            match SignatureAlgorithm::RsaPkcs1v15(HashAlgorithm::SHA2_384).verify(
                &public_key,
                data_to_sign.as_bytes(),
                &jwt.signature,
            ) {
                Ok(_) => return Some(true),
                Err(error) => {
                    spawn_notification.emit(Notification::from_description_and_type(
                        NotificationType::Error,
                        error.to_string(),
                    ));

                    return Some(false);
                }
            }
        }
        JwtSignatureAlgorithm::Rs512(key) => {
            let public_key = check_asymmetric_key!(
                key: key,
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification,
                key_kind: PublicKey,
                key_kind_as_str: "public"
            );

            log::debug!("data_to_sign: {:?}", data_to_sign.as_bytes());
            log::debug!("signature: {:?}", jwt.signature);

            let is_ok = verify!(
                signature_algo: SignatureAlgorithm::RsaPkcs1v15,
                hash_algo: HashAlgorithm::SHA2_512,
                public_key: &public_key,
                data_to_sign: data_to_sign.as_bytes(),
                jwt_signature: &jwt.signature,
                notificator: spawn_notification
            );

            return Some(is_ok)
            // match SignatureAlgorithm::RsaPkcs1v15(HashAlgorithm::SHA2_512).verify(
            //     &public_key,
            //     data_to_sign.as_bytes(),
            //     &jwt.signature,
            // ) {
            //     Ok(_) => return Some(true),
            //     Err(error) => {
            //         spawn_notification.emit(Notification::from_description_and_type(
            //             NotificationType::Error,
            //             error.to_string(),
            //         ));
            //
            //         return Some(false);
            //     }
            // }
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

#[derive(PartialEq, Properties)]
pub struct JwtUtilsProps {
    pub jwt: Jwt,
    pub set_jwt: Callback<Jwt>,
}

#[function_component(JwtUtils)]
pub fn jwt_utils(props: &JwtUtilsProps) -> Html {
    let data = use_state(|| None);
    let bytes_format = use_state(|| BytesFormat::Hex);

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
    let bytes_format_setter = bytes_format.setter();
    let jwt = props.jwt.clone();
    let notifications = use_notification::<Notification>();
    let generate = Callback::from(move |_event: MouseEvent| {
        let notifications = notifications.clone();
        data_setter.set(generate_jwt(
            &jwt,
            Callback::from(move |notification| notifications.spawn(notification)),
        ));
        bytes_format_setter.set(BytesFormat::Ascii);
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
            {if props.jwt.signature_algorithm.is_supported() {
                html! {
                    <div class={classes!("horizontal")}>
                        <button class={classes!("jwt-util-button")} onclick={validate}>{"Validate signature"}</button>
                        <button class={classes!("jwt-util-button")} onclick={recalculate}>{"Recalculate signature"}</button>
                        <button class={classes!("jwt-util-button")} onclick={generate}>{"Regenerate JWT"}</button>
                    </div>
                }
            } else {
                html! {}
            }}
            {if let Some(data) = (*data).as_ref() {
                build_simple_output((*data).clone(),  *(bytes_format), Callback::from(move |notification| notifications.spawn(notification)))
            } else {
                html! {}
            }}
        </div>
    }
}
