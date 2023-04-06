use std::os::raw;

use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, use_effect_with_deps, use_state, Callback, Html, Properties, TargetCast};
use yew_notifications::{use_notification, Notification, NotificationType};

use super::BytesFormat;
use crate::common::{encode_bytes, get_format_button_class, get_set_format_callback, parse_bytes, BYTES_FORMATS};

#[derive(PartialEq, Properties, Clone)]
pub struct ByteInputProps {
    #[prop_or(BytesFormat::Hex)]
    format: BytesFormat,
    bytes: Vec<u8>,
    setter: Callback<Vec<u8>>,
}

#[function_component(ByteInput)]
pub fn byte_input(props: &ByteInputProps) -> Html {
    let ByteInputProps { format, bytes, setter } = &props;

    let raw_value = use_state(|| encode_bytes(bytes, *format));
    let bytes = use_state(|| bytes.clone());
    let bytes_format = use_state(|| *format);

    let format_setter = bytes_format.setter();
    let raw_value_setter = raw_value.setter();
    use_effect_with_deps(
        move |format| {
            log::debug!("change format");
            format_setter.set(**format);

            raw_value_setter.set(encode_bytes(bytes, **format));
        },
        bytes_format.clone(),
    );

    use_effect_with_deps(
        move |props| {
            log::debug!("changed props");
        },
        props.clone(),
    );

    let setter = setter.clone();
    let raw_value_setter = raw_value.setter();
    let notifications = use_notification::<Notification>();
    let format = *bytes_format;
    let oninput = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let value = input.value();

        match parse_bytes(&value, format) {
            Ok(bytes) => setter.emit(bytes),
            Err(error) => notifications.spawn(Notification::new(
                NotificationType::Error,
                "Can not parse input",
                error.to_string(),
            )),
        }

        // raw_value_setter.set(value);
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

pub fn build_byte_input(bytes: Vec<u8>, setter: Callback<Vec<u8>>) -> Html {
    html! {
        <ByteInput {bytes} {setter} />
    }
}
