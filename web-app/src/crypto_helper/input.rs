mod simple;
mod krb;

use yew::{html, Html, Properties, function_component, classes};

use self::{simple::build_simple_input, krb::build_krb_input};

use super::Algorithm;

fn get_input_components(algorithm: &Algorithm) -> Html {
    match algorithm {
        Algorithm::Md5 => build_simple_input(algorithm),
        Algorithm::Sha1 => build_simple_input(algorithm),
        Algorithm::Sha256 => build_simple_input(algorithm),
        Algorithm::Sha512 => build_simple_input(algorithm),
        Algorithm::Aes128CtsHmacSha196 => build_krb_input(),
        Algorithm::Aes256CtsHmacSha196 => build_krb_input(),
    }
}

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct InputProps {
    pub algorithm: Algorithm,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    html! {
        <div class={classes!("in-container")}>
            {get_input_components(&props.algorithm)}
        </div>
    }
}
