use serde_json::Value;

use super::signature::JwtSignatureAlgorithm;

pub mod editor;
pub mod viewer;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Jwt {
    pub raw_header: String,
    pub parsed_header: String,

    pub raw_payload: String,
    pub parsed_payload: String,

    pub raw_signature: String,
    pub parsed_signature: String,
    pub signature: Vec<u8>,
    pub signature_algorithm: JwtSignatureAlgorithm,

    pub leftover: String,
}

impl Jwt {
    pub fn set_parsed_header(&mut self, parsed_header: impl Into<String>) {
        self.parsed_header = parsed_header.into();

        let header: Result<Value, _> = serde_json::from_str(&self.parsed_header);
        self.signature_algorithm = header
            .map(|header| {
                let algo: JwtSignatureAlgorithm = header
                    .get("alg")
                    .map(|algo| algo.try_into().unwrap_or_default())
                    .unwrap_or_default();
                algo
            })
            .unwrap_or_default();
    }
}
