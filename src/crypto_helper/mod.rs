#[macro_use]
pub mod macros;

mod algorithm;
mod computations;
mod info;
mod input;
mod output;

pub use algorithm::Algorithm;
use info::Info;
use input::Input;
use output::Output;
use picky_krb::crypto::{ChecksumSuite, CipherSuite};
use sha1::{Digest, Sha1};
use web_sys::KeyboardEvent;
use yew::{function_component, html, use_effect_with, use_state, Callback, Html};
use yew_hooks::{use_clipboard, use_local_storage, use_location};
use yew_notifications::{use_notification, Notification, NotificationType};

use self::computations::{
    process_argon2, process_hmac_sha, process_krb_cipher, process_krb_hmac, process_rsa, process_zlib,
};
use crate::crypto_helper::computations::process_bcrypt;
use crate::url_query_params::{generate_crypto_helper_link, Asn1};

const CRYPTO_HELPER_LOCAL_STORAGE_KEY: &str = "CRYPTO_HELPER_DATA";

fn convert(algorithm: &Algorithm) -> Result<Vec<u8>, String> {
    match algorithm {
        Algorithm::Md5(input) => Ok(md5::compute(input).to_vec()),
        Algorithm::Sha1(input) => {
            let mut sha1 = Sha1::new();
            sha1.update(input);
            Ok(sha1.finalize().to_vec())
        }
        Algorithm::Sha256(input) => Ok({
            use sha2::Digest;

            let mut hasher = sha2::Sha256::new();
            hasher.update(input);
            hasher.finalize().to_vec()
        }),
        Algorithm::Sha384(input) => Ok({
            use sha2::Digest;

            let mut hasher = sha2::Sha384::new();
            hasher.update(input);
            hasher.finalize().to_vec()
        }),
        Algorithm::Sha512(input) => Ok({
            use sha2::Digest;

            let mut hasher = sha2::Sha512::new();
            hasher.update(input);
            hasher.finalize().to_vec()
        }),
        Algorithm::Aes128CtsHmacSha196(input) => process_krb_cipher(CipherSuite::Aes128CtsHmacSha196.cipher(), input),
        Algorithm::Aes256CtsHmacSha196(input) => process_krb_cipher(CipherSuite::Aes256CtsHmacSha196.cipher(), input),
        Algorithm::HmacSha196Aes128(input) => process_krb_hmac(ChecksumSuite::HmacSha196Aes128.hasher(), input),
        Algorithm::HmacSha196Aes256(input) => process_krb_hmac(ChecksumSuite::HmacSha196Aes256.hasher(), input),
        Algorithm::Rsa(input) => process_rsa(input),
        Algorithm::Bcrypt(input) => process_bcrypt(input),
        Algorithm::Zlib(input) => process_zlib(input),
        Algorithm::Argon2(input) => process_argon2(input),
        Algorithm::HmacSha(input) => process_hmac_sha(input),
    }
}

#[function_component(CryptoHelper)]
pub fn crypto_helper() -> Html {
    let notification_manager = use_notification::<Notification>();

    let algorithm = use_state(Algorithm::default);
    let output = use_state(Vec::new);

    let output_setter = output.setter();
    let algorithm_data = (*algorithm).clone();
    let notifications = notification_manager.clone();
    let go = Callback::from(move |_: ()| {
        match convert(&algorithm_data) {
            Ok(output) => output_setter.set(output),
            Err(err) => notifications.spawn(Notification::new(
                NotificationType::Error,
                "Processing error",
                err,
                Notification::NOTIFICATION_LIFETIME,
            )),
        };
    });
    let go_onclick = go.clone();
    let onclick = Callback::from(move |_| {
        go_onclick.emit(());
    });

    let algorithm_setter = algorithm.setter();
    let location = use_location();
    let notifications = notification_manager.clone();
    let local_storage = use_local_storage::<String>(CRYPTO_HELPER_LOCAL_STORAGE_KEY.to_owned());
    let notification_manager_clone = notifications.clone();
    use_effect_with([], move |_: &[(); 0]| {
        let query = &location.search;

        // First, we try to load data from the url.
        // question mark + one any other char
        if query.len() >= 2 {
            match serde_qs::from_str(&query[1..]) {
                Ok(algorithm) => {
                    algorithm_setter.set(algorithm);
                }
                Err(err) => notification_manager_clone.spawn(Notification::new(
                    NotificationType::Error,
                    "Can not load data from url",
                    err.to_string(),
                    Notification::NOTIFICATION_LIFETIME,
                )),
            }
        } else {
            let raw_data = if let Some(raw_data) = (*local_storage).as_ref() {
                raw_data.as_str()
            } else {
                return;
            };
            match serde_json::from_str(raw_data) {
                Ok(algorithm) => {
                    algorithm_setter.set(algorithm);
                }
                Err(err) => notification_manager_clone.spawn(Notification::new(
                    NotificationType::Error,
                    "Can not load data from the local storage",
                    err.to_string(),
                    Notification::NOTIFICATION_LIFETIME,
                )),
            }
        }
    });

    let local_storage = use_local_storage::<String>(CRYPTO_HELPER_LOCAL_STORAGE_KEY.to_owned());
    use_effect_with(algorithm.clone(), move |algorithm| {
        let algorithm: &Algorithm = algorithm;
        local_storage
            .set(serde_json::to_string(algorithm).expect("algorithm serialization into json string should never fail"));
    });

    let algorithm_data = (*algorithm).clone();
    let clipboard = use_clipboard();

    let notification_manager_clone = notifications.clone();
    let share_by_link = Callback::from(move |_| {
        clipboard.write_text(generate_crypto_helper_link(&algorithm_data));
        notification_manager_clone.spawn(Notification::from_description_and_type(
            NotificationType::Info,
            "link copied",
        ));
    });

    let output_data = (*output).clone();
    let notification_manager_clone = notifications.clone();
    let decode_as_asn1 = Callback::from(move |_| {
        if output_data.is_empty() {
            notification_manager_clone.spawn(Notification::new(
                NotificationType::Warn,
                "No output to decode",
                "Please perform a computation first.",
                Notification::NOTIFICATION_LIFETIME,
            ));
        } else {
            let query = Asn1 {
                asn1: output_data.clone(),
            };
            let query_string = serde_qs::to_string(&query).expect("Failed to serialize query");
            let url = format!("/asn1?{}", query_string);
            web_sys::window()
                .expect("no global `window` exists")
                .open_with_url(&url)
                .expect("Failed to open new tab");
        }
    });

    let onkeydown = Callback::from(move |event: KeyboardEvent| {
        if event.ctrl_key() && event.code() == "Enter" {
            go.emit(());
        }
    });

    html! {
        <article class="vertical" {onkeydown}>
            <Info set_algorithm={algorithm.setter()} algorithm={(*algorithm).clone()} />
            <Input algorithm={(*algorithm).clone()} setter={algorithm.setter()} />
            <div class="horizontal">
                <button class="action-button" {onclick}>{"Go"}</button>
                <span class="total">{"(ctrl+enter)"}</span>
            </div>
            <Output algorithm={(*algorithm).clone()} output={(*output).clone()} />
            <div class="horizontal">
                <button class="button-with-icon" onclick={share_by_link}>
                    <img src="/public/img/icons/share_by_link.png" />
                </button>
                <button class="button-with-icon" onclick={decode_as_asn1}>
                    <img src="/public/img/icons/linking.png" />
                    {"ans1"}
                </button>
            </div>
        </article>
    }
}
