use picky::signature::SignatureAlgorithm;
use picky_krb::crypto::{Checksum, Cipher};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rsa::{PaddingScheme, PublicKey as PublicKeyTrait};
use std::convert::TryInto;
use bcrypt::Version;

use super::algorithm::{KrbInput, KrbInputData, KrbMode, RsaAction, RsaInput, BcryptInput, BcryptAction, BcryptHashAction};

pub fn process_rsa(input: &RsaInput) -> Result<Vec<u8>, String> {
    let payload = &input.payload;
    match &input.action {
        RsaAction::Encrypt(public_key) => {
            let mut rng = ChaCha8Rng::from_entropy();
            public_key
                .encrypt(&mut rng, PaddingScheme::PKCS1v15Encrypt, payload)
                .map_err(|err| err.to_string())
        }
        RsaAction::Decrypt(private_key) => private_key
            .decrypt(PaddingScheme::PKCS1v15Encrypt, payload)
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
        BcryptAction::Hash(bcrypt_hash_action) => hash_bcrypt(bcrypt_hash_action, &input.data),
        BcryptAction::Verify(hashed) => verify_bcrypt(&input.data, hashed),
    }
}
fn hash_bcrypt(hash_action: &BcryptHashAction, password: &[u8]) -> Result<Vec<u8>, String> {
    match hash_action.salt.len() {
        16 => hash_with_salt_bcrypt(password, hash_action.rounds, &hash_action.salt),
        0 => hash_without_salt_bcrypt(password, hash_action.rounds),
        len => Err(format!("Invalid bcrypt salt len: expected 16 bytes but got {}", len))
    }
}
fn hash_with_salt_bcrypt(password: &[u8], rounds: u32, salt: &[u8]) -> Result<Vec<u8>, String> {
    let res = bcrypt::hash_with_salt(password, rounds, salt.try_into().expect("Should not be visible")); // it's guaranteed that salt's length is 16
    match &res {
        Ok(hash_parts) => Ok(hash_parts.format_for_version(Version::TwoB).into_bytes()),
        Err(e) => Err(e.to_string()),
    }
}
fn hash_without_salt_bcrypt(password: &[u8], cost: u32) -> Result<Vec<u8>, String> {
    match bcrypt::hash(password, cost) {
        Ok(hash) => Ok(hash.into_bytes()),
        Err(e) => Err(e.to_string())
    }
}
fn verify_bcrypt(password: &[u8], hashed: &str) -> Result<Vec<u8>, String>{
    match bcrypt::verify(password, hashed) {
        Ok(res) => Ok(match res {
            true => vec![1],
            false => vec![],
        }),
        Err(e) => Err(e.to_string()),
    }
}