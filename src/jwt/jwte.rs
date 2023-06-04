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
        let mut start_over = String::new();

        let mut parts = token.split('.');

        let (raw_header, parsed_header, signature_algorithm) = loop {
            let raw_header = if let Some(data) = parts.next() {
                data.trim()
            } else {
                return Err("Invalid JWT: missing header.".into());
            };

            let raw_header = match raw_header.rfind(|c: char| !is_jwt_allowed_char(&c)) {
                Some(index) => {
                    start_over.push_str(&raw_header[0..(index + 1)]);
                    &raw_header[(index + 1)..]
                }
                None => raw_header,
            };

            let parsed_header = match String::from_utf8(match decode_base64(raw_header) {
                Ok(data) => data,
                Err(_) => {
                    start_over.push_str(raw_header);
                    start_over.push('.');
                    continue;
                }
            }) {
                Ok(parsed) => parsed,
                Err(_) => {
                    start_over.push_str(raw_header);
                    start_over.push('.');
                    continue;
                }
            };

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

            break (raw_header.to_owned(), parsed_header, signature_algorithm);
        };

        let raw_payload = parts
            .next()
            .ok_or_else(|| "JWT Payload is not present".to_owned())?
            .to_owned();
        let parsed_payload = String::from_utf8(decode_base64(&raw_payload)?)
            .map_err(|err| format!("Decoded payload is not UTF-8 text: {:?}", err))?;

        let mut leftover = String::new();

        let raw_signature = parts.next().ok_or_else(|| "JWT Signature is not present".to_owned())?;
        let raw_signature = match raw_signature.find(|c: char| !is_jwt_allowed_char(&c)) {
            Some(index) => {
                leftover.push_str(&raw_signature[index..]);
                raw_signature[0..index].to_owned()
            }
            None => raw_signature.to_owned(),
        };
        let signature = decode_base64(&raw_signature)?;
        let parsed_signature = hex::encode(&signature);

        leftover.push_str(
            &parts
                .next()
                .map(|part| {
                    let mut part = part.to_owned();
                    part.insert(0, '.');
                    part
                })
                .unwrap_or_default(),
        );
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

            start_over,
            leftover,
        }))
    }
}

impl Default for Jwte {
    fn default() -> Self {
        Jwte::Jwt(Jwt::default())
    }
}
