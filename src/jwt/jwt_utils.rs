use base64::Engine;
use base64::engine::GeneralPurpose;
use base64::engine::general_purpose::STANDARD;
use picky::hash::HashAlgorithm;
use picky::key::{PrivateKey, PublicKey};
use picky::signature::SignatureAlgorithm;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::{Callback, Html, Properties, TargetCast, function_component, html, use_state};
use yew_hooks::use_clipboard;
use yew_notifications::{Notification, NotificationType, use_notification};

use super::jwt::Jwt;
use super::signature::JwtSignatureAlgorithm;
use crate::common::{BytesFormat, build_byte_input, build_simple_output};
use crate::url_query_params::generate_jwt_link;

const DEFAULT_TEXT_FOR_RSA_PLACEHOLDER: &str = "RSA private/public key in PEM (-----BEGIN RSA PRIVATE/PUBLIC KEY-----)";
const DEFAULT_TEXT_FOR_EC_PLACEHOLDER: &str = "EC private/public key in PEM (-----BEGIN EC PRIVATE/PUBLIC KEY-----)";

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
        JwtSignatureAlgorithm::Hs256(key) => build_byte_input(
            key.clone(),
            Callback::from(move |key| {
                set_signature_algo.emit(JwtSignatureAlgorithm::Hs256(key));
            }),
            None,
            Some("HMAC SHA256 key".into()),
        ),
        JwtSignatureAlgorithm::Hs384(key) => build_byte_input(
            key.clone(),
            Callback::from(move |key| {
                set_signature_algo.emit(JwtSignatureAlgorithm::Hs384(key));
            }),
            None,
            Some("HMAC SHA384 key".into()),
        ),
        JwtSignatureAlgorithm::Hs512(key) => build_byte_input(
            key.clone(),
            Callback::from(move |key| {
                set_signature_algo.emit(JwtSignatureAlgorithm::Hs512(key));
            }),
            None,
            Some("HMAC SHA512 key".into()),
        ),
        JwtSignatureAlgorithm::Rs256(key) => {
            generate_placeholder!(
                signature: JwtSignatureAlgorithm::Rs256,
                default_text: DEFAULT_TEXT_FOR_RSA_PLACEHOLDER,
                set_signature_algo: set_signature_algo,
                key: key
            )
        }
        JwtSignatureAlgorithm::Rs384(key) => {
            generate_placeholder!(
                signature: JwtSignatureAlgorithm::Rs384,
                default_text: DEFAULT_TEXT_FOR_RSA_PLACEHOLDER,
                set_signature_algo: set_signature_algo,
                key: key
            )
        }
        JwtSignatureAlgorithm::Rs512(key) => {
            generate_placeholder!(
                signature: JwtSignatureAlgorithm::Rs512,
                default_text: DEFAULT_TEXT_FOR_RSA_PLACEHOLDER,
                set_signature_algo: set_signature_algo,
                key: key
            )
        }
        JwtSignatureAlgorithm::Es256(key) => {
            generate_placeholder!(
                signature: JwtSignatureAlgorithm::Es256,
                default_text: DEFAULT_TEXT_FOR_EC_PLACEHOLDER,
                set_signature_algo: set_signature_algo,
                key: key
            )
        }
        JwtSignatureAlgorithm::Es384(key) => {
            generate_placeholder!(
                signature: JwtSignatureAlgorithm::Es384,
                default_text: DEFAULT_TEXT_FOR_EC_PLACEHOLDER,
                set_signature_algo: set_signature_algo,
                key: key
            )
        }
        JwtSignatureAlgorithm::Es512(key) => {
            generate_placeholder!(
                signature: JwtSignatureAlgorithm::Es512,
                default_text: DEFAULT_TEXT_FOR_EC_PLACEHOLDER,
                set_signature_algo: set_signature_algo,
                key: key
            )
        }
        JwtSignatureAlgorithm::Unsupported(algo_name) => {
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
        STANDARD.encode(jwt.parsed_header.as_bytes()),
        STANDARD.encode(jwt.parsed_payload.as_bytes())
    );

    match &jwt.signature_algorithm {
        JwtSignatureAlgorithm::None => Some(Vec::new()),
        JwtSignatureAlgorithm::Hs256(key) => {
            check_symmetric_key!(
                key: key,
                len_hint: jwt.signature_algorithm.key_len_hint(),
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification
            );

            Some(sign_hmac!(
                hash_alg: sha2::Sha256,
                key: key,
                msg: data_to_sign.as_bytes(),
            ))
        }
        JwtSignatureAlgorithm::Hs384(key) => {
            check_symmetric_key!(
                key: key,
                len_hint: jwt.signature_algorithm.key_len_hint(),
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification
            );

            Some(sign_hmac!(
                hash_alg: sha2::Sha384,
                key: key,
                msg: data_to_sign.as_bytes(),
            ))
        }
        JwtSignatureAlgorithm::Hs512(key) => {
            check_symmetric_key!(
                key: key,
                len_hint: jwt.signature_algorithm.key_len_hint(),
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification
            );

            Some(sign_hmac!(
                hash_alg: sha2::Sha512,
                key: key,
                msg: data_to_sign.as_bytes(),
            ))
        }
        JwtSignatureAlgorithm::Rs256(key) => {
            let private_key = check_asymmetric_key!(
                key: key,
                name: jwt.signature_algorithm.to_string(),
                notificator: &spawn_notification,
                key_kind: PrivateKey,
            );

            sign!(
                signature_algo: SignatureAlgorithm::RsaPkcs1v15,
                hash_algo: HashAlgorithm::SHA2_256,
                name: jwt.signature_algorithm.to_string(),
                private_key: &private_key,
                data_to_sign: data_to_sign.as_bytes(),
                notificator: &spawn_notification
            )
        }
        JwtSignatureAlgorithm::Rs384(key) => {
            let private_key = check_asymmetric_key!(
                key: key,
                name: jwt.signature_algorithm.to_string(),
                notificator: &spawn_notification,
                key_kind: PrivateKey,
            );

            sign!(
                signature_algo: SignatureAlgorithm::RsaPkcs1v15,
                hash_algo: HashAlgorithm::SHA2_384,
                name: jwt.signature_algorithm.to_string(),
                private_key: &private_key,
                data_to_sign: data_to_sign.as_bytes(),
                notificator: &spawn_notification
            )
        }
        JwtSignatureAlgorithm::Rs512(key) => {
            let private_key = check_asymmetric_key!(
                key: key,
                name: jwt.signature_algorithm.to_string(),
                notificator: &spawn_notification,
                key_kind: PrivateKey,
            );

            sign!(
                signature_algo: SignatureAlgorithm::RsaPkcs1v15,
                hash_algo: HashAlgorithm::SHA2_512,
                name: jwt.signature_algorithm.to_string(),
                private_key: &private_key,
                data_to_sign: data_to_sign.as_bytes(),
                notificator: &spawn_notification
            )
        }
        JwtSignatureAlgorithm::Es256(key) => {
            let private_key = check_asymmetric_key!(
                key: key,
                name: jwt.signature_algorithm.to_string(),
                notificator: &spawn_notification,
                key_kind: PrivateKey,
            );

            sign!(
                signature_algo: SignatureAlgorithm::Ecdsa,
                hash_algo: HashAlgorithm::SHA2_256,
                name: jwt.signature_algorithm.to_string(),
                private_key: &private_key,
                data_to_sign: data_to_sign.as_bytes(),
                notificator: &spawn_notification
            )
        }
        JwtSignatureAlgorithm::Es384(key) => {
            let private_key = check_asymmetric_key!(
                key: key,
                name: jwt.signature_algorithm.to_string(),
                notificator: &spawn_notification,
                key_kind: PrivateKey,
            );

            sign!(
                signature_algo: SignatureAlgorithm::Ecdsa,
                hash_algo: HashAlgorithm::SHA2_384,
                name: jwt.signature_algorithm.to_string(),
                private_key: &private_key,
                data_to_sign: data_to_sign.as_bytes(),
                notificator: &spawn_notification
            )
        }
        JwtSignatureAlgorithm::Es512(key) => {
            let private_key = check_asymmetric_key!(
                key: key,
                name: jwt.signature_algorithm.to_string(),
                notificator: &spawn_notification,
                key_kind: PrivateKey,
            );

            sign!(
                signature_algo: SignatureAlgorithm::Ecdsa,
                hash_algo: HashAlgorithm::SHA2_512,
                name: jwt.signature_algorithm.to_string(),
                private_key: &private_key,
                data_to_sign: data_to_sign.as_bytes(),
                notificator: &spawn_notification
            )
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
        STANDARD.encode(jwt.parsed_header.as_bytes()),
        STANDARD.encode(jwt.parsed_payload.as_bytes())
    );

    Some(match &jwt.signature_algorithm {
        JwtSignatureAlgorithm::None => Vec::<u8>::new() == jwt.signature,
        JwtSignatureAlgorithm::Hs256(key) => {
            check_symmetric_key!(
                key: key,
                len_hint: jwt.signature_algorithm.key_len_hint(),
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification
            );

            verify_hmac!(
                hash_alg: sha2::Sha384,
                key: key,
                msg: data_to_sign.as_bytes(),
                digest: jwt.signature.as_slice(),
            )
        }
        JwtSignatureAlgorithm::Hs384(key) => {
            check_symmetric_key!(
                key: key,
                len_hint: jwt.signature_algorithm.key_len_hint(),
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification
            );

            verify_hmac!(
                hash_alg: sha2::Sha384,
                key: key,
                msg: data_to_sign.as_bytes(),
                digest: jwt.signature.as_slice(),
            )
        }
        JwtSignatureAlgorithm::Hs512(key) => {
            check_symmetric_key!(
                key: key,
                len_hint: jwt.signature_algorithm.key_len_hint(),
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification
            );

            verify_hmac!(
                hash_alg: sha2::Sha512,
                key: key,
                msg: data_to_sign.as_bytes(),
                digest: jwt.signature.as_slice(),
            )
        }
        JwtSignatureAlgorithm::Rs256(key) => {
            let public_key = check_asymmetric_key!(
                key: key,
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification,
                key_kind: PublicKey,
            );

            verify!(
                signature_algo: SignatureAlgorithm::RsaPkcs1v15,
                hash_algo: HashAlgorithm::SHA2_256,
                public_key: &public_key,
                data_to_sign: data_to_sign.as_bytes(),
                jwt_signature: &jwt.signature,
                notificator: spawn_notification
            )
        }
        JwtSignatureAlgorithm::Rs384(key) => {
            let public_key = check_asymmetric_key!(
                key: key,
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification,
                key_kind: PublicKey,
            );

            verify!(
                signature_algo: SignatureAlgorithm::RsaPkcs1v15,
                hash_algo: HashAlgorithm::SHA2_384,
                public_key: &public_key,
                data_to_sign: data_to_sign.as_bytes(),
                jwt_signature: &jwt.signature,
                notificator: spawn_notification
            )
        }
        JwtSignatureAlgorithm::Rs512(key) => {
            let public_key = check_asymmetric_key!(
                key: key,
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification,
                key_kind: PublicKey,
            );

            verify!(
                signature_algo: SignatureAlgorithm::RsaPkcs1v15,
                hash_algo: HashAlgorithm::SHA2_512,
                public_key: &public_key,
                data_to_sign: data_to_sign.as_bytes(),
                jwt_signature: &jwt.signature,
                notificator: spawn_notification
            )
        }
        JwtSignatureAlgorithm::Es256(key) => {
            let public_key = check_asymmetric_key!(
                key: key,
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification,
                key_kind: PublicKey,
            );

            verify!(
                signature_algo: SignatureAlgorithm::Ecdsa,
                hash_algo: HashAlgorithm::SHA2_256,
                public_key: &public_key,
                data_to_sign: data_to_sign.as_bytes(),
                jwt_signature: &jwt.signature,
                notificator: spawn_notification
            )
        }
        JwtSignatureAlgorithm::Es384(key) => {
            let public_key = check_asymmetric_key!(
                key: key,
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification,
                key_kind: PublicKey,
            );

            verify!(
                signature_algo: SignatureAlgorithm::Ecdsa,
                hash_algo: HashAlgorithm::SHA2_384,
                public_key: &public_key,
                data_to_sign: data_to_sign.as_bytes(),
                jwt_signature: &jwt.signature,
                notificator: spawn_notification
            )
        }
        JwtSignatureAlgorithm::Es512(key) => {
            let public_key = check_asymmetric_key!(
                key: key,
                name: jwt.signature_algorithm.to_string(),
                notificator: spawn_notification,
                key_kind: PublicKey,
            );

            verify!(
                signature_algo: SignatureAlgorithm::Ecdsa,
                hash_algo: HashAlgorithm::SHA2_512,
                public_key: &public_key,
                data_to_sign: data_to_sign.as_bytes(),
                jwt_signature: &jwt.signature,
                notificator: spawn_notification
            )
        }
        JwtSignatureAlgorithm::Unsupported(algo_name) => {
            spawn_notification.emit(Notification::from_description_and_type(
                NotificationType::Warn,
                format!("Unsupported signature algorithm: {}.", algo_name,),
            ));

            return None;
        }
    })
}

pub fn generate_jwt(jwt: &Jwt, spawn_notification: Callback<Notification>) -> Option<Vec<u8>> {
    let signature = calculate_signature(jwt, spawn_notification)?;

    let engine = GeneralPurpose::new(&base64::alphabet::STANDARD, base64::engine::general_purpose::NO_PAD);

    let header = engine.encode(jwt.parsed_header.as_bytes());
    let payload = engine.encode(jwt.parsed_payload.as_bytes());
    let signature = engine.encode(signature);

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
    let notifications = use_notification::<Notification>();
    let clipboard = use_clipboard();
    let share_by_link = Callback::from(move |_| {
        let notifications_manager = notifications.clone();
        if let Some(new_jwt) = generate_jwt(
            &jwt,
            Callback::from(move |notification| notifications_manager.spawn(notification)),
        )
        .and_then(|data| String::from_utf8(data).ok())
        {
            clipboard.write_text(generate_jwt_link(new_jwt));

            notifications.spawn(Notification::from_description_and_type(
                NotificationType::Info,
                "link to jwt copied",
            ));
        } else {
            notifications.spawn(Notification::from_description_and_type(
                NotificationType::Error,
                "can not generate jwt",
            ));
        }
    });

    let jwt = props.jwt.clone();
    let set_jwt = props.set_jwt.clone();
    let notifications = use_notification::<Notification>();

    html! {
        <div class="vertical">
            {get_input_component(&props.jwt.signature_algorithm, Callback::from(move |signature_algo| {
                let mut new_jwt = jwt.clone();
                new_jwt.signature_algorithm = signature_algo;

                set_jwt.emit(new_jwt);
            }))}
            {if props.jwt.signature_algorithm.is_supported() {
                html! {
                    <div class="horizontal">
                        <button class="jwt-util-button" onclick={validate}>{"Validate signature"}</button>
                        <button class="jwt-util-button" onclick={recalculate}>{"Recalculate signature"}</button>
                        <button class="jwt-util-button" onclick={generate}>{"Generate JWT"}</button>
                        <button class="button-with-icon" onclick={share_by_link}>
                            <img src="/public/img/icons/share_by_link.png" />
                        </button>
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
