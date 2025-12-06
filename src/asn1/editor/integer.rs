use base64::Engine;
use rsa::BigUint;
use web_sys::HtmlInputElement;
use yew::{
    Callback, Html, MouseEvent, Properties, TargetCast, UseStateSetter, function_component, html, use_effect_with,
    use_state,
};

use crate::common::{BASE64, HEX, get_format_button_class};
use crate::utils::{decode_base64, decode_decimal};

const DECIMAL_BYTES: &str = "decimal bytes";
const DECIMAL_INTEGER: &str = "decimal value";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IntegerFormat {
    Hex,
    Base64,
    DecimalBytes,
    DecimalInteger,
}

impl AsRef<str> for IntegerFormat {
    fn as_ref(&self) -> &str {
        match self {
            IntegerFormat::Hex => HEX,
            IntegerFormat::Base64 => BASE64,
            IntegerFormat::DecimalBytes => DECIMAL_BYTES,
            IntegerFormat::DecimalInteger => DECIMAL_INTEGER,
        }
    }
}

impl From<IntegerFormat> for &str {
    fn from(format: IntegerFormat) -> Self {
        match format {
            IntegerFormat::Hex => HEX,
            IntegerFormat::Base64 => BASE64,
            IntegerFormat::DecimalBytes => DECIMAL_BYTES,
            IntegerFormat::DecimalInteger => DECIMAL_INTEGER,
        }
    }
}

pub const INTEGER_FORMATS: &'static [IntegerFormat] = &[
    IntegerFormat::Hex,
    IntegerFormat::Base64,
    IntegerFormat::DecimalBytes,
    IntegerFormat::DecimalInteger,
];

pub const BYTES_FORMATS: &'static [IntegerFormat] = &[IntegerFormat::Hex, IntegerFormat::Base64];

#[derive(PartialEq, Properties, Clone)]
pub struct IntegerEditorProps {
    pub value: Vec<u8>,
    pub setter: Callback<Vec<u8>>,
    pub formats: &'static [IntegerFormat],
}

#[function_component(IntegerEditor)]
pub fn integer_editor(props: &IntegerEditorProps) -> Html {
    let integer_format = use_state(|| IntegerFormat::Hex);
    let encoded = use_state(|| encode_bytes(&props.value, *integer_format));

    let setter = props.setter.clone();
    let format = *integer_format;
    let oninput = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let value = input.value();

        if let Ok(integer) = parse_integer(&value, format) {
            setter.emit(integer);
        }
    });

    let format_setter = integer_format.setter();
    let encoded_setter = encoded.setter();
    let parsed_bytes = props.value.clone();
    use_effect_with(integer_format.clone(), move |format| {
        format_setter.set(**format);
        encoded_setter.set(encode_bytes(parsed_bytes, **format));
    });

    let encoded_setter = encoded.setter();
    let format_value = *integer_format;
    use_effect_with(props.clone(), move |props| {
        let bytes = props.value.clone();
        let raw = encode_bytes(&bytes, format_value);

        encoded_setter.set(raw);
    });

    html! {
        <div class="vertical">
            <div class="formats-container">{
                props.formats.iter().map(|format| {
                    html! {
                        <button
                            class={get_format_button_class(*integer_format == *format)}
                            onclick={get_set_format_callback(*format, integer_format.setter())}
                        >
                            {<&str>::from(*format)}
                        </button>
                    }
                }).collect::<Html>()
            }</div>
            <textarea
                class={"modal-input"}
                value={(*encoded).clone()}
                {oninput}
            />
        </div>
    }
}

fn get_set_format_callback(format: IntegerFormat, set_format: UseStateSetter<IntegerFormat>) -> Callback<MouseEvent> {
    Callback::from(move |_event| {
        set_format.set(format);
    })
}

fn encode_bytes(integer: impl AsRef<[u8]>, format: IntegerFormat) -> String {
    match format {
        IntegerFormat::Hex => hex::encode(integer),
        IntegerFormat::Base64 => base64::engine::general_purpose::STANDARD.encode(integer),
        IntegerFormat::DecimalBytes => integer
            .as_ref()
            .iter()
            .map(|byte| byte.to_string())
            .collect::<Vec<String>>()
            .join(" "),
        IntegerFormat::DecimalInteger => {
            let bytes = integer.as_ref();

            BigUint::from_bytes_be(if bytes.len() > 1 {
                if bytes[0] == 0x00 { &bytes[1..] } else { bytes }
            } else if bytes.is_empty() {
                &[0]
            } else {
                bytes
            })
            .to_string()
        }
    }
}

fn parse_integer(raw: &str, format: IntegerFormat) -> Result<Vec<u8>, String> {
    match format {
        IntegerFormat::Hex => {
            let raw = raw
                .to_ascii_lowercase()
                .chars()
                .filter(|c| c.is_ascii_digit() || ('a'..='f').contains(c))
                .collect::<String>();
            hex::decode(raw).map_err(|err| format!("invalid hex input: {:?}", err))
        }
        IntegerFormat::Base64 => {
            let raw = raw
                .chars()
                .filter(|c| {
                    c.is_ascii_lowercase()
                        || c.is_ascii_uppercase()
                        || c.is_ascii_digit()
                        || *c == '+'
                        || *c == '/'
                        || *c == '='
                        // url-encoded base64:
                        || *c == '-'
                        || *c == '_'
                })
                .collect::<String>();
            decode_base64(&raw)
        }
        IntegerFormat::DecimalBytes => decode_decimal(raw),
        IntegerFormat::DecimalInteger => BigUint::parse_bytes(raw.as_bytes(), 10)
            .ok_or_else(|| "invalid decimal integer input".to_string())
            .map(|integer| integer.to_bytes_be()),
    }
}
