use js_sys::Function;
use time::OffsetDateTime;
use wasm_bindgen::JsValue;
use web_sys::MouseEvent;
use yew::Callback;

pub fn format_date_time(datetime: &OffsetDateTime) -> String {
    format!(
        "{}:{}:{} {}.{}.{}",
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
        datetime.day(),
        datetime.month(),
        datetime.year()
    )
}

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
