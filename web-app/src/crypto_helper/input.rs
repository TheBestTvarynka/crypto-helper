mod simple;
mod krb;

use yew::{html, Html, Properties, function_component, classes, UseStateSetter, Callback};

use self::{simple::build_simple_input, krb::build_krb_input};

use super::Algorithm;

fn get_input_components(algorithm: &Algorithm, setter: &UseStateSetter<Algorithm>) -> Html {
    match algorithm {
        Algorithm::Md5(_) => build_simple_input(algorithm),
        Algorithm::Sha1(_) => build_simple_input(algorithm),
        Algorithm::Sha256(_) => build_simple_input(algorithm),
        Algorithm::Sha512(_) => build_simple_input(algorithm),
        Algorithm::Aes128CtsHmacSha196(kerberos_input) => {
            let setter = setter.clone();
            build_krb_input(kerberos_input.clone(), Callback::from(move |kerberos_input| {
                setter.set(Algorithm::Aes128CtsHmacSha196(kerberos_input))
            }))
        },
        Algorithm::Aes256CtsHmacSha196(kerberos_input) => {
            let setter = setter.clone();
            build_krb_input(kerberos_input.clone(), Callback::from(move |kerberos_input| {
                setter.set(Algorithm::Aes256CtsHmacSha196(kerberos_input))
            }))
        },
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
        <div class={classes!("in-container")}>
            {get_input_components(&props.algorithm, &props.setter)}
        </div>
    }
}
