use std::fmt::{self, Display};

use serde_json::Value;

const JWT_SIGNATURE_ALGORITHMS: [&str; 9] = [
    "HS256",
    "HS512",
    "none",
    "RS256",
    "HS384",
    "RS384",
    "RS512",
    "ES256",
    "ES384"
];

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum JwtSignatureAlgorithm {
    None,

    /// HMAC using SHA-256
    Hs256(String),

    /// HMAC using SHA-384
    Hs384(String),

    /// HMAC using SHA-512
    Hs512(String),

    /// RSASSA-PKCS1-v1_5 using SHA-256
    /// A signature can only be generated using the private key.
    Rs256(String),

    /// RSASSA-PKCS1-v1_5 using SHA-384
    Rs384(String),

    /// RSASSA-PKCS1-v1_5 using SHA-512
    Rs512(String),

    /// ECDSA using P-256 and SHA-256
    Es256(String),

    /// ECDSA using P-256 and SHA-384
    Es384(String),

    Unsupported(String),
}

// quick ref: https://www.drupal.org/files/issues/2022-01-08/fix-key-size-3257542-2.patch

impl JwtSignatureAlgorithm {
    pub fn key_len_hint(&self) -> Option<usize> {
        match self {
            JwtSignatureAlgorithm::None => Some(0),
            JwtSignatureAlgorithm::Hs256(_) => Some(32),
            JwtSignatureAlgorithm::Hs384(_) => Some(48),
            JwtSignatureAlgorithm::Hs512(_) => Some(64),
            JwtSignatureAlgorithm::Rs256(_) => None,
            JwtSignatureAlgorithm::Rs384(_) => None,
            JwtSignatureAlgorithm::Rs512(_) => None,
            JwtSignatureAlgorithm::Es256(_) => None,
            JwtSignatureAlgorithm::Es384(_) => None,
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
                } else if value == JWT_SIGNATURE_ALGORITHMS[4] {
                    Ok(Self::Hs384(Default::default()))
                } else if value == JWT_SIGNATURE_ALGORITHMS[1] {
                    Ok(Self::Hs512(Default::default()))
                } else if value == JWT_SIGNATURE_ALGORITHMS[2] {
                    Ok(Self::None)
                } else if value == JWT_SIGNATURE_ALGORITHMS[3] {
                    Ok(Self::Rs256(Default::default()))
                } else if value == JWT_SIGNATURE_ALGORITHMS[5] {
                    Ok(Self::Rs384(Default::default()))
                } else if value == JWT_SIGNATURE_ALGORITHMS[6] {
                    Ok(Self::Rs512(Default::default()))
                } else if value == JWT_SIGNATURE_ALGORITHMS[7] {
                    Ok(Self::Es256(Default::default()))
                } else if value == JWT_SIGNATURE_ALGORITHMS[8] {
                    Ok(Self::Es384(Default::default()))
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
            JwtSignatureAlgorithm::Hs384(_) => write!(f, "{}", JWT_SIGNATURE_ALGORITHMS[4]),
            JwtSignatureAlgorithm::Hs512(_) => write!(f, "{}", JWT_SIGNATURE_ALGORITHMS[1]),
            JwtSignatureAlgorithm::Rs256(_) => write!(f, "{}", JWT_SIGNATURE_ALGORITHMS[3]),
            JwtSignatureAlgorithm::Rs384(_) => write!(f, "{}", JWT_SIGNATURE_ALGORITHMS[5]),
            JwtSignatureAlgorithm::Rs512(_) => write!(f, "{}", JWT_SIGNATURE_ALGORITHMS[6]),
            JwtSignatureAlgorithm::Es256(_) => write!(f, "{}", JWT_SIGNATURE_ALGORITHMS[7]),
            JwtSignatureAlgorithm::Es384(_) => write!(f, "{}", JWT_SIGNATURE_ALGORITHMS[8]),
            JwtSignatureAlgorithm::Unsupported(algo) => write!(f, "{}", algo),
        }
    }
}
