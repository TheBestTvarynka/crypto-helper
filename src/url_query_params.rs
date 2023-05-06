use serde::{Deserialize, Serialize};

use crate::crypto_helper::{Algorithm, KrbInput, KrbInputData};

const APP_HOST: &str = env!("APP_HOST");

#[derive(Serialize, Deserialize)]
struct HexData(String);

impl From<&[u8]> for HexData {
    fn from(input: &[u8]) -> Self {
        Self(hex::encode(input))
    }
}

#[derive(Serialize, Deserialize)]
struct KrbInputQuery {
    key: HexData,
    key_usage: i32,
    payload: HexData,
}

impl From<&KrbInputData> for KrbInputQuery {
    fn from(input: &KrbInputData) -> Self {
        Self {
            key: input.key.as_slice().into(),
            key_usage: input.key_usage,
            payload: input.payload.as_slice().into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
enum KrbCipherModeQuery {
    Encrypt,
    Decrypt,
}

impl From<bool> for KrbCipherModeQuery {
    fn from(mode: bool) -> Self {
        if mode {
            Self::Decrypt
        } else {
            Self::Encrypt
        }
    }
}

#[derive(Serialize, Deserialize)]
struct KrbCipherQuery {
    mode: KrbCipherModeQuery,
    data: KrbInputQuery,
}

impl From<&KrbInput> for KrbCipherQuery {
    fn from(input: &KrbInput) -> Self {
        Self {
            mode: input.mode.into(),
            data: (&input.data).into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
enum AlgorithmQuery {
    Md5(HexData),
    Sha1(HexData),
    Sha256(HexData),
    Sha384(HexData),
    Sha512(HexData),
    Aes128CtsHmacSha196(KrbCipherQuery),
    Aes256CtsHmacSha196(KrbCipherQuery),
    HmacSha196Aes128(KrbInputQuery),
    HmacSha196Aes256(KrbInputQuery),
}

pub fn generate_crypto_helper_link(algorithm: &Algorithm) -> String {
    let query = match algorithm {
        Algorithm::Md5(input) => AlgorithmQuery::Md5(input.as_slice().into()),
        Algorithm::Sha1(input) => AlgorithmQuery::Sha1(input.as_slice().into()),
        Algorithm::Sha256(input) => AlgorithmQuery::Sha256(input.as_slice().into()),
        Algorithm::Sha384(input) => AlgorithmQuery::Sha384(input.as_slice().into()),
        Algorithm::Sha512(input) => AlgorithmQuery::Sha512(input.as_slice().into()),
        Algorithm::Aes128CtsHmacSha196(input) => todo!(),
        Algorithm::Aes256CtsHmacSha196(input) => todo!(),
        Algorithm::HmacSha196Aes128(input) => todo!(),
        Algorithm::HmacSha196Aes256(input) => todo!(),
        Algorithm::Rsa(input) => todo!(),
    };

    let mut link = APP_HOST.to_string();

    link.push('&');
    link.push_str(&serde_qs::to_string(&query).unwrap());

    link
}

#[cfg(test)]
mod tests {
    use crate::crypto_helper::Algorithm;
    use crate::url_query_params::generate_crypto_helper_link;

    #[test]
    fn q() {
        println!(
            "{}",
            generate_crypto_helper_link(&Algorithm::Md5(vec![
                84, 104, 101, 66, 101, 115, 116, 84, 118, 97, 114, 121, 110, 107, 97
            ]))
        );
    }
}
