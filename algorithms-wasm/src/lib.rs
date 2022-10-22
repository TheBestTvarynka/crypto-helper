mod hashing;
mod utils;

use picky_krb::crypto::CipherSuite;
use utils::hex_to_vec;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, algorithms-wasm!");
}

#[wasm_bindgen]
pub fn simple_hash(data: &str) -> Result<String, String> {
    Err(format!("this data `{}` was hashed!", data))
}

#[wasm_bindgen]
pub fn md5(payload: &str) -> Result<String, String> {
    Ok(hex::encode(md5::compute(&hex_to_vec(payload)?).as_ref()))
}

#[wasm_bindgen]
pub fn sha1(payload: &str) -> Result<String, String> {
    Ok(hex::encode(hashing::sha1(&hex_to_vec(payload)?)))
}

#[wasm_bindgen]
pub fn sha256(payload: &str) -> Result<String, String> {
    Ok(sha256::digest_bytes(&hex_to_vec(payload)?))
}

#[wasm_bindgen]
pub fn sha512(payload: &str) -> Result<String, String> {
    Ok(hex::encode(hmac_sha512::Hash::hash(&hex_to_vec(payload)?)))
}

#[wasm_bindgen]
pub fn aes256_cts_hmac_sha1_96_encrypt(
    key: &str,
    key_usage: i32,
    payload: &str,
) -> Result<String, String> {
    let key = hex_to_vec(key)?;
    let payload = hex_to_vec(payload)?;

    Ok(hex::encode(CipherSuite::Aes256CtsHmacSha196
        .cipher()
        .encrypt(&key, key_usage, &payload)
        .map_err(|e| format!("{:?}", e))?))
}

#[wasm_bindgen]
pub fn aes256_cts_hmac_sha1_96_decrypt(
    key: &str,
    key_usage: i32,
    payload: &str,
) -> Result<String, String> {
    let key = hex_to_vec(key)?;
    let payload = hex_to_vec(payload)?;

    Ok(hex::encode(CipherSuite::Aes256CtsHmacSha196
        .cipher()
        .decrypt(&key, key_usage, &payload)
        .map_err(|e| format!("{:?}", e))?))
}
