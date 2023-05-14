use crate::crypto_helper::Algorithm;

const APP_HOST: &str = env!("APP_HOST");

pub fn generate_crypto_helper_link(algorithm: &Algorithm) -> String {
    let mut link = APP_HOST.to_string();

    link.push_str("/crypto-helper/&");
    link.push_str(&serde_qs::to_string(algorithm).unwrap());

    link
}

#[cfg(test)]
mod tests {
    use crate::crypto_helper::{Algorithm, KrbInput, KrbInputData, KrbMode};
    use crate::url_query_params::generate_crypto_helper_link;

    #[test]
    fn q() {
        println!(
            "{}",
            generate_crypto_helper_link(&Algorithm::Md5(vec![
                84, 104, 101, 66, 101, 115, 116, 84, 118, 97, 114, 121, 110, 107, 97
            ]))
        );

        println!(
            "{}",
            generate_crypto_helper_link(&Algorithm::Aes256CtsHmacSha196(KrbInput {
                mode: KrbMode::Encrypt,
                data: KrbInputData {
                    key: vec![1, 2, 3, 4, 5],
                    key_usage: 4,
                    payload: vec![5, 4, 5, 45, 95, 3, 54, 53],
                },
            }))
        );
    }
}
