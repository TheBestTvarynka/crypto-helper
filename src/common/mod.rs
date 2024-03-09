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

// oPtImIzAtIoN
pub fn hex_format_byte(byte: u8) -> &'static str {
    match byte {
        0 => "00",
        1 => "01",
        2 => "02",
        3 => "03",
        4 => "04",
        5 => "05",
        6 => "06",
        7 => "07",
        8 => "08",
        9 => "09",
        10 => "0a",
        11 => "0b",
        12 => "0c",
        13 => "0d",
        14 => "0e",
        15 => "0f",
        16 => "10",
        17 => "11",
        18 => "12",
        19 => "13",
        20 => "14",
        21 => "15",
        22 => "16",
        23 => "17",
        24 => "18",
        25 => "19",
        26 => "1a",
        27 => "1b",
        28 => "1c",
        29 => "1d",
        30 => "1e",
        31 => "1f",
        32 => "20",
        33 => "21",
        34 => "22",
        35 => "23",
        36 => "24",
        37 => "25",
        38 => "26",
        39 => "27",
        40 => "28",
        41 => "29",
        42 => "2a",
        43 => "2b",
        44 => "2c",
        45 => "2d",
        46 => "2e",
        47 => "2f",
        48 => "30",
        49 => "31",
        50 => "32",
        51 => "33",
        52 => "34",
        53 => "35",
        54 => "36",
        55 => "37",
        56 => "38",
        57 => "39",
        58 => "3a",
        59 => "3b",
        60 => "3c",
        61 => "3d",
        62 => "3e",
        63 => "3f",
        64 => "40",
        65 => "41",
        66 => "42",
        67 => "43",
        68 => "44",
        69 => "45",
        70 => "46",
        71 => "47",
        72 => "48",
        73 => "49",
        74 => "4a",
        75 => "4b",
        76 => "4c",
        77 => "4d",
        78 => "4e",
        79 => "4f",
        80 => "50",
        81 => "51",
        82 => "52",
        83 => "53",
        84 => "54",
        85 => "55",
        86 => "56",
        87 => "57",
        88 => "58",
        89 => "59",
        90 => "5a",
        91 => "5b",
        92 => "5c",
        93 => "5d",
        94 => "5e",
        95 => "5f",
        96 => "60",
        97 => "61",
        98 => "62",
        99 => "63",
        100 => "64",
        101 => "65",
        102 => "66",
        103 => "67",
        104 => "68",
        105 => "69",
        106 => "6a",
        107 => "6b",
        108 => "6c",
        109 => "6d",
        110 => "6e",
        111 => "6f",
        112 => "70",
        113 => "71",
        114 => "72",
        115 => "73",
        116 => "74",
        117 => "75",
        118 => "76",
        119 => "77",
        120 => "78",
        121 => "79",
        122 => "7a",
        123 => "7b",
        124 => "7c",
        125 => "7d",
        126 => "7e",
        127 => "7f",
        128 => "80",
        129 => "81",
        130 => "82",
        131 => "83",
        132 => "84",
        133 => "85",
        134 => "86",
        135 => "87",
        136 => "88",
        137 => "89",
        138 => "8a",
        139 => "8b",
        140 => "8c",
        141 => "8d",
        142 => "8e",
        143 => "8f",
        144 => "90",
        145 => "91",
        146 => "92",
        147 => "93",
        148 => "94",
        149 => "95",
        150 => "96",
        151 => "97",
        152 => "98",
        153 => "99",
        154 => "9a",
        155 => "9b",
        156 => "9c",
        157 => "9d",
        158 => "9e",
        159 => "9f",
        160 => "a0",
        161 => "a1",
        162 => "a2",
        163 => "a3",
        164 => "a4",
        165 => "a5",
        166 => "a6",
        167 => "a7",
        168 => "a8",
        169 => "a9",
        170 => "aa",
        171 => "ab",
        172 => "ac",
        173 => "ad",
        174 => "ae",
        175 => "af",
        176 => "b0",
        177 => "b1",
        178 => "b2",
        179 => "b3",
        180 => "b4",
        181 => "b5",
        182 => "b6",
        183 => "b7",
        184 => "b8",
        185 => "b9",
        186 => "ba",
        187 => "bb",
        188 => "bc",
        189 => "bd",
        190 => "be",
        191 => "bf",
        192 => "c0",
        193 => "c1",
        194 => "c2",
        195 => "c3",
        196 => "c4",
        197 => "c5",
        198 => "c6",
        199 => "c7",
        200 => "c8",
        201 => "c9",
        202 => "ca",
        203 => "cb",
        204 => "cc",
        205 => "cd",
        206 => "ce",
        207 => "cf",
        208 => "d0",
        209 => "d1",
        210 => "d2",
        211 => "d3",
        212 => "d4",
        213 => "d5",
        214 => "d6",
        215 => "d7",
        216 => "d8",
        217 => "d9",
        218 => "da",
        219 => "db",
        220 => "dc",
        221 => "dd",
        222 => "de",
        223 => "df",
        224 => "e0",
        225 => "e1",
        226 => "e2",
        227 => "e3",
        228 => "e4",
        229 => "e5",
        230 => "e6",
        231 => "e7",
        232 => "e8",
        233 => "e9",
        234 => "ea",
        235 => "eb",
        236 => "ec",
        237 => "ed",
        238 => "ee",
        239 => "ef",
        240 => "f0",
        241 => "f1",
        242 => "f2",
        243 => "f3",
        244 => "f4",
        245 => "f5",
        246 => "f6",
        247 => "f7",
        248 => "f8",
        249 => "f9",
        250 => "fa",
        251 => "fb",
        252 => "fc",
        253 => "fd",
        254 => "fe",
        255 => "ff",
    }
}
