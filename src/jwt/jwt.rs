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
}
