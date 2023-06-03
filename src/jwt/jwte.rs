use std::str::FromStr;

use serde_json::Value;

use super::jwt::Jwt;
use super::signature::JwtSignatureAlgorithm;
use crate::utils::decode_base64;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Jwe {}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Eq)]
pub enum Jwte {
    Jwt(Jwt),
    #[allow(dead_code)]
    Jwe(Jwe),
}

fn is_jwt_allowed_char(c: &char) -> bool {
    c.is_alphabetic() || c.is_numeric() || *c == '+' || *c == '/' || *c == '='
    // in case of the url-encoded base64 token
    || *c == '-' || *c == '_'
    // JWT parts separator
    || *c == '.'
}

impl FromStr for Jwte {
    type Err = String;

    fn from_str(token: &str) -> Result<Self, Self::Err> {
        let token = token.replace("Authorization", "");
        let token = token.replace("Bearer", "");
        let token = token.trim().chars().filter(is_jwt_allowed_char).collect::<String>();

        let mut parts = token.split('.');

        let raw_header = parts
            .next()
            .ok_or_else(|| "JWT Header is not present".to_owned())?
            .to_owned();
        let parsed_header = String::from_utf8(decode_base64(&raw_header)?)
            .map_err(|err| format!("Decoded header is not UTF-8 text: {:?}", err))?;

        let raw_payload = parts
            .next()
            .ok_or_else(|| "JWT Payload is not present".to_owned())?
            .to_owned();
        let parsed_payload = String::from_utf8(decode_base64(&raw_payload)?)
            .map_err(|err| format!("Decoded payload is not UTF-8 text: {:?}", err))?;

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

        let leftover = parts
            .next()
            .map(|part| {
                let mut part = part.to_owned();
                part.insert(0, '.');
                part
            })
            .unwrap_or_default();
        let leftover = parts.fold(leftover, |mut leftover, part| {
            leftover.push('.');
            leftover.push_str(part);
            leftover
        });

        Ok(Jwte::Jwt(Jwt {
            raw_header,
            parsed_header,

            raw_payload,
            parsed_payload,

            raw_signature,
            parsed_signature,
            signature,
            signature_algorithm,

            leftover,
        }))
    }
}

impl Default for Jwte {
    fn default() -> Self {
        Jwte::Jwt(Jwt::default())
    }
}
