use picky::hash::HashAlgorithm;

pub const SUPPORTED_ALGORITHMS: [&str; 9] = [
    "MD5",
    "SHA1",
    "SHA256",
    "SHA512",
    "AES128-CTS-HMAC-SHA1-96",
    "AES256-CTS-HMAC-SHA1-96",
    "HMAC-SHA1-96-AES128",
    "HMAC-SHA1-96-AES256",
    "RSA",
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
pub struct RsaSignInput {
    pub hash_algorithm: HashAlgorithm,
    pub rsa_key: String,
}

impl Default for RsaSignInput {
    fn default() -> Self {
        Self {
            hash_algorithm: HashAlgorithm::SHA1,
            rsa_key: String::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RsaVerifyInput {
    pub hash_algorithm: HashAlgorithm,
    pub rsa_key: String,
    pub signature: String,
}

impl Default for RsaVerifyInput {
    fn default() -> Self {
        RsaVerifyInput {
            hash_algorithm: HashAlgorithm::SHA1,
            rsa_key: String::new(),
            signature: String::new(),
        }
    }
}

const RSA_ACTIONS: [&str; 4] = ["Sign", "Verify", "Encrypt", "Decrypt"];
pub const RSA_HASH_ALGOS: [&str; 8] = [
    "MD5", "SHA1", "SHA2_224", "SHA2_256", "SHA2_384", "SHA2_512", "SHA3_384", "SHA3_512",
];

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RsaAction {
    Encrypt(String),
    Decrypt(String),
    Sign(RsaSignInput),
    Verify(RsaVerifyInput),
}

impl RsaAction {
    pub fn enumerate_actions() -> &'static [&'static str; 4] {
        &RSA_ACTIONS
    }
}

impl TryFrom<&str> for RsaAction {
    type Error = ();

    fn try_from(action_literal: &str) -> Result<Self, Self::Error> {
        if action_literal == RSA_ACTIONS[0] {
            Ok(RsaAction::Sign(Default::default()))
        } else if action_literal == RSA_ACTIONS[1] {
            Ok(RsaAction::Verify(Default::default()))
        } else if action_literal == RSA_ACTIONS[2] {
            Ok(RsaAction::Encrypt(Default::default()))
        } else if action_literal == RSA_ACTIONS[3] {
            Ok(RsaAction::Decrypt(Default::default()))
        } else {
            Err(())
        }
    }
}

impl AsRef<str> for RsaAction {
    fn as_ref(&self) -> &str {
        match self {
            RsaAction::Encrypt(_) => "Encrypt",
            RsaAction::Decrypt(_) => "Decrypt",
            RsaAction::Sign(_) => "Sign",
            RsaAction::Verify(_) => "Verify",
        }
    }
}

impl PartialEq<&str> for RsaAction {
    fn eq(&self, other: &&str) -> bool {
        self.as_ref() == *other
    }
}

impl Default for RsaAction {
    fn default() -> Self {
        Self::Sign(Default::default())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct RsaInput {
    // false - sign
    // true - validate
    pub action: RsaAction,
    pub payload: String,
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
    Rsa(RsaInput),
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
        } else if value == SUPPORTED_ALGORITHMS[8] {
            return Ok(Algorithm::Rsa(Default::default()));
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
            Algorithm::Rsa(_) => SUPPORTED_ALGORITHMS[8],
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
        Algorithm::Rsa(Default::default())
    }
}
