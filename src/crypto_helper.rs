mod algorithm;
mod computations;
mod info;
mod input;
mod macros;
mod output;
mod serde;

pub use algorithm::{Algorithm, KrbInput, KrbInputData, KrbMode, RSA_HASH_ALGOS};
use info::Info;
use input::Input;
use output::Output;
use picky_krb::crypto::{ChecksumSuite, CipherSuite};
use sha1::{Digest, Sha1};
use yew::{classes, function_component, html, use_effect_with_deps, use_state, Callback, Html};
use yew_hooks::{use_clipboard, use_location};
use yew_notifications::{use_notification, Notification, NotificationType};

use self::computations::{process_krb_cipher, process_krb_hmac, process_rsa};
use crate::url_query_params::generate_crypto_helper_link;

fn from_hex(input: &str) -> Result<Vec<u8>, String> {
    hex::decode(input).map_err(|err| format!("invalid hex input:{:?}", err))
}

fn convert(algrithm: &Algorithm) -> Result<Vec<u8>, String> {
    match algrithm {
        Algorithm::Md5(input) => Ok(md5::compute(input).to_vec()),
        Algorithm::Sha1(input) => {
            let mut sha1 = Sha1::new();
            sha1.update(input);
            Ok(sha1.finalize().to_vec())
        }
        Algorithm::Sha256(input) => Ok(hmac_sha256::Hash::hash(input).to_vec()),
        Algorithm::Sha384(input) => Ok(hmac_sha512::sha384::Hash::hash(input).to_vec()),
        Algorithm::Sha512(input) => Ok(hmac_sha512::Hash::hash(input).to_vec()),
        Algorithm::Aes128CtsHmacSha196(input) => process_krb_cipher(CipherSuite::Aes128CtsHmacSha196.cipher(), input),
        Algorithm::Aes256CtsHmacSha196(input) => process_krb_cipher(CipherSuite::Aes256CtsHmacSha196.cipher(), input),
        Algorithm::HmacSha196Aes128(input) => process_krb_hmac(ChecksumSuite::HmacSha196Aes128.hasher(), input),
        Algorithm::HmacSha196Aes256(input) => process_krb_hmac(ChecksumSuite::HmacSha196Aes256.hasher(), input),
        Algorithm::Rsa(input) => process_rsa(input),
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
    let onclick = Callback::from(move |_| {
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

    let algorithm_setter = algorithm.setter();
    let r = use_location();
    let notifications = notification_manager.clone();
    use_effect_with_deps(
        move |_: &[(); 0]| {
            let query = &r.search;

            if query.len() < 2 {
                return;
            }

            log::debug!("remove question mark from url query: {:?}", query.chars().next());

            match serde_qs::from_str(&query[1..]) {
                Ok(algorithm) => {
                    algorithm_setter.set(algorithm);
                }
                Err(err) => notifications.spawn(Notification::new(
                    NotificationType::Error,
                    "Can not load data from url",
                    err.to_string(),
                    Notification::NOTIFICATION_LIFETIME,
                )),
            }
        },
        [],
    );

    let algorithm_data = (*algorithm).clone();
    let clipboard = use_clipboard();
    let share_by_link = Callback::from(move |_| {
        clipboard.write_text(generate_crypto_helper_link(&algorithm_data));

        notification_manager.spawn(Notification::from_description_and_type(
            NotificationType::Info,
            "link copied",
        ));
    });

    html! {
        <article class={classes!("vertical")}>
            <Info set_algorithm={algorithm.setter()} algorithm={(*algorithm).clone()} />
            <Input algorithm={(*algorithm).clone()} setter={algorithm.setter()} />
            <div class={classes!("horizontal")}>
                <button {onclick}>{"Go"}</button>
            </div>
            <Output algorithm={(*algorithm).clone()} output={(*output).clone()} />
            <div class={classes!("horizontal")}>
                <button class={classes!("button-with-icon")} onclick={share_by_link}>
                    <img src="/public/img/icons/share_by_link.png" />
                </button>
            </div>
        </article>
    }
}
