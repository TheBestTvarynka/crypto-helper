use serde_json::Value;

pub mod editor;
pub mod viewer;

const JWT_SIGNATURE_ALGORITHMS: [&str; 1] = ["HS256"];

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum JwtSignatureAlgorithm {
    Hs256(String),
    Unsupported,
}

impl TryFrom<&Value> for JwtSignatureAlgorithm {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Err("Invalid jwt signature algorithm: null but string extpected".into()),
            Value::Bool(_) => Err("Invalid jwt signature algorithm: bool but string extpected".into()),
            Value::Number(_) => Err("Invalid jwt signature algorithm: number but string extpected".into()),
            Value::String(value) => {
                if value == JWT_SIGNATURE_ALGORITHMS[0] {
                    Ok(Self::Hs256(Default::default()))
                } else {
                    Ok(Self::Unsupported)
                }
            }
            Value::Array(_) => Err("Invalid jwt signature algorithm: array but string extpected".into()),
            Value::Object(_) => Err("Invalid jwt signature algorithm: object but string extpected".into()),
        }
    }
}

impl Default for JwtSignatureAlgorithm {
    fn default() -> Self {
        Self::Unsupported
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Jwt {
    pub raw_header: String,
    pub parsed_header: String,
    pub header: Value,

    pub raw_payload: String,
    pub parsed_payload: String,
    pub payload: Value,

    pub raw_signature: String,
    pub parsed_signature: String,
    pub signature: Vec<u8>,
    pub signature_algorithm: JwtSignatureAlgorithm,
}
