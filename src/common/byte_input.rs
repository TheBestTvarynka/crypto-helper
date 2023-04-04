use web_sys::HtmlInputElement;
use yew::{Html, html, function_component, Callback, Properties, classes, use_state, use_effect_with_deps, TargetCast};
use yew_notifications::{use_notification, Notification, NotificationType};

use crate::common::{BYTES_FORMATS, get_format_button_class, encode_bytes, get_set_format_callback, parse_bytes};

use super::BytesFormat;

#[derive(PartialEq, Properties, Clone)]
pub struct ByteInputProps {
    format: BytesFormat,
    bytes: Vec<u8>,
    setter: Callback<Vec<u8>>,
}

#[function_component(ByteInput)]
pub fn byte_input(props: &ByteInputProps) -> Html {
    let ByteInputProps { format, bytes, setter } = &props;

    let bytes_format = use_state(|| *format);
    let raw_value = use_state(|| encode_bytes(bytes, format));

    let format_setter = bytes_format.setter();
    use_effect_with_deps(
        move |format| {
            format_setter.set(**format);
        },
        bytes_format.clone(),
    );

    let setter = setter.clone();
    let notifications = use_notification::<Notification>();
    let format = *bytes_format;
    let oninput = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let value = input.value();

        match parse_bytes(&value, format) {
            Ok(bytes) => setter.emit(bytes),
            Err(error) => notifications.spawn(Notification::new(NotificationType::Error, "Can not parse input", error.to_string())),
        }
    });

    html! {
        <div class={classes!("output")}>
            <div class={classes!("formats-container")}>{
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
            <textarea
                rows="2"
                placeholder={"place input here"}
                class={classes!("base-input")}
                value={(*raw_value).clone()}
                {oninput}
            />
            <span class={classes!("total")}>{format!("total: {}", (*bytes).len())}</span>
        </div>
    }
}