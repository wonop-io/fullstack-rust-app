use std::num::NonZeroU32;

use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use ring::pbkdf2;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

const KEY_LENGTH: usize = 32;
const ITERATIONS: u32 = 10_000;

pub fn encrypt(plaintext: String, password: &str, salt: &[u8]) -> Vec<u8> {
    let mut key = [0u8; KEY_LENGTH];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(ITERATIONS).unwrap(),
        salt,
        password.as_bytes(),
        &mut key,
    );

    let cipher = Aes256Cbc::new_from_slices(&key, salt).unwrap();
    cipher.encrypt_vec(plaintext.as_bytes())
}

pub fn decrypt(ciphertext: &[u8], password: &str, salt: &[u8]) -> Result<String, String> {
    let mut key = [0u8; KEY_LENGTH];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(ITERATIONS).unwrap(),
        salt,
        password.as_bytes(),
        &mut key,
    );

    let cipher = Aes256Cbc::new_from_slices(&key, salt).unwrap();
    cipher
        .decrypt_vec(ciphertext)
        .map_err(|_| String::from("Decryption failed"))
        .and_then(|decrypted| String::from_utf8(decrypted).map_err(|e| e.to_string()))
}
