use serde::{Deserialize, Serialize};
use web_sys::window;

use crate::crypto_helper::Algorithm;
use crate::serde::{deserialize_bytes, serialize_bytes};

fn origin() -> String {
    window()
        .expect("windows object must present")
        .location()
        .origin()
        .expect("page must have the origin string")
}

pub fn generate_crypto_helper_link(algorithm: &Algorithm) -> String {
    let mut link = origin();

    link.push_str("/crypto-helper/?");
    link.push_str(&serde_qs::to_string(algorithm).unwrap());

    link
}

#[derive(Serialize, Deserialize)]
pub struct Jwt {
    pub jwt: String,
}

pub fn generate_jwt_link(jwt: String) -> String {
    let mut link = origin();

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
    let mut link = origin();

    link.push_str("/asn1/?");
    link.push_str(&serde_qs::to_string(&Asn1 { asn1 }).unwrap());

    link
}
