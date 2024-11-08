use std::collections::HashMap;
use base64::{prelude::BASE64_STANDARD, Engine};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use anyhow::Result;
use serde::Serialize;

/// rand_string generates a random string of the given size
pub fn rand_string(size: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

/// obj_hash generate a hash of the given serializable object
pub fn obj_hash(value: impl Serialize) -> Result<String> {
    // serialize to json and md5 hash it
    let content = serde_json::to_string(&value)?;
    Ok(format!("{:x}", md5::compute(content)))
}

/// base64encode encode the given string
pub fn base64encode(value: Vec<u8>) -> String {
    BASE64_STANDARD.encode(value)
}

/// base64decode decode the given string
pub fn base64decode(value: &str) -> Result<Vec<u8>> {
    Ok(BASE64_STANDARD.decode(value)?)
}

/// decode map from secret and encrypt string
pub fn decode(secret: &str, encrypt_string: &str) -> Result<HashMap<String, String>> {
    let encrypt_data = base64decode(encrypt_string)?;
    let decrypt_data = simple_crypt::decrypt(&encrypt_data, secret.as_bytes())?;
    let q_map = serde_json::from_slice(&decrypt_data)?;
    Ok(q_map)
}

/// encode_map encode map to secret and encrypt string
pub fn encode_map(m: HashMap<String, String>) -> Result<(String, String)> {
    let q_value = serde_json::to_vec(&m)?;
    let secret = rand_string(12);
    let encrypt_data = simple_crypt::encrypt(&q_value, secret.as_bytes())?;
    let encrypt_string = base64encode(encrypt_data);
    Ok((secret, encrypt_string))
}

/// encode_map_with_secret encode map to encrypt string
pub fn encode_map_with_secret(m: HashMap<String, String>, secret: &str) -> Result<String> {
    let q_value = serde_json::to_vec(&m)?;
    let encrypt_data = simple_crypt::encrypt(&q_value, secret.as_bytes())?;
    let encrypt_string = base64encode(encrypt_data);
    Ok(encrypt_string)
}