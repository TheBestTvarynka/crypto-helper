mod krb;
mod simple;

use yew::{classes, function_component, html, Html, Properties};

use self::{krb::build_krb_output, simple::build_simple_output};

use super::Algorithm;

fn get_output_components(algorithm: &Algorithm, output: &[u8]) -> Html {
    match algorithm {
        Algorithm::Md5(_) => build_simple_output(output),
        Algorithm::Sha1(_) => build_simple_output(output),
        Algorithm::Sha256(_) => build_simple_output(output),
        Algorithm::Sha512(_) => build_simple_output(output),
        Algorithm::Aes128CtsHmacSha196(input) => build_krb_output(input, output),
        Algorithm::Aes256CtsHmacSha196(input) => build_krb_output(input, output),
        Algorithm::HmacSha196Aes128(_) => build_simple_output(output),
        Algorithm::HmacSha196Aes256(_) => build_simple_output(output),
    }
}

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct OutputProps {
    pub algorithm: Algorithm,
    pub output: Vec<u8>,
}

#[function_component(Output)]
pub fn output(props: &OutputProps) -> Html {
    html! {
        <div class={classes!("container")}>
            {get_output_components(&props.algorithm, &props.output)}
        </div>
    }
}
