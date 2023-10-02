mod argon2;
mod bcrypt;
mod krb;
mod rsa;
mod zlib;

use picky_krb::crypto::CipherSuite;
use yew::{function_component, html, Callback, Html, Properties, UseStateSetter};

use self::argon2::build_argon2_input;
use self::bcrypt::build_bcrypt_input;
use self::krb::build_krb_input;
use self::rsa::build_rsa_input;
use self::zlib::build_zlib_input;
use super::algorithm::{KrbInput, KrbMode};
use super::Algorithm;
use crate::common::build_byte_input;

fn get_input_components(algorithm: &Algorithm, setter: &UseStateSetter<Algorithm>) -> Html {
    let setter = setter.clone();
    match algorithm {
        Algorithm::Md5(input) => build_byte_input(
            input.clone(),
            Callback::from(move |input| setter.set(Algorithm::Md5(input))),
            None,
            Some("md5".into()),
        ),
        Algorithm::Sha1(input) => build_byte_input(
            input.clone(),
            Callback::from(move |input| setter.set(Algorithm::Sha1(input))),
            None,
            Some("sha1".into()),
        ),
        Algorithm::Sha256(input) => build_byte_input(
            input.clone(),
            Callback::from(move |input| setter.set(Algorithm::Sha256(input))),
            None,
            Some("sha256".into()),
        ),
        Algorithm::Sha384(input) => build_byte_input(
            input.clone(),
            Callback::from(move |input| setter.set(Algorithm::Sha384(input))),
            None,
            Some("sha384".into()),
        ),
        Algorithm::Sha512(input) => build_byte_input(
            input.clone(),
            Callback::from(move |input| setter.set(Algorithm::Sha512(input))),
            None,
            Some("sha512".into()),
        ),
        Algorithm::Aes128CtsHmacSha196(kerberos_input) => build_krb_input(
            kerberos_input.clone(),
            Callback::from(move |kerberos_input| setter.set(Algorithm::Aes128CtsHmacSha196(kerberos_input))),
            CipherSuite::Aes128CtsHmacSha196,
            true,
        ),
        Algorithm::Aes256CtsHmacSha196(kerberos_input) => build_krb_input(
            kerberos_input.clone(),
            Callback::from(move |kerberos_input| setter.set(Algorithm::Aes256CtsHmacSha196(kerberos_input))),
            CipherSuite::Aes256CtsHmacSha196,
            true,
        ),
        Algorithm::HmacSha196Aes128(kerberos_input) => build_krb_input(
            KrbInput {
                data: kerberos_input.clone(),
                mode: KrbMode::Encrypt,
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
                mode: KrbMode::Encrypt,
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
        Algorithm::Bcrypt(input) => build_bcrypt_input(
            input.clone(),
            Callback::from(move |input| setter.set(Algorithm::Bcrypt(input))),
        ),
        Algorithm::Zlib(input) => build_zlib_input(
            input.clone(),
            Callback::from(move |input| setter.set(Algorithm::Zlib(input))),
        ),
        Algorithm::Argon2(input) => build_argon2_input(
            input.clone(),
            Callback::from(move |input| setter.set(Algorithm::Argon2(input))),
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
        <div class="container">
            {get_input_components(&props.algorithm, &props.setter)}
        </div>
    }
}
