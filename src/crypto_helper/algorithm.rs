pub const SUPPORTED_ALGORITHMS: [&str; 8] = [
    "MD5",
    "SHA1",
    "SHA256",
    "SHA512",
    "AES128-CTS-HMAC-SHA1-96",
    "AES256-CTS-HMAC-SHA1-96",
    "HMAC-SHA1-96-AES128",
    "HMAC-SHA1-96-AES256",
];

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct KrbInputData {
    pub key: String,
    pub key_usage: String,
    pub payload: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct KrbInput {
    // false - encrypt
    // true - decrypt
    pub mode: bool,
    pub data: KrbInputData,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Algorithm {
    Md5(String),
    Sha1(String),
    Sha256(String),
    Sha512(String),
    Aes128CtsHmacSha196(KrbInput),
    Aes256CtsHmacSha196(KrbInput),
    HmacSha196Aes128(KrbInputData),
    HmacSha196Aes256(KrbInputData),
}

impl TryFrom<&str> for Algorithm {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == SUPPORTED_ALGORITHMS[0] {
            return Ok(Algorithm::Md5(Default::default()));
        } else if value == SUPPORTED_ALGORITHMS[1] {
            return Ok(Algorithm::Sha1(Default::default()));
        } else if value == SUPPORTED_ALGORITHMS[2] {
            return Ok(Algorithm::Sha256(Default::default()));
        } else if value == SUPPORTED_ALGORITHMS[3] {
            return Ok(Algorithm::Sha512(Default::default()));
        } else if value == SUPPORTED_ALGORITHMS[4] {
            return Ok(Algorithm::Aes128CtsHmacSha196(Default::default()));
        } else if value == SUPPORTED_ALGORITHMS[5] {
            return Ok(Algorithm::Aes256CtsHmacSha196(Default::default()));
        } else if value == SUPPORTED_ALGORITHMS[6] {
            return Ok(Algorithm::HmacSha196Aes128(Default::default()));
        } else if value == SUPPORTED_ALGORITHMS[7] {
            return Ok(Algorithm::HmacSha196Aes256(Default::default()));
        }

        Err(format!("invalid algorithm name: {:?}", value))
    }
}

impl From<&Algorithm> for &str {
    fn from(algorithm: &Algorithm) -> Self {
        match algorithm {
            Algorithm::Md5(_) => SUPPORTED_ALGORITHMS[0],
            Algorithm::Sha1(_) => SUPPORTED_ALGORITHMS[1],
            Algorithm::Sha256(_) => SUPPORTED_ALGORITHMS[2],
            Algorithm::Sha512(_) => SUPPORTED_ALGORITHMS[3],
            Algorithm::Aes128CtsHmacSha196(_) => SUPPORTED_ALGORITHMS[4],
            Algorithm::Aes256CtsHmacSha196(_) => SUPPORTED_ALGORITHMS[5],
            Algorithm::HmacSha196Aes128(_) => SUPPORTED_ALGORITHMS[6],
            Algorithm::HmacSha196Aes256(_) => SUPPORTED_ALGORITHMS[7],
        }
    }
}

impl PartialEq<&str> for &Algorithm {
    fn eq(&self, other: &&str) -> bool {
        let as_str: &str = (*self).into();

        as_str == *other
    }
}

impl Default for Algorithm {
    fn default() -> Self {
        Algorithm::Sha512(Default::default())
    }
}
