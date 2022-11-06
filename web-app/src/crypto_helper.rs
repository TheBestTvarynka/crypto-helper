use picky_krb::crypto::CipherSuite;
use sha1::{Digest, Sha1};
use yew::{classes, function_component, html, use_state, Callback, Html};

mod algorithm;
mod info;
mod input;
mod output;

use info::Info;
use input::Input;
use output::Output;

use self::algorithm::Algorithm;

fn from_hex(input: &str) -> Vec<u8> {
    hex::decode(input).unwrap_or_default()
}

fn convert(algrithm: &Algorithm) -> Vec<u8> {
    match algrithm {
        Algorithm::Md5(input) => md5::compute(from_hex(input)).to_vec(),
        Algorithm::Sha1(input) => {
            let mut sha1 = Sha1::new();
            sha1.update(from_hex(input));
            sha1.finalize().to_vec()
        }
        Algorithm::Sha256(input) => hmac_sha256::Hash::hash(&from_hex(input)).to_vec(),
        Algorithm::Sha512(input) => hmac_sha512::Hash::hash(&from_hex(input)).to_vec(),
        Algorithm::Aes128CtsHmacSha196(input) => {
            if input.mode {
                CipherSuite::Aes128CtsHmacSha196
                    .cipher()
                    .decrypt(
                        &from_hex(&input.key),
                        input.key_usage.parse::<i32>().unwrap_or_default(),
                        &from_hex(&input.payload),
                    )
                    .unwrap_or_default()
            } else {
                CipherSuite::Aes128CtsHmacSha196
                    .cipher()
                    .encrypt(
                        &from_hex(&input.key),
                        input.key_usage.parse::<i32>().unwrap_or_default(),
                        &from_hex(&input.payload),
                    )
                    .unwrap_or_default()
            }
        }
        Algorithm::Aes256CtsHmacSha196(input) => {
            if input.mode {
                CipherSuite::Aes256CtsHmacSha196
                    .cipher()
                    .decrypt(
                        &from_hex(&input.key),
                        input.key_usage.parse::<i32>().unwrap_or_default(),
                        &from_hex(&input.payload),
                    )
                    .unwrap_or_default()
            } else {
                CipherSuite::Aes256CtsHmacSha196
                    .cipher()
                    .encrypt(
                        &from_hex(&input.key),
                        input.key_usage.parse::<i32>().unwrap_or_default(),
                        &from_hex(&input.payload),
                    )
                    .unwrap_or_default()
            }
        }
    }
}

#[function_component(CryptoHelper)]
pub fn crypto_helper() -> Html {
    let algorithm = use_state(Algorithm::default);
    let output = use_state(Vec::new);

    let output_setter = output.setter();
    let algorithm_data = (*algorithm).clone();
    let onclick = Callback::from(move |_| {
        output_setter.set(convert(&algorithm_data));
    });

    html! {
        <article class={classes!("vertical")}>
            <Info set_algorithm={algorithm.setter()} algorithm={(*algorithm).clone()} />
            <Input algorithm={(*algorithm).clone()} setter={algorithm.setter()} />
            <div class={classes!("horizontal")}>
                <button onclick={onclick}>{"Go"}</button>
                <label for={"autoConvert"}>
                    <input type={"checkbox"} id={"autoConvert"} /><span>{"autogo"}</span>
                </label>
            </div>
            <Output algorithm={(*algorithm).clone()} output={(*output).clone()} />
        </article>
    }
}
