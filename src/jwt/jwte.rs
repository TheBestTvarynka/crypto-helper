use std::str::FromStr;

use serde_json::Value;

use super::jwt::Jwt;
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
        let header: Value =
            serde_json::from_str(&parsed_header).map_err(|err| format!("invalid header json: {:?}", err))?;

        let raw_payload = parts
            .next()
            .ok_or_else(|| "JWT Payload is not present".to_owned())?
            .to_owned();
        let parsed_payload = String::from_utf8(decode_base64(&raw_payload)?).unwrap();
        let payload =
            serde_json::from_str(&parsed_payload).map_err(|err| format!("invalid payload json: {:?}", err))?;

        let raw_signature = parts
            .next()
            .ok_or_else(|| "JWT Signature is not present".to_owned())?
            .to_owned();
        let signature = decode_base64(&raw_signature)?;
        let parsed_signature = hex::encode(&signature);
        let signature_algorithm = header
            .get("alg")
            .ok_or_else(|| "alg is not present in the JWT header".to_owned())?
            .try_into()?;

        if parts.next().is_some() {
            return Err("Too many dots in the JWT".into());
        }

        Ok(Jwte::Jwt(Jwt {
            raw_header,
            parsed_header,
            header,

            raw_payload,
            parsed_payload,
            payload,

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
