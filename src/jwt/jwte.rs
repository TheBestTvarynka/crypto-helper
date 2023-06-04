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

        let r = Ok(Jwte::Jwt(Jwt {
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
        }));

        log::debug!("{:?}", r);

        r
    }
}

impl Default for Jwte {
    fn default() -> Self {
        Jwte::Jwt(Jwt::default())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::super::jwt::Jwt;
    use super::Jwte;
    use crate::jwt::signature::JwtSignatureAlgorithm;

    macro_rules! parse_jwt_success {
        ($name:ident, $raw_jwt:expr, $expected_jwt:expr) => {
            #[test]
            fn $name() {
                let jwt = Jwte::from_str($raw_jwt).unwrap();
                assert_eq!($expected_jwt, jwt);
            }
        };
    }

    parse_jwt_success!(
        jwt_with_quotes_and_http_header_prefix,
        "\"Authorization: Barier: eyJhbGciOiJIUzI1NiJ9.eyJSb2xlIjoiQWRtaW4iLCJJc3N1ZXIiOiJJc3N1ZXIiLCJVc2VybmFtZSI6IkphdmFJblVzZSIsImV4cCI6MTY3MDAwNDI1NCwiaWF0IjoxNjcwMDA0MjU0fQ.ZGsN42vr-bM4uxXowtlNl7xRerkdKu6i29VS8DFQ4Tw\"",
        Jwte::Jwt(Jwt {
            raw_header: "eyJhbGciOiJIUzI1NiJ9".into(),
            parsed_header: "{\"alg\":\"HS256\"}".into(),

            raw_payload: "eyJSb2xlIjoiQWRtaW4iLCJJc3N1ZXIiOiJJc3N1ZXIiLCJVc2VybmFtZSI6IkphdmFJblVzZSIsImV4cCI6MTY3MDAwNDI1NCwiaWF0IjoxNjcwMDA0MjU0fQ".into(),
            parsed_payload: "{\"Role\":\"Admin\",\"Issuer\":\"Issuer\",\"Username\":\"JavaInUse\",\"exp\":1670004254,\"iat\":1670004254}".into(),

            raw_signature: "ZGsN42vr-bM4uxXowtlNl7xRerkdKu6i29VS8DFQ4Tw".into(),
            parsed_signature: "646b0de36bebf9b338bb15e8c2d94d97bc517ab91d2aeea2dbd552f03150e13c".into(),
            signature: vec![100, 107, 13, 227, 107, 235, 249, 179, 56, 187, 21, 232, 194, 217, 77, 151, 188, 81, 122, 185, 29, 42, 238, 162, 219, 213, 82, 240, 49, 80, 225, 60],
            signature_algorithm: JwtSignatureAlgorithm::Hs256(Vec::new()),

            start_over: "\"Authorization: Barier: ".into(),
            leftover: "\"".into(),
        })
    );
    parse_jwt_success!(
        jwt_with_quotes,
        "\"eyJhbGciOiJIUzI1NiJ9.eyJSb2xlIjoiQWRtaW4iLCJJc3N1ZXIiOiJJc3N1ZXIiLCJVc2VybmFtZSI6IkphdmFJblVzZSIsImV4cCI6MTY3MDAwNDI1NCwiaWF0IjoxNjcwMDA0MjU0fQ.ZGsN42vr-bM4uxXowtlNl7xRerkdKu6i29VS8DFQ4Tw\"",
        Jwte::Jwt(Jwt {
            raw_header: "eyJhbGciOiJIUzI1NiJ9".into(),
            parsed_header: "{\"alg\":\"HS256\"}".into(),

            raw_payload: "eyJSb2xlIjoiQWRtaW4iLCJJc3N1ZXIiOiJJc3N1ZXIiLCJVc2VybmFtZSI6IkphdmFJblVzZSIsImV4cCI6MTY3MDAwNDI1NCwiaWF0IjoxNjcwMDA0MjU0fQ".into(),
            parsed_payload: "{\"Role\":\"Admin\",\"Issuer\":\"Issuer\",\"Username\":\"JavaInUse\",\"exp\":1670004254,\"iat\":1670004254}".into(),

            raw_signature: "ZGsN42vr-bM4uxXowtlNl7xRerkdKu6i29VS8DFQ4Tw".into(),
            parsed_signature: "646b0de36bebf9b338bb15e8c2d94d97bc517ab91d2aeea2dbd552f03150e13c".into(),
            signature: vec![100, 107, 13, 227, 107, 235, 249, 179, 56, 187, 21, 232, 194, 217, 77, 151, 188, 81, 122, 185, 29, 42, 238, 162, 219, 213, 82, 240, 49, 80, 225, 60],
            signature_algorithm: JwtSignatureAlgorithm::Hs256(Vec::new()),

            start_over: "\"".into(),
            leftover: "\"".into(),
        })
    );
    parse_jwt_success!(
        jwt_with_fake_header,
        "\"Authorization: Barier: eyJhbGciOiJIU543zI1NiJ9.eyJhbGciOiJIUzI1NiJ9.eyJSb2xlIjoiQWRtaW4iLCJJc3N1ZXIiOiJJc3N1ZXIiLCJVc2VybmFtZSI6IkphdmFJblVzZSIsImV4cCI6MTY3MDAwNDI1NCwiaWF0IjoxNjcwMDA0MjU0fQ.ZGsN42vr-bM4uxXowtlNl7xRerkdKu6i29VS8DFQ4Tw\"",
        Jwte::Jwt(Jwt {
            raw_header: "eyJhbGciOiJIUzI1NiJ9".into(),
            parsed_header: "{\"alg\":\"HS256\"}".into(),

            raw_payload: "eyJSb2xlIjoiQWRtaW4iLCJJc3N1ZXIiOiJJc3N1ZXIiLCJVc2VybmFtZSI6IkphdmFJblVzZSIsImV4cCI6MTY3MDAwNDI1NCwiaWF0IjoxNjcwMDA0MjU0fQ".into(),
            parsed_payload: "{\"Role\":\"Admin\",\"Issuer\":\"Issuer\",\"Username\":\"JavaInUse\",\"exp\":1670004254,\"iat\":1670004254}".into(),

            raw_signature: "ZGsN42vr-bM4uxXowtlNl7xRerkdKu6i29VS8DFQ4Tw".into(),
            parsed_signature: "646b0de36bebf9b338bb15e8c2d94d97bc517ab91d2aeea2dbd552f03150e13c".into(),
            signature: vec![100, 107, 13, 227, 107, 235, 249, 179, 56, 187, 21, 232, 194, 217, 77, 151, 188, 81, 122, 185, 29, 42, 238, 162, 219, 213, 82, 240, 49, 80, 225, 60],
            signature_algorithm: JwtSignatureAlgorithm::Hs256(Vec::new()),

            start_over: "\"Authorization: Barier: eyJhbGciOiJIU543zI1NiJ9.".into(),
            leftover: "\"".into(),
        })
    );
    parse_jwt_success!(
        simple_jwt,
        "eyJhbGciOiJIUzI1NiJ9.eyJSb2xlIjoiQWRtaW4iLCJJc3N1ZXIiOiJJc3N1ZXIiLCJVc2VybmFtZSI6IkphdmFJblVzZSIsImV4cCI6MTY3MDAwNDI1NCwiaWF0IjoxNjcwMDA0MjU0fQ.ZGsN42vr-bM4uxXowtlNl7xRerkdKu6i29VS8DFQ4Tw",
        Jwte::Jwt(Jwt {
            raw_header: "eyJhbGciOiJIUzI1NiJ9".into(),
            parsed_header: "{\"alg\":\"HS256\"}".into(),

            raw_payload: "eyJSb2xlIjoiQWRtaW4iLCJJc3N1ZXIiOiJJc3N1ZXIiLCJVc2VybmFtZSI6IkphdmFJblVzZSIsImV4cCI6MTY3MDAwNDI1NCwiaWF0IjoxNjcwMDA0MjU0fQ".into(),
            parsed_payload: "{\"Role\":\"Admin\",\"Issuer\":\"Issuer\",\"Username\":\"JavaInUse\",\"exp\":1670004254,\"iat\":1670004254}".into(),

            raw_signature: "ZGsN42vr-bM4uxXowtlNl7xRerkdKu6i29VS8DFQ4Tw".into(),
            parsed_signature: "646b0de36bebf9b338bb15e8c2d94d97bc517ab91d2aeea2dbd552f03150e13c".into(),
            signature: vec![100, 107, 13, 227, 107, 235, 249, 179, 56, 187, 21, 232, 194, 217, 77, 151, 188, 81, 122, 185, 29, 42, 238, 162, 219, 213, 82, 240, 49, 80, 225, 60],
            signature_algorithm: JwtSignatureAlgorithm::Hs256(Vec::new()),

            start_over: String::new(),
            leftover: String::new(),
        })
    );
    parse_jwt_success!(
        trash_text_at_both_ends,
        "\"Authorization: Barier: eyJhbGciOiJIU543zI1NiJ9.eyJhbGciOiJIUzI1NiJ9.eyJSb2xlIjoiQWRtaW4iLCJJc3N1ZXIiOiJJc3N1ZXIiLCJVc2VybmFtZSI6IkphdmFJblVzZSIsImV4cCI6MTY3MDAwNDI1NCwiaWF0IjoxNjcwMDA0MjU0fQ.ZGsN42vr-bM4uxXowtlNl7xRerkdKu6i29VS8DFQ4Tw\".eyJhbGciOiJIUzI1NiJ9.sometrash.\"",
        Jwte::Jwt(Jwt {
            raw_header: "eyJhbGciOiJIUzI1NiJ9".into(),
            parsed_header: "{\"alg\":\"HS256\"}".into(),

            raw_payload: "eyJSb2xlIjoiQWRtaW4iLCJJc3N1ZXIiOiJJc3N1ZXIiLCJVc2VybmFtZSI6IkphdmFJblVzZSIsImV4cCI6MTY3MDAwNDI1NCwiaWF0IjoxNjcwMDA0MjU0fQ".into(),
            parsed_payload: "{\"Role\":\"Admin\",\"Issuer\":\"Issuer\",\"Username\":\"JavaInUse\",\"exp\":1670004254,\"iat\":1670004254}".into(),

            raw_signature: "ZGsN42vr-bM4uxXowtlNl7xRerkdKu6i29VS8DFQ4Tw".into(),
            parsed_signature: "646b0de36bebf9b338bb15e8c2d94d97bc517ab91d2aeea2dbd552f03150e13c".into(),
            signature: vec![100, 107, 13, 227, 107, 235, 249, 179, 56, 187, 21, 232, 194, 217, 77, 151, 188, 81, 122, 185, 29, 42, 238, 162, 219, 213, 82, 240, 49, 80, 225, 60],
            signature_algorithm: JwtSignatureAlgorithm::Hs256(Vec::new()),

            start_over: "\"Authorization: Barier: eyJhbGciOiJIU543zI1NiJ9.".into(),
            leftover: "\".eyJhbGciOiJIUzI1NiJ9.sometrash.\"".into(),
        })
    );
}
