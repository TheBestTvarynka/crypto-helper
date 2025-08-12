use yew::{Callback, Html, Properties, function_component, html, use_effect_with, use_state};
use yew_hooks::use_clipboard;
use yew_notifications::{Notification, NotificationType};

use super::BytesFormat;
use crate::common::{BYTES_FORMATS, encode_bytes, get_format_button_class, get_set_format_callback};

#[derive(PartialEq, Properties, Clone)]
pub struct SimpleOutputProps {
    output: Vec<u8>,
    format: BytesFormat,
    add_notification: Callback<Notification>,
}

#[function_component(SimpleOutput)]
pub fn simple_output(props: &SimpleOutputProps) -> Html {
    let SimpleOutputProps {
        output,
        format,
        add_notification,
    } = props.clone();

    let bytes_format = use_state(|| format);

    let format_setter = bytes_format.setter();
    use_effect_with(bytes_format.clone(), move |format| {
        format_setter.set(**format);
    });

    let encoded_bytes = encode_bytes(&output, *bytes_format);

    let encoded = encoded_bytes.clone();
    let clipboard = use_clipboard();
    let onclick = Callback::from(move |_| {
        clipboard.write_text(encoded.clone());

        add_notification.emit(Notification::from_description_and_type(
            NotificationType::Info,
            "output copied",
        ));
    });

    html! {
        <div class="output">
            <div class="formats-container">{
                BYTES_FORMATS.iter().map(|format| {
                    html! {
                        <button
                            class={get_format_button_class(*bytes_format == *format)}
                            onclick={get_set_format_callback(*format, bytes_format.setter())}
                        >
                            {<&str>::from(format)}
                        </button>
                    }
                }).collect::<Html>()
            }</div>
            <span class="simple-digest" onclick={onclick}>{encoded_bytes}</span>
            <span class="total">{format!("total: {}", output.len())}</span>
        </div>
    }
}

pub fn build_simple_output(output: Vec<u8>, format: BytesFormat, add_notification: Callback<Notification>) -> Html {
    html! {
        <SimpleOutput {output} {format} {add_notification} />
    }
}
