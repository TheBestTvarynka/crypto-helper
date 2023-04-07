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

#[macro_export]
macro_rules! check_asymmetric_key {
    (
        key: $key:expr,
        name: $name:expr,
        notificator: $notificator:expr,
        key_kind: $key_kind:ty,
    ) => {{
        let rsa_key = match <$key_kind>::from_pem_str($key) {
                Ok(key) => key,
                Err(error) => {
                    log::error!("invalid RSA {} key", $name);
                    $notificator.emit(Notification::new(
                        NotificationType::Error,
                        format!("Invalid RSA {} key", $name),
                        format!("{:?}", error),
                    ));

                    return None;
                }
        };

        rsa_key
    }};
}

#[macro_export]
macro_rules! verify {
    (
        signature_algo: $signature_algo:expr,
        hash_algo: $hash_algo:expr,
        public_key: $public_key:expr,
        data_to_sign: $data_to_sign:expr,
        jwt_signature: $jwt_signature:expr,
        notificator: $notificator:expr
    ) => {{
        match $signature_algo($hash_algo).verify(
                $public_key,
                $data_to_sign,
                $jwt_signature
            ) {
                Ok(_) => true,
                Err(error) => {
                    $notificator.emit(Notification::from_description_and_type(
                        NotificationType::Error,
                        error.to_string(),
                    ));
                    false
                }
        }
    }};
}

#[macro_export]
macro_rules! sign {
    (
        signature_algo: $signature_algo:expr,
        hash_algo: $hash_algo:expr,
        name: $name:expr,
        private_key: $private_key:expr,
        data_to_sign: $data_to_sign:expr,
        notificator: $notificator:expr
    ) => {{
        match $signature_algo($hash_algo).sign(
                $data_to_sign,
                $private_key
            ) {
                Ok(signature) => Some(signature),
                Err(error) => {
                    $notificator.emit(Notification::new(
                        NotificationType::Error,
                        format!("Can not generate {} signature", $name),
                        format!("{:?}", error),
                    ));
                    None
                }
        }
    }};
}