use js_sys::Function;
use wasm_bindgen::JsValue;
use web_sys::MouseEvent;
use yew::Callback;

pub fn gen_copy_func(data: &str) -> String {
    let data = data
        .chars()
        .map(|c| {
            if c == '\n' {
                "\\n".to_owned()
            } else if c == '\\' {
                "\\\\".to_owned()
            } else {
                c.to_string()
            }
        })
        .collect::<String>();
    format!("navigator.clipboard.writeText('{}');", data)
}

pub fn gen_copy_onclick(data: String) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        log::debug!("data to copy: `{}`", data);
        let function = Function::new_no_args(&gen_copy_func(&data));
        if let Err(err) = function.call0(&JsValue::null()) {
            log::error!("error oncopy: {:?}", err);
        }
    })
}

pub fn decode_base64(input: &str) -> Result<Vec<u8>, String> {
    if input.contains('_') || input.contains('-') {
        let input = input
            .chars()
            .map(|c| match c {
                '-' => '+',
                '_' => '/',
                c => c,
            })
            .collect::<String>();
        base64::decode(input).map_err(|err| format!("invalid base64: {:?}", err))
    } else {
        base64::decode(input).map_err(|err| format!("invalid base64: {:?}", err))
    }
}

pub fn decode_decimal(input: &str) -> Result<Vec<u8>, String> {
    input
        .chars()
        .filter(|c| c.is_ascii_digit() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .map(|s| {
            s.parse::<u8>()
                .map_err(|err| format!("invalid decimal input: {:?}", err))
        })
        .collect::<Result<Vec<u8>, String>>()
}

pub fn decode_binary(input: &str) -> Result<Vec<u8>, String> {
    let binary_str = input.chars().filter(|c| c == &'0' || c == &'1').collect::<String>();

    if binary_str.len() % 8 != 0 {
        return Err("invalid binary input: not a multiple of 8 bits".to_string());
    }

    binary_str
        .as_str()
        .chars()
        .collect::<Vec<char>>()
        .chunks(8)
        .map(|chunk| {
            let s: String = chunk.iter().collect();
            u8::from_str_radix(&s, 2).map_err(|err| format!("invalid binary input: {:?}", err))
        })
        .collect::<Result<Vec<u8>, String>>()
}
