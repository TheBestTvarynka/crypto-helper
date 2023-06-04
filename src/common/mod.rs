mod byte_input;
mod bytes_viewer;
mod checkbox;
mod simple_output;
mod switch;

pub use byte_input::{build_byte_input, ByteInput, ByteInputProps};
pub use bytes_viewer::{BytesViewer, BytesViewerProps};
pub use checkbox::{Checkbox, CheckboxProps};
pub use simple_output::build_simple_output;
pub use switch::{Switch, SwitchProps};
use web_sys::MouseEvent;
use yew::{classes, Callback, Classes, UseStateSetter};

use crate::utils::{decode_binary, decode_decimal};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum BytesFormat {
    #[default]
    Hex,
    Base64,
    Ascii,
    Decimal,
    Binary,
}

impl AsRef<str> for BytesFormat {
    fn as_ref(&self) -> &str {
        match self {
            BytesFormat::Hex => "hex",
            BytesFormat::Base64 => "base64",
            BytesFormat::Ascii => "ascii",
            BytesFormat::Decimal => "decimal",
            BytesFormat::Binary => "binary",
        }
    }
}

impl From<&BytesFormat> for &str {
    fn from(format: &BytesFormat) -> Self {
        match format {
            BytesFormat::Hex => "hex",
            BytesFormat::Base64 => "base64",
            BytesFormat::Ascii => "ascii",
            BytesFormat::Decimal => "decimal",
            BytesFormat::Binary => "binary",
        }
    }
}

pub const BYTES_FORMATS: [BytesFormat; 5] = [
    BytesFormat::Hex,
    BytesFormat::Base64,
    BytesFormat::Ascii,
    BytesFormat::Decimal,
    BytesFormat::Binary,
];

fn encode_bytes(bytes: impl AsRef<[u8]>, format: BytesFormat) -> String {
    match format {
        BytesFormat::Hex => hex::encode(bytes),
        BytesFormat::Base64 => base64::encode(bytes),
        BytesFormat::Ascii => bytes.as_ref().iter().map(|c| *c as char).collect(),
        BytesFormat::Decimal => bytes
            .as_ref()
            .iter()
            .map(|byte| byte.to_string())
            .collect::<Vec<String>>()
            .join(" "),
        BytesFormat::Binary => bytes
            .as_ref()
            .iter()
            .map(|byte| format!("{:08b}", byte))
            .collect::<Vec<String>>()
            .join(" "),
    }
}

fn parse_bytes(raw: &str, format: BytesFormat) -> Result<Vec<u8>, String> {
    match format {
        BytesFormat::Hex => hex::decode(raw).map_err(|err| format!("invalid hex input:{:?}", err)),
        BytesFormat::Base64 => base64::decode(raw).map_err(|err| format!("invalid base64 input:{:?}", err)),
        BytesFormat::Ascii => Ok(raw.into()),
        BytesFormat::Decimal => decode_decimal(raw),
        BytesFormat::Binary => decode_binary(raw),
    }
}

fn get_format_button_class(selected: bool) -> Classes {
    if selected {
        classes!("format-button", "format-button-selected")
    } else {
        classes!("format-button")
    }
}

fn get_set_format_callback(format: BytesFormat, set_format: UseStateSetter<BytesFormat>) -> Callback<MouseEvent> {
    Callback::from(move |_event| {
        set_format.set(format);
    })
}
