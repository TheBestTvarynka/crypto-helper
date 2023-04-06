mod algorithm;
mod computations;
mod info;
mod input;
mod output;

pub use algorithm::RSA_HASH_ALGOS;
use info::Info;
use input::Input;
use output::Output;
use picky_krb::crypto::{ChecksumSuite, CipherSuite};
use sha1::{Digest, Sha1};
use yew::{classes, function_component, html, use_state, Callback, Html};
use yew_notifications::{use_notification, Notification, NotificationType};

use self::algorithm::Algorithm;
use self::computations::{process_krb_cipher, process_krb_hmac, process_rsa};

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
    let onclick = Callback::from(move |_| {
        match convert(&algorithm_data) {
            Ok(output) => output_setter.set(output),
            Err(err) => notification_manager.spawn(Notification::new(NotificationType::Error, "Processing error", err)),
        };
    });

    html! {
        <article class={classes!("vertical")}>
            <Info set_algorithm={algorithm.setter()} algorithm={(*algorithm).clone()} />
            <Input algorithm={(*algorithm).clone()} setter={algorithm.setter()} />
            <div class={classes!("horizontal")}>
                <button {onclick}>{"Go"}</button>
            </div>
            <Output algorithm={(*algorithm).clone()} output={(*output).clone()} />
        </article>
    }
}
