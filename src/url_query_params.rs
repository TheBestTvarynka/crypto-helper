use serde::{Deserialize, Serialize};

use crate::crypto_helper::algorithm::Algorithm;
use crate::settings::APP_HOST;

pub fn generate_crypto_helper_link(algorithm: &Algorithm) -> String {
    let mut link = APP_HOST.to_string();

    link.push_str("/crypto-helper/?");
    link.push_str(&serde_qs::to_string(algorithm).unwrap());

    link
}

#[derive(Serialize, Deserialize)]
pub struct Jwt {
    pub jwt: String,
}

pub fn generate_jwt_link(jwt: String) -> String {
    let mut link = APP_HOST.to_string();

    link.push_str("/jwt/?");
    link.push_str(&serde_qs::to_string(&Jwt { jwt }).unwrap());

    link
}
