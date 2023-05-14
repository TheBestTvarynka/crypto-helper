use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, use_effect_with_deps, use_state, Callback, Html, Properties, TargetCast};

use super::BytesFormat;
use crate::common::{encode_bytes, get_format_button_class, get_set_format_callback, parse_bytes, BYTES_FORMATS};

#[derive(PartialEq, Properties, Clone)]
pub struct ByteInputProps {
    #[prop_or(BytesFormat::Hex)]
    format: BytesFormat,
    #[prop_or_default]
    placeholder: String,
    bytes: Vec<u8>,
    setter: Callback<Vec<u8>>,
}

#[function_component(ByteInput)]
pub fn byte_input(props: &ByteInputProps) -> Html {
    let ByteInputProps {
        format,
        bytes,
        setter,
        placeholder,
    } = &props;

    let raw_value = use_state(|| encode_bytes(bytes, *format));
    let bytes = use_state(|| bytes.clone());
    let bytes_format = use_state(|| *format);
    let is_valid = use_state(|| true);

    let format_setter = bytes_format.setter();
    let raw_value_setter = raw_value.setter();
    let parsed_bytes = (*bytes).clone();
    use_effect_with_deps(
        move |format| {
            format_setter.set(**format);
            raw_value_setter.set(encode_bytes(parsed_bytes, **format));
        },
        bytes_format.clone(),
    );

    let bytes_setter = bytes.setter();
    let raw_value_setter = raw_value.setter();
    let format_value = *bytes_format;
    use_effect_with_deps(
        move |props| {
            let bytes = props.bytes.clone();
            let raw = encode_bytes(&bytes, format_value);

            bytes_setter.set(bytes);
            raw_value_setter.set(raw);
        },
        props.clone(),
    );

    let setter = setter.clone();
    let raw_value_setter = raw_value.setter();
    let format = *bytes_format;
    let set_is_valid = is_valid.setter();
    let oninput = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let value = input.value();

        match parse_bytes(&value, format) {
            Ok(bytes) => {
                setter.emit(bytes);
                set_is_valid.set(true);
            }
            Err(_) => {
                set_is_valid.set(false);
            }
        }

        raw_value_setter.set(value);
    });

    html! {
        <div class={classes!("bytes-input", "vertical")}>
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
                placeholder={format!("{}: place {} encoded input here", placeholder, (*bytes_format).as_ref())}
                class={classes!("base-input", if !(*is_valid) { "input-error" } else { "" })}
                value={(*raw_value).clone()}
                {oninput}
            />
            <span class={classes!("total")}>{format!("total: {}", (*bytes).len())}</span>
        </div>
    }
}

pub fn build_byte_input(
    bytes: Vec<u8>,
    setter: Callback<Vec<u8>>,
    format: Option<BytesFormat>,
    placeholder: Option<String>,
) -> Html {
    html! {
        <ByteInput {bytes} {setter} format={format.unwrap_or_default()} placeholder={placeholder.unwrap_or_default()} />
    }
}
