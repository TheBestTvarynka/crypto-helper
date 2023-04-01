#[macro_export]
macro_rules! check_symmetric_key {
    (key: $key:expr, len_hint: $len_hint:expr, name: $name:expr, notificator: $notificator:expr) => {{
        let key = match hex::decode($key) {
            Ok(key) => key,
            Err(error) => {
                log::error!("invalid {} key: {}", $name, $key);
                $notificator.emit(Notification::new(
                    NotificationType::Error,
                    format!("Invalid {} key", $name),
                    format!("{:?}", error),
                ));

                return Default::default();
            }
        };

        if key.is_empty() {
            $notificator.emit(Notification::from_description_and_type(
                NotificationType::Error,
                "Input key is empty.",
            ));

            return Default::default();
        }

        if let Some(key_len) = $len_hint {
            if key_len > key.len() {
                $notificator.emit(Notification::from_description_and_type(
                    NotificationType::Warn,
                    format!(
                        "Input key is too short. Got {}, but expected at least {}.",
                        key.len(),
                        key_len
                    ),
                ));
            }
        }

        key
    }};
}
