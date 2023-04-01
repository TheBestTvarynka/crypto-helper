use std::str::FromStr;

use serde_json::Value;

use super::jwt::Jwt;
use super::signature::JwtSignatureAlgorithm;
use crate::utils::decode_base64;

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
        let signature = decode_base64(&raw_signature)?;
        let parsed_signature = hex::encode(&signature);

        let header: Result<Value, _> = serde_json::from_str(&parsed_header);
        let signature_algorithm = header
            .map(|header| {
                let algo: JwtSignatureAlgorithm = header
                    .get("alg")
                    .map(|algo| algo.try_into().unwrap_or_default())
                    .unwrap_or_default();
                algo
            })
            .unwrap_or_default();

        if parts.next().is_some() {
            return Err("Too many dots in the JWT".into());
        }

        Ok(Jwte::Jwt(Jwt {
            raw_header,
            parsed_header,

            raw_payload,
            parsed_payload,

            raw_signature,
            parsed_signature,
            signature,
            signature_algorithm,
        }))
    }
}

impl Default for Jwte {
    fn default() -> Self {
        Jwte::Jwt(Jwt::default())
    }
}
