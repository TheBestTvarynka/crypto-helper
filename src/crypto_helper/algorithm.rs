use picky::hash::HashAlgorithm;
use picky::key::{PrivateKey, PublicKey};
use rsa::pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};

use super::serde::*;

pub const SUPPORTED_ALGORITHMS: [&str; 10] = [
    "MD5",                     // 0
    "SHA1",                    // 1
    "SHA256",                  // 2
    "SHA512",                  // 3
    "AES128-CTS-HMAC-SHA1-96", // 4
    "AES256-CTS-HMAC-SHA1-96", // 5
    "HMAC-SHA1-96-AES128",     // 6
    "HMAC-SHA1-96-AES256",     // 7
    "RSA",                     // 8
    "SHA384",                  // 9
];

pub const HASHING_ALGOS: [&str; 5] = [
    SUPPORTED_ALGORITHMS[0],
    SUPPORTED_ALGORITHMS[1],
    SUPPORTED_ALGORITHMS[2],
    SUPPORTED_ALGORITHMS[3],
    SUPPORTED_ALGORITHMS[9],
];

pub const ENCRYPTION_ALGOS: [&str; 3] = [
    SUPPORTED_ALGORITHMS[4],
    SUPPORTED_ALGORITHMS[5],
    SUPPORTED_ALGORITHMS[8],
];

pub const HMAC_ALGOS: [&str; 2] = [SUPPORTED_ALGORITHMS[6], SUPPORTED_ALGORITHMS[7]];

const RSA_ACTIONS: [&str; 4] = ["Sign", "Verify", "Encrypt", "Decrypt"];
pub const RSA_HASH_ALGOS: [&str; 8] = [
    "MD5", "SHA1", "SHA2_224", "SHA2_256", "SHA2_384", "SHA2_512", "SHA3_384", "SHA3_512",
];

const DEFAULT_RSA_PRIVATE_KEY: &str = include_str!("../../public/assets/rsa_private_key.pem");
const DEFAULT_RSA_PUBLIC_KEY: &str = include_str!("../../public/assets/rsa_public_key.pem");

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct KrbInputData {
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    pub key: Vec<u8>,
    pub key_usage: i32,
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    pub payload: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Serialize, Deserialize)]
pub enum KrbMode {
    #[default]
    Encrypt,
    Decrypt,
}

impl From<KrbMode> for bool {
    fn from(mode: KrbMode) -> Self {
        match mode {
            KrbMode::Encrypt => false,
            KrbMode::Decrypt => true,
        }
    }
}

impl From<bool> for KrbMode {
    fn from(mode: bool) -> Self {
        match mode {
            true => KrbMode::Decrypt,
            false => KrbMode::Encrypt,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct KrbInput {
    pub mode: KrbMode,
    pub data: KrbInputData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct RsaHashAlgorithm(pub HashAlgorithm);

impl TryFrom<&str> for RsaHashAlgorithm {
    type Error = ();

    fn try_from(raw: &str) -> Result<Self, Self::Error> {
        if RSA_HASH_ALGOS[0] == raw {
            Ok(Self(HashAlgorithm::MD5))
        } else if RSA_HASH_ALGOS[1] == raw {
            Ok(Self(HashAlgorithm::SHA1))
        } else if RSA_HASH_ALGOS[2] == raw {
            Ok(Self(HashAlgorithm::SHA2_224))
        } else if RSA_HASH_ALGOS[3] == raw {
            Ok(Self(HashAlgorithm::SHA2_256))
        } else if RSA_HASH_ALGOS[4] == raw {
            Ok(Self(HashAlgorithm::SHA2_384))
        } else if RSA_HASH_ALGOS[5] == raw {
            Ok(Self(HashAlgorithm::SHA2_512))
        } else if RSA_HASH_ALGOS[6] == raw {
            Ok(Self(HashAlgorithm::SHA3_384))
        } else if RSA_HASH_ALGOS[7] == raw {
            Ok(Self(HashAlgorithm::SHA3_512))
        } else {
            Err(())
        }
    }
}

impl From<&RsaHashAlgorithm> for &str {
    fn from(rsa_hash_algorithm: &RsaHashAlgorithm) -> Self {
        match &rsa_hash_algorithm.0 {
            HashAlgorithm::MD5 => RSA_HASH_ALGOS[0],
            HashAlgorithm::SHA1 => RSA_HASH_ALGOS[1],
            HashAlgorithm::SHA2_224 => RSA_HASH_ALGOS[2],
            HashAlgorithm::SHA2_256 => RSA_HASH_ALGOS[3],
            HashAlgorithm::SHA2_384 => RSA_HASH_ALGOS[4],
            HashAlgorithm::SHA2_512 => RSA_HASH_ALGOS[5],
            HashAlgorithm::SHA3_384 => RSA_HASH_ALGOS[6],
            HashAlgorithm::SHA3_512 => RSA_HASH_ALGOS[7],
            _ => "Other",
        }
    }
}

impl PartialEq<&str> for RsaHashAlgorithm {
    fn eq(&self, other: &&str) -> bool {
        let as_str: &str = self.into();

        as_str == *other
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct RsaSignInput {
    pub hash_algorithm: RsaHashAlgorithm,
    #[serde(
        serialize_with = "serialize_private_key",
        deserialize_with = "deserialize_private_key"
    )]
    pub rsa_private_key: PrivateKey,
}

impl Default for RsaSignInput {
    fn default() -> Self {
        Self {
            hash_algorithm: RsaHashAlgorithm(HashAlgorithm::SHA1),
            rsa_private_key: PrivateKey::from_pem_str(DEFAULT_RSA_PRIVATE_KEY).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct RsaVerifyInput {
    pub hash_algorithm: RsaHashAlgorithm,
    #[serde(serialize_with = "serialize_public_key", deserialize_with = "deserialize_public_key")]
    pub rsa_public_key: PublicKey,
    pub signature: Vec<u8>,
}

impl Default for RsaVerifyInput {
    fn default() -> Self {
        RsaVerifyInput {
            hash_algorithm: RsaHashAlgorithm(HashAlgorithm::SHA1),
            rsa_public_key: PublicKey::from_pem_str(DEFAULT_RSA_PUBLIC_KEY).unwrap(),
            signature: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum RsaAction {
    #[serde(
        serialize_with = "serialize_rsa_public_key",
        deserialize_with = "deserialize_rsa_public_key"
    )]
    Encrypt(RsaPublicKey),
    #[serde(
        serialize_with = "serialize_rsa_private_key",
        deserialize_with = "deserialize_rsa_private_key"
    )]
    Decrypt(RsaPrivateKey),
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
            Ok(RsaAction::Encrypt(
                RsaPublicKey::from_pkcs1_pem(DEFAULT_RSA_PUBLIC_KEY).unwrap(),
            ))
        } else if action_literal == RSA_ACTIONS[3] {
            Ok(RsaAction::Decrypt(
                RsaPrivateKey::from_pkcs1_pem(DEFAULT_RSA_PRIVATE_KEY).unwrap(),
            ))
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

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct RsaInput {
    pub action: RsaAction,
    pub payload: Vec<u8>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Algorithm {
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    Md5(Vec<u8>),
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    Sha1(Vec<u8>),
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    Sha256(Vec<u8>),
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    Sha384(Vec<u8>),
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    Sha512(Vec<u8>),
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
        } else if value == SUPPORTED_ALGORITHMS[9] {
            return Ok(Algorithm::Sha384(Default::default()));
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

        log::error!("invalid algo literal: {}", value);

        Err(format!("invalid algorithm name: {:?}", value))
    }
}

impl From<&Algorithm> for &str {
    fn from(algorithm: &Algorithm) -> Self {
        match algorithm {
            Algorithm::Md5(_) => SUPPORTED_ALGORITHMS[0],
            Algorithm::Sha1(_) => SUPPORTED_ALGORITHMS[1],
            Algorithm::Sha256(_) => SUPPORTED_ALGORITHMS[2],
            Algorithm::Sha384(_) => SUPPORTED_ALGORITHMS[9],
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
        Algorithm::Aes256CtsHmacSha196(Default::default())
    }
}
