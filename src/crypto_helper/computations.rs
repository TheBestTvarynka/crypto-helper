use picky::signature::SignatureAlgorithm;
use picky_krb::crypto::{Checksum, Cipher};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rsa::{PaddingScheme, PublicKey as PublicKeyTrait};

use super::algorithm::{KrbInput, KrbInputData, KrbMode, RsaAction, RsaInput, BcryptInput, BcryptAction};

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
        BcryptAction::Hash(h) => {
            let hash_result = bcrypt::hash(&input.data, h.rounds);
            match hash_result {
                Ok(hash) => Ok(hash.into_bytes()),
                Err(e) => Err(e.to_string())
            }
        },
        BcryptAction::Verify(s) => Err("Not supported yet. Todo!".to_string()), // todo!
    }
    // let res = bcrypt::hash(&input.data, input.rounds);
    // log::debug!("Str - {:?}, len - {}", input.data, input.data.len());
    // match res {
    //     Ok(hash) => Ok(hash.into_bytes()),
    //     Err(e) => Err(e.to_string())
    // }
}