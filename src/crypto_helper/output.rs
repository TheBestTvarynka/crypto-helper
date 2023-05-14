mod krb;

use yew::{classes, function_component, html, Callback, Html, Properties};
use yew_notifications::{use_notification, Notification};

use self::krb::build_krb_output;
use super::Algorithm;
use crate::common::{build_simple_output, BytesFormat};

fn get_output_components(algorithm: &Algorithm, output: &[u8], add_notification: Callback<Notification>) -> Html {
    match algorithm {
        Algorithm::Md5(_) => build_simple_output(output.into(), BytesFormat::Hex, add_notification),
        Algorithm::Sha1(_) => build_simple_output(output.into(), BytesFormat::Hex, add_notification),
        Algorithm::Sha256(_) => build_simple_output(output.into(), BytesFormat::Hex, add_notification),
        Algorithm::Sha384(_) => build_simple_output(output.into(), BytesFormat::Hex, add_notification),
        Algorithm::Sha512(_) => build_simple_output(output.into(), BytesFormat::Hex, add_notification),
        Algorithm::Aes128CtsHmacSha196(input) => build_krb_output(input.mode, output.to_vec(), add_notification),
        Algorithm::Aes256CtsHmacSha196(input) => build_krb_output(input.mode, output.to_vec(), add_notification),
        Algorithm::HmacSha196Aes128(_) => build_simple_output(output.into(), BytesFormat::Hex, add_notification),
        Algorithm::HmacSha196Aes256(_) => build_simple_output(output.into(), BytesFormat::Hex, add_notification),
        Algorithm::Rsa(_) => build_simple_output(output.into(), BytesFormat::Hex, add_notification),
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct OutputProps {
    pub algorithm: Algorithm,
    pub output: Vec<u8>,
}

#[function_component(Output)]
pub fn output(props: &OutputProps) -> Html {
    let notification_manager = use_notification::<Notification>();

    let add_notification = Callback::from(move |n| {
        notification_manager.spawn(n);
    });

    html! {
        <div class={classes!("container")}>
            {get_output_components(&props.algorithm, &props.output, add_notification)}
        </div>
    }
}
