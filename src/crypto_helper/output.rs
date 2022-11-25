mod krb;
mod simple;

use yew::{classes, function_component, html, Callback, Html, Properties};

use crate::notification::Notification;

use self::{krb::build_krb_output, simple::build_simple_output};

use super::Algorithm;

fn get_output_components(
    algorithm: &Algorithm,
    output: &[u8],
    add_notification: &Callback<Notification>,
) -> Html {
    match algorithm {
        Algorithm::Md5(_) => build_simple_output(output, add_notification.clone()),
        Algorithm::Sha1(_) => build_simple_output(output, add_notification.clone()),
        Algorithm::Sha256(_) => build_simple_output(output, add_notification.clone()),
        Algorithm::Sha512(_) => build_simple_output(output, add_notification.clone()),
        Algorithm::Aes128CtsHmacSha196(input) => {
            build_krb_output(input, output, add_notification.clone())
        }
        Algorithm::Aes256CtsHmacSha196(input) => {
            build_krb_output(input, output, add_notification.clone())
        }
        Algorithm::HmacSha196Aes128(_) => build_simple_output(output, add_notification.clone()),
        Algorithm::HmacSha196Aes256(_) => build_simple_output(output, add_notification.clone()),
        Algorithm::Rsa(_) => build_simple_output(output, add_notification.clone()),
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct OutputProps {
    pub algorithm: Algorithm,
    pub output: Vec<u8>,
    pub add_notification: Callback<Notification>,
}

#[function_component(Output)]
pub fn output(props: &OutputProps) -> Html {
    html! {
        <div class={classes!("container")}>
            {get_output_components(&props.algorithm, &props.output, &props.add_notification)}
        </div>
    }
}
