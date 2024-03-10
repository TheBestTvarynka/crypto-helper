mod byte_input;
mod bytes_viewer;
mod checkbox;
mod rc_slice;
mod simple_output;
mod switch;
mod table;

pub use byte_input::{build_byte_input, ByteInput};
pub use checkbox::Checkbox;
pub use rc_slice::RcSlice;
pub use simple_output::build_simple_output;
pub use switch::Switch;
pub use table::TableView;
use web_sys::MouseEvent;
use yew::{Callback, UseStateSetter};

use crate::utils::{decode_base64, decode_binary, decode_decimal};

const HEX: &str = "hex";
const BASE64: &str = "base64";
const ASCII: &str = "ascii";
const DECIMAL: &str = "decimal";
const BINARY: &str = "binary";

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
            BytesFormat::Hex => HEX,
            BytesFormat::Base64 => BASE64,
            BytesFormat::Ascii => ASCII,
            BytesFormat::Decimal => DECIMAL,
            BytesFormat::Binary => BINARY,
        }
    }
}

impl From<&BytesFormat> for &str {
    fn from(format: &BytesFormat) -> Self {
        match format {
            BytesFormat::Hex => HEX,
            BytesFormat::Base64 => BASE64,
            BytesFormat::Ascii => ASCII,
            BytesFormat::Decimal => DECIMAL,
            BytesFormat::Binary => BINARY,
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

pub fn encode_bytes(bytes: impl AsRef<[u8]>, format: BytesFormat) -> String {
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
        BytesFormat::Hex => {
            let raw = raw
                .to_ascii_lowercase()
                .chars()
                .filter(|c| c.is_ascii_digit() || ('a'..='f').contains(c))
                .collect::<String>();
            hex::decode(raw).map_err(|err| format!("invalid hex input: {:?}", err))
        }
        BytesFormat::Base64 => {
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
        BytesFormat::Ascii => Ok(raw.into()),
        BytesFormat::Decimal => decode_decimal(raw),
        BytesFormat::Binary => decode_binary(raw),
    }
}

fn get_format_button_class(selected: bool) -> &'static str {
    if selected {
        "format-button format-button-selected"
    } else {
        "format-button"
    }
}

fn get_set_format_callback(format: BytesFormat, set_format: UseStateSetter<BytesFormat>) -> Callback<MouseEvent> {
    Callback::from(move |_event| {
        set_format.set(format);
    })
}

static BYTE_HEX_STR_ARRAY: [&'static str; 256] = [
    "00",
    "01",
    "02",
    "03",
    "04",
    "05",
    "06",
    "07",
    "08",
    "09",
    "0a",
    "0b",
    "0c",
    "0d",
    "0e",
    "0f",
    "10",
    "11",
    "12",
    "13",
    "14",
    "15",
    "16",
    "17",
    "18",
    "19",
    "1a",
    "1b",
    "1c",
    "1d",
    "1e",
    "1f",
    "20",
    "21",
    "22",
    "23",
    "24",
    "25",
    "26",
    "27",
    "28",
    "29",
    "2a",
    "2b",
    "2c",
    "2d",
    "2e",
    "2f",
    "30",
    "31",
    "32",
    "33",
    "34",
    "35",
    "36",
    "37",
    "38",
    "39",
    "3a",
    "3b",
    "3c",
    "3d",
    "3e",
    "3f",
    "40",
    "41",
    "42",
    "43",
    "44",
    "45",
    "46",
    "47",
    "48",
    "49",
    "4a",
    "4b",
    "4c",
    "4d",
    "4e",
    "4f",
    "50",
    "51",
    "52",
    "53",
    "54",
    "55",
    "56",
    "57",
    "58",
    "59",
    "5a",
    "5b",
    "5c",
    "5d",
    "5e",
    "5f",
    "60",
    "61",
    "62",
    "63",
    "64",
    "65",
    "66",
    "67",
    "68",
    "69",
    "6a",
    "6b",
    "6c",
    "6d",
    "6e",
    "6f",
    "70",
    "71",
    "72",
    "73",
    "74",
    "75",
    "76",
    "77",
    "78",
    "79",
    "7a",
    "7b",
    "7c",
    "7d",
    "7e",
    "7f",
    "80",
    "81",
    "82",
    "83",
    "84",
    "85",
    "86",
    "87",
    "88",
    "89",
    "8a",
    "8b",
    "8c",
    "8d",
    "8e",
    "8f",
    "90",
    "91",
    "92",
    "93",
    "94",
    "95",
    "96",
    "97",
    "98",
    "99",
    "9a",
    "9b",
    "9c",
    "9d",
    "9e",
    "9f",
    "a0",
    "a1",
    "a2",
    "a3",
    "a4",
    "a5",
    "a6",
    "a7",
    "a8",
    "a9",
    "aa",
    "ab",
    "ac",
    "ad",
    "ae",
    "af",
    "b0",
    "b1",
    "b2",
    "b3",
    "b4",
    "b5",
    "b6",
    "b7",
    "b8",
    "b9",
    "ba",
    "bb",
    "bc",
    "bd",
    "be",
    "bf",
    "c0",
    "c1",
    "c2",
    "c3",
    "c4",
    "c5",
    "c6",
    "c7",
    "c8",
    "c9",
    "ca",
    "cb",
    "cc",
    "cd",
    "ce",
    "cf",
    "d0",
    "d1",
    "d2",
    "d3",
    "d4",
    "d5",
    "d6",
    "d7",
    "d8",
    "d9",
    "da",
    "db",
    "dc",
    "dd",
    "de",
    "df",
    "e0",
    "e1",
    "e2",
    "e3",
    "e4",
    "e5",
    "e6",
    "e7",
    "e8",
    "e9",
    "ea",
    "eb",
    "ec",
    "ed",
    "ee",
    "ef",
    "f0",
    "f1",
    "f2",
    "f3",
    "f4",
    "f5",
    "f6",
    "f7",
    "f8",
    "f9",
    "fa",
    "fb",
    "fc",
    "fd",
    "fe",
    "ff",
];

// oPtImIzAtIoN
pub fn hex_format_byte(byte: u8) -> &'static str {
    BYTE_HEX_STR_ARRAY[byte as usize]
}
