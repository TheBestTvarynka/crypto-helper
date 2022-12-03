mod krb;
mod rsa;

use picky_krb::crypto::CipherSuite;
use yew::{classes, function_component, html, Callback, Html, Properties, UseStateSetter};

use crate::common::build_simple_input;

use self::{krb::build_krb_input, rsa::build_rsa_input};

use super::{algorithm::KrbInput, Algorithm};

fn get_input_components(algorithm: &Algorithm, setter: &UseStateSetter<Algorithm>) -> Html {
    let setter = setter.clone();
    match algorithm {
        Algorithm::Md5(input) => build_simple_input(
            input.clone(),
            "hex-encoded MD5 input".into(),
            Callback::from(move |input| setter.set(Algorithm::Md5(input))),
        ),
        Algorithm::Sha1(input) => build_simple_input(
            input.clone(),
            "hex-encoded SHA1 input".into(),
            Callback::from(move |input| setter.set(Algorithm::Sha1(input))),
        ),
        Algorithm::Sha256(input) => build_simple_input(
            input.clone(),
            "hex-encoded SHA256 input".into(),
            Callback::from(move |input| setter.set(Algorithm::Sha256(input))),
        ),
        Algorithm::Sha512(input) => build_simple_input(
            input.clone(),
            "hex-encoded SHA512 input".into(),
            Callback::from(move |input| setter.set(Algorithm::Sha512(input))),
        ),
        Algorithm::Aes128CtsHmacSha196(kerberos_input) => build_krb_input(
            kerberos_input.clone(),
            Callback::from(move |kerberos_input| {
                setter.set(Algorithm::Aes128CtsHmacSha196(kerberos_input))
            }),
            CipherSuite::Aes128CtsHmacSha196,
            true,
        ),
        Algorithm::Aes256CtsHmacSha196(kerberos_input) => build_krb_input(
            kerberos_input.clone(),
            Callback::from(move |kerberos_input| {
                setter.set(Algorithm::Aes256CtsHmacSha196(kerberos_input))
            }),
            CipherSuite::Aes256CtsHmacSha196,
            true,
        ),
        Algorithm::HmacSha196Aes128(kerberos_input) => build_krb_input(
            KrbInput {
                data: kerberos_input.clone(),
                mode: false,
            },
            Callback::from(move |kerberos_input: KrbInput| {
                setter.set(Algorithm::HmacSha196Aes128(kerberos_input.data))
            }),
            CipherSuite::Aes128CtsHmacSha196,
            false,
        ),
        Algorithm::HmacSha196Aes256(kerberos_input) => build_krb_input(
            KrbInput {
                data: kerberos_input.clone(),
                mode: false,
            },
            Callback::from(move |kerberos_input: KrbInput| {
                setter.set(Algorithm::HmacSha196Aes256(kerberos_input.data))
            }),
            CipherSuite::Aes256CtsHmacSha196,
            false,
        ),
        Algorithm::Rsa(input) => build_rsa_input(
            input.clone(),
            Callback::from(move |input| setter.set(Algorithm::Rsa(input))),
        ),
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct InputProps {
    pub algorithm: Algorithm,
    pub setter: UseStateSetter<Algorithm>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    html! {
        <div class={classes!("container")}>
            {get_input_components(&props.algorithm, &props.setter)}
        </div>
    }
}
