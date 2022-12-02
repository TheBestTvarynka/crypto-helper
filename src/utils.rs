use time::OffsetDateTime;

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

pub fn gen_copy_func(hex_data: &str) -> String {
    format!("navigator.clipboard.writeText('{}');", hex_data)
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
