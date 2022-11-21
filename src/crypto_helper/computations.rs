use picky::{
    key::{PrivateKey, PublicKey},
    signature::SignatureAlgorithm,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey},
    PaddingScheme, PublicKey as PublicKeyTrait, RsaPrivateKey, RsaPublicKey,
};

use super::{
    algorithm::{RsaAction, RsaInput},
    from_hex,
};

pub fn process_rsa(input: &RsaInput) -> Result<Vec<u8>, String> {
    let payload = from_hex(&input.payload)?;
    match &input.action {
        RsaAction::Encrypt(input) => {
            let public_key = RsaPublicKey::from_pkcs1_pem(input).map_err(|err| err.to_string())?;
            let mut rng = ChaCha8Rng::from_entropy();
            public_key
                .encrypt(&mut rng, PaddingScheme::PKCS1v15Encrypt, &payload)
                .map_err(|err| err.to_string())
        }
        RsaAction::Decrypt(input) => {
            let private_key =
                RsaPrivateKey::from_pkcs1_pem(input).map_err(|err| err.to_string())?;
            private_key
                .decrypt(PaddingScheme::PKCS1v15Encrypt, &payload)
                .map_err(|err| err.to_string())
        }
        RsaAction::Sign(input) => {
            let private_key =
                PrivateKey::from_pem_str(&input.rsa_key).map_err(|err| err.to_string())?;
            Ok(SignatureAlgorithm::RsaPkcs1v15(input.hash_algorithm)
                .sign(&payload, &private_key)
                .map_err(|err| err.to_string())?)
        }
        RsaAction::Verify(input) => {
            let signature = from_hex(&input.signature)?;
            let public_key =
                PublicKey::from_pem_str(&input.rsa_key).map_err(|err| err.to_string())?;
            SignatureAlgorithm::RsaPkcs1v15(input.hash_algorithm)
                .verify(&public_key, &payload, &signature)
                .map(|_| vec![1])
                .map_err(|err| err.to_string())
        }
    }
}
