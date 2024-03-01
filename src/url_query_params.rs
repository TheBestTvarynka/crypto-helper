use serde::{Deserialize, Serialize};

use crate::crypto_helper::Algorithm;
use crate::serde::{deserialize_bytes, serialize_bytes};

const APP_HOST: &str = env!("APP_HOST");

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

#[derive(Serialize, Deserialize)]
pub struct Asn1 {
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    pub asn1: Vec<u8>,
}

pub fn generate_asn1_link(asn1: Vec<u8>) -> String {
    let mut link = APP_HOST.to_string();

    link.push_str("/asn1/?");
    link.push_str(&serde_qs::to_string(&Asn1 { asn1 }).unwrap());

    link
}
