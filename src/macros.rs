macro_rules! verify_hmac {
    (
        hash_alg: $hash_alg:ty,
        key: $key:expr,
        msg: $msg:expr,
        digest: $digest:expr,
    ) => {{
        use hmac::{Hmac, Mac};

        let mut mac = Hmac::<$hash_alg>::new_from_slice($key).expect("hmac key length should be checked");
        mac.update($msg);
        mac.verify($digest.into()).is_ok()
    }};
}

macro_rules! sign_hmac {
    (
        hash_alg: $hash_alg:ty,
        key: $key:expr,
        msg: $msg:expr,
    ) => {{
        use hmac::{Hmac, Mac};

        let mut mac = Hmac::<$hash_alg>::new_from_slice($key).expect("hmac key length should be checked");
        mac.update($msg);
        mac.finalize().into_bytes().to_vec()
    }};
}
