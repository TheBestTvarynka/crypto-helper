use std::str::FromStr;

use crate::utils::decode_base64;

use super::jwt::Jwt;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Jwe {}

#[derive(Debug, PartialEq, Eq)]
pub enum Jwte {
    Jwt(Jwt),
    #[allow(dead_code)]
    Jwe(Jwe),
}

impl FromStr for Jwte {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');

        let raw_header = parts
            .next()
            .ok_or_else(|| "JWT Header is not present".to_owned())?
            .to_owned();
        let parsed_header = String::from_utf8(decode_base64(&raw_header)?).unwrap();
        let raw_payload = parts
            .next()
            .ok_or_else(|| "JWT Payload is not present".to_owned())?
            .to_owned();
        let parsed_payload = String::from_utf8(decode_base64(&raw_payload)?).unwrap();
        let raw_signature = parts
            .next()
            .ok_or_else(|| "JWT Signature is not present".to_owned())?
            .to_owned();
        let parsed_signature = hex::encode(decode_base64(&raw_signature)?);

        if parts.next().is_some() {
            return Err("Too many dots in the JWT".into());
        }

        Ok(Jwte::Jwt(Jwt {
            raw_header,
            raw_payload,
            raw_signature,
            parsed_header,
            parsed_payload,
            parsed_signature,
        }))
    }
}

impl Default for Jwte {
    fn default() -> Self {
        Jwte::Jwt(Jwt::default())
    }
}
