use std::fmt::{self, Display};

use serde_json::Value;

const JWT_SIGNATURE_ALGORITHMS: [&str; 4] = ["HS256", "HS512", "none", "RS256"];

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum JwtSignatureAlgorithm {
    None,
    /// HMAC using SHA-256
    Hs256(String),
    /// HMAC using SHA-512
    Hs512(String),
    /// RSASSA-PKCS1-v1_5 using SHA-256
    ///
    /// A signature can only be generated using the private key.
    RS256(String),
    Unsupported(String),
}

// quick ref: https://www.drupal.org/files/issues/2022-01-08/fix-key-size-3257542-2.patch

impl JwtSignatureAlgorithm {
    pub fn key_len_hint(&self) -> Option<usize> {
        match self {
            JwtSignatureAlgorithm::None => Some(0),
            JwtSignatureAlgorithm::Hs256(_) => Some(32),
            JwtSignatureAlgorithm::Hs512(_) => Some(64),
            JwtSignatureAlgorithm::RS256(_) => None,
            JwtSignatureAlgorithm::Unsupported(_) => None,
        }
    }

    pub fn is_supported(&self) -> bool {
        !matches!(self, JwtSignatureAlgorithm::Unsupported(_))
    }
}

impl TryFrom<&Value> for JwtSignatureAlgorithm {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Err("Invalid jwt signature algorithm: null but string expected".into()),
            Value::Bool(_) => Err("Invalid jwt signature algorithm: bool but string expected".into()),
            Value::Number(_) => Err("Invalid jwt signature algorithm: number but string expected".into()),
            Value::String(value) => {
                if value == JWT_SIGNATURE_ALGORITHMS[0] {
                    Ok(Self::Hs256(Default::default()))
                } else if value == JWT_SIGNATURE_ALGORITHMS[1] {
                    Ok(Self::Hs512(Default::default()))
                } else if value == JWT_SIGNATURE_ALGORITHMS[2] {
                    Ok(Self::None)
                } else if value == JWT_SIGNATURE_ALGORITHMS[3] {
                    Ok(Self::RS256(Default::default()))
                } else {
                    Ok(Self::Unsupported(value.clone()))
                }
            }
            Value::Array(_) => Err("Invalid jwt signature algorithm: array but string expected".into()),
            Value::Object(_) => Err("Invalid jwt signature algorithm: object but string expected".into()),
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
            JwtSignatureAlgorithm::None => write!(f, "{}", JWT_SIGNATURE_ALGORITHMS[2]),
            JwtSignatureAlgorithm::Hs256(_) => write!(f, "{}", JWT_SIGNATURE_ALGORITHMS[0]),
            JwtSignatureAlgorithm::Hs512(_) => write!(f, "{}", JWT_SIGNATURE_ALGORITHMS[1]),
            JwtSignatureAlgorithm::RS256(_) => write!(f, "{}", JWT_SIGNATURE_ALGORITHMS[3]),
            JwtSignatureAlgorithm::Unsupported(algo) => write!(f, "{}", algo),
        }
    }
}
