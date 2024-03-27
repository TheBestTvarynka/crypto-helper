use std::convert::TryInto;
use std::io::Write;

use argon2::{PasswordHasher, PasswordVerifier};
use base64::Engine;
use bcrypt::Version;
use flate2::write::{ZlibDecoder, ZlibEncoder};
use flate2::Compression;
use picky::signature::SignatureAlgorithm;
use picky_krb::crypto::{Checksum, Cipher};
use rsa::rand_core::OsRng;
use rsa::Pkcs1v15Encrypt;

use super::algorithm::{
    Argon2Action, Argon2Input, BcryptAction, BcryptInput, KrbInput, KrbInputData, KrbMode, RsaAction, RsaInput,
    ZlibInput, ZlibMode,
};

pub fn process_rsa(input: &RsaInput) -> Result<Vec<u8>, String> {
    let payload = &input.payload;
    match &input.action {
        RsaAction::Encrypt(public_key) => {
            let mut rng = OsRng;
            public_key
                .encrypt(&mut rng, Pkcs1v15Encrypt, payload)
                .map_err(|err| err.to_string())
        }
        RsaAction::Decrypt(private_key) => private_key
            .decrypt(Pkcs1v15Encrypt, payload)
            .map_err(|err| err.to_string()),
        RsaAction::Sign(input) => Ok(SignatureAlgorithm::RsaPkcs1v15(input.hash_algorithm.0)
            .sign(payload, &input.rsa_private_key)
            .map_err(|err| err.to_string())?),
        RsaAction::Verify(input) => SignatureAlgorithm::RsaPkcs1v15(input.hash_algorithm.0)
            .verify(&input.rsa_public_key, payload, &input.signature)
            .map(|_| vec![1])
            .map_err(|err| err.to_string()),
    }
}

pub fn process_krb_cipher(cipher: Box<dyn Cipher>, input: &KrbInput) -> Result<Vec<u8>, String> {
    match input.mode {
        KrbMode::Decrypt => cipher
            .decrypt(&input.data.key, input.data.key_usage, &input.data.payload)
            .map_err(|err| err.to_string()),
        KrbMode::Encrypt => cipher
            .encrypt(&input.data.key, input.data.key_usage, &input.data.payload)
            .map_err(|err| err.to_string()),
    }
}

pub fn process_krb_hmac(hasher: Box<dyn Checksum>, input: &KrbInputData) -> Result<Vec<u8>, String> {
    hasher
        .checksum(&input.key, input.key_usage, &input.payload)
        .map_err(|err| err.to_string())
}

pub fn process_bcrypt(input: &BcryptInput) -> Result<Vec<u8>, String> {
    match &input.action {
        BcryptAction::Hash(hash) => match hash.salt.len() {
            16 => bcrypt::hash_with_salt(&input.data, hash.rounds, hash.salt.as_slice().try_into().unwrap())
                .map(|hash| hash.format_for_version(Version::TwoB).into_bytes())
                .map_err(|e| e.to_string()),
            0 => bcrypt::hash(&input.data, hash.rounds)
                .map(|hash| hash.into_bytes())
                .map_err(|e| e.to_string()),
            len => Err(format!("Invalid bcrypt salt len: expected 16 bytes but got {}", len)),
        },
        BcryptAction::Verify(hash) => bcrypt::verify(&input.data, hash)
            .map(|r| if r { vec![1] } else { vec![0] })
            .map_err(|e| e.to_string()),
    }
}

pub fn process_zlib(input: &ZlibInput) -> Result<Vec<u8>, String> {
    match input.mode {
        ZlibMode::Compress => {
            let mut compressor = ZlibEncoder::new(Vec::new(), Compression::fast());
            compressor
                .write_all(&input.data)
                .map_err(|err| format!("Can not compress the input data: {:?}", err))?;
            compressor
                .finish()
                .map_err(|err| format!("Can not finish compression: {:?}", err))
        }
        ZlibMode::Decompress => {
            let mut decompressor = ZlibDecoder::new(Vec::new());
            decompressor
                .write_all(&input.data)
                .map_err(|err| format!("Can not decompress the input data: {:?}", err))?;
            decompressor
                .finish()
                .map_err(|err| format!("Can not finish decompression: {:?}", err))
        }
    }
}

pub fn process_argon2(input: &Argon2Input) -> Result<Vec<u8>, String> {
    match &input.action {
        Argon2Action::Hash(hash_action) => {
            let argon2ctx = argon2::Argon2::new(
                hash_action.variant.into(),
                hash_action.version.into(),
                hash_action.into(),
            );

            let salt = if hash_action.salt.is_empty() {
                password_hash::SaltString::generate(OsRng)
            } else {
                password_hash::SaltString::from_b64(
                    &base64::engine::general_purpose::STANDARD_NO_PAD.encode(&hash_action.salt),
                )
                .map_err(|e| e.to_string())?
            };

            let hash = argon2ctx
                .hash_password(&input.data, salt.as_salt())
                .map_err(|err| err.to_string())?
                .to_string();

            Ok(hash.into_bytes())
        }
        Argon2Action::Verify(data) => {
            let str = String::from_utf8(data.clone()).map_err(|e| e.to_string())?;
            let hash = argon2::PasswordHash::new(&str).map_err(|e| e.to_string())?;

            if argon2::Argon2::default().verify_password(&input.data, &hash).is_ok() {
                Ok(vec![1])
            } else {
                Ok(vec![0])
            }
        }
    }
}
