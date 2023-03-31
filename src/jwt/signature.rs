use std::fmt::{self, Display};

use serde_json::Value;

const JWT_SIGNATURE_ALGORITHMS: [&str; 1] = ["HS256"];

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum JwtSignatureAlgorithm {
    Hs256(String),
    Unsupported(String),
}

impl JwtSignatureAlgorithm {
    pub fn key_len_hint(&self) -> Option<usize> {
        match self {
            JwtSignatureAlgorithm::Hs256(_) => Some(32),
            JwtSignatureAlgorithm::Unsupported(_) => None,
        }
    }
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
                    Ok(Self::Unsupported(value.clone()))
                }
            }
            Value::Array(_) => Err("Invalid jwt signature algorithm: array but string extpected".into()),
            Value::Object(_) => Err("Invalid jwt signature algorithm: object but string extpected".into()),
        }
    }
}

impl Default for JwtSignatureAlgorithm {
    fn default() -> Self {
        Self::Unsupported(String::new())
    }
}

impl Display for JwtSignatureAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JwtSignatureAlgorithm::Hs256(_) => write!(f, "HS256"),
            JwtSignatureAlgorithm::Unsupported(algo) => write!(f, "{}", algo),
        }
    }
}
