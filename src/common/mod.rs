mod byte_input;
mod bytes_viewer;
mod simple_input;
mod simple_output;
mod switch;

pub use byte_input::{build_byte_input, ByteInput, ByteInputProps};
pub use bytes_viewer::{BytesViewer, BytesViewerProps};
pub use simple_input::{build_simple_input, SimpleInput, SimpleInputProps};
pub use simple_output::build_simple_output;
pub use switch::{Switch, SwitchProps};
use web_sys::MouseEvent;
use yew::{classes, Callback, Classes, UseStateSetter};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BytesFormat {
    Hex,
    Base64,
    Ascii,
}

impl From<&BytesFormat> for &str {
    fn from(format: &BytesFormat) -> Self {
        match format {
            BytesFormat::Hex => "hex",
            BytesFormat::Base64 => "base64",
            BytesFormat::Ascii => "ascii",
        }
    }
}

pub const BYTES_FORMATS: [BytesFormat; 3] = [BytesFormat::Hex, BytesFormat::Base64, BytesFormat::Ascii];

fn encode_bytes(bytes: &[u8], format: BytesFormat) -> String {
    match format {
        BytesFormat::Hex => hex::encode(bytes),
        BytesFormat::Base64 => base64::encode(bytes),
        BytesFormat::Ascii => bytes.iter().map(|c| *c as char).collect(),
    }
}

fn parse_bytes(raw: &str, format: BytesFormat) -> Result<Vec<u8>, String> {
    match format {
        BytesFormat::Hex => hex::decode(raw).map_err(|err| format!("invalid hex input:{:?}", err)),
        BytesFormat::Base64 => base64::decode(raw).map_err(|err| format!("invalid base64 input:{:?}", err)),
        BytesFormat::Ascii => Ok(raw.into()),
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
