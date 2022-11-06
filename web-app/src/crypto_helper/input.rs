mod krb;
mod simple;

use yew::{classes, function_component, html, Callback, Html, Properties, UseStateSetter};

use self::{krb::build_krb_input, simple::build_simple_input};

use super::Algorithm;

fn get_input_components(algorithm: &Algorithm, setter: &UseStateSetter<Algorithm>) -> Html {
    let setter = setter.clone();
    match algorithm {
        Algorithm::Md5(input) => build_simple_input(
            input.clone(),
            Callback::from(move |input| setter.set(Algorithm::Md5(input))),
        ),
        Algorithm::Sha1(input) => build_simple_input(
            input.clone(),
            Callback::from(move |input| setter.set(Algorithm::Sha1(input))),
        ),
        Algorithm::Sha256(input) => build_simple_input(
            input.clone(),
            Callback::from(move |input| setter.set(Algorithm::Sha256(input))),
        ),
        Algorithm::Sha512(input) => build_simple_input(
            input.clone(),
            Callback::from(move |input| setter.set(Algorithm::Sha512(input))),
        ),
        Algorithm::Aes128CtsHmacSha196(kerberos_input) => build_krb_input(
            kerberos_input.clone(),
            Callback::from(move |kerberos_input| {
                setter.set(Algorithm::Aes128CtsHmacSha196(kerberos_input))
            }),
        ),
        Algorithm::Aes256CtsHmacSha196(kerberos_input) => build_krb_input(
            kerberos_input.clone(),
            Callback::from(move |kerberos_input| {
                setter.set(Algorithm::Aes256CtsHmacSha196(kerberos_input))
            }),
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
