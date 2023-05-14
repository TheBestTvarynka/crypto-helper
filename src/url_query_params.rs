use crate::crypto_helper::Algorithm;

const APP_HOST: &str = env!("APP_HOST");

pub fn generate_crypto_helper_link(algorithm: &Algorithm) -> String {
    let mut link = APP_HOST.to_string();

    link.push_str("/crypto-helper/?");
    link.push_str(&serde_qs::to_string(algorithm).unwrap());

    link
}
