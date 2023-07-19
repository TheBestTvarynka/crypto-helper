pub mod ui;

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

pub fn parse_bytes(raw: &str, format: BytesFormat) -> Result<Vec<u8>, String> {
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
