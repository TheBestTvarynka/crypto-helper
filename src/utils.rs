use base64::Engine;
use base64::engine::GeneralPurpose;
use yew::Callback;
use yew_hooks::UseClipboardHandle;
use yew_notifications::{Notification, NotificationType, NotificationsManager};

pub fn copy_to_clipboard_with_notification<T>(
    data_to_copy: String,
    clipboard: UseClipboardHandle,
    data_name: impl Into<String>,
    notifications: NotificationsManager<Notification>,
) -> Callback<T> {
    let data_name = data_name.into();

    Callback::from(move |_| {
        clipboard.write_text(data_to_copy.clone());

        notifications.spawn(Notification::from_description_and_type(
            NotificationType::Info,
            format!("{} copied.", data_name),
        ));
    })
}

pub fn decode_base64(input: &str) -> Result<Vec<u8>, String> {
    let engine = GeneralPurpose::new(&base64::alphabet::STANDARD, base64::engine::general_purpose::NO_PAD);

    if input.contains('_') || input.contains('-') {
        let input = input
            .chars()
            .map(|c| match c {
                '-' => '+',
                '_' => '/',
                c => c,
            })
            .collect::<String>();
        engine.decode(input).map_err(|err| format!("invalid base64: {:?}", err))
    } else {
        engine.decode(input).map_err(|err| format!("invalid base64: {:?}", err))
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
    let binary_str = input.chars().filter(|c| *c == '0' || *c == '1').collect::<String>();

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
