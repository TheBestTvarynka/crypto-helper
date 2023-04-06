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
macro_rules! check_public_key {
    (public_key: $public_key:expr, name: $name:expr, notificator: $notificator:expr) => {{
        let public_key = match PublicKey::from_pem_str($public_key) {
                Ok(key) => key,
                Err(error) => {
                    log::error!("invalid public {} key", $name);
                    $notificator.emit(Notification::new(
                        NotificationType::Error,
                        format!("Invalid public {} key", $name),
                        format!("{:?}", error),
                    ));

                    return None;
                }
        };

        public_key
    }};
}

#[macro_export]
macro_rules! check_private_key {
    (private_key: $private_key:expr, name: $name:expr, notificator: $notificator:expr) => {{
        let private_key = match PrivateKey::from_pem_str($private_key) {
                Ok(key) => key,
                Err(error) => {
                    log::error!("invalid private {} key", $name);
                    $notificator.emit(Notification::new(
                        NotificationType::Error,
                        format!("Invalid private {} key", $name),
                        format!("{:?}", error),
                    ));

                    return None;
                }
        };

        private_key
    }};
}

#[macro_export]
macro_rules! verify_signature {
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
macro_rules! sign_signature {
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