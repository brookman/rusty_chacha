use anyhow::{bail, Result};
use std::fs;

use chacha20poly1305::{
    aead::{Aead, OsRng, Payload},
    AeadCore, ChaCha20Poly1305, Key, KeyInit, Nonce,
};

pub fn generate_cha_cha_20_key() -> Vec<u8> {
    ChaCha20Poly1305::generate_key(&mut OsRng).to_vec()
}

/// Important: A nonce must only be used once.
/// Do not encrypt multiple pieces of data with the same nonce.
pub fn generate_cha_cha_20_nonce() -> Vec<u8> {
    // must be unique per piece of data!
    ChaCha20Poly1305::generate_nonce(&mut OsRng).to_vec()
}

/// Encrypts `cleartext` using the given ChaCha20 `key` (32 bytes).
/// A randomly generated nonce is prepended to the result (first 12 bytes).
pub fn encrypt(key: Vec<u8>, cleartext: Vec<u8>, aad: Option<Vec<u8>>) -> Result<Vec<u8>> {
    let aad = aad.unwrap_or_default();
    let nonce = generate_cha_cha_20_nonce();
    let cipertext = encrypt_internal(&key, &cleartext, &aad, &nonce)?;

    let mut result = vec![];
    result.extend(nonce); // prepend nonce to the result
    result.extend(cipertext);
    Ok(result)
}

/// Decrypts `ciphertext` using the given ChaCha20 `key` (32 bytes).
/// The first 12 bytes of the `ciphertext` must contain the nonce.
/// This is already the case when using `encrypt()`.
pub fn decrypt(key: Vec<u8>, ciphertext: Vec<u8>, aad: Option<Vec<u8>>) -> Result<Vec<u8>> {
    if ciphertext.len() < 12 {
        bail!("Ciphertext too short to contain nonce (must be >= 12 bytes)");
    }

    let aad = aad.unwrap_or_default();
    let nonce = &ciphertext[..12]; // the first 12 bytes
    let ciphertext = &ciphertext[12..]; // the rest

    decrypt_internal(&key, ciphertext, &aad, nonce)
}

fn encrypt_internal(key: &[u8], cleartext: &[u8], aad: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
    if key.len() != 32 {
        bail!("Key must be 32 bytes long");
    }
    if nonce.len() != 12 {
        bail!("Nonce must be 12 bytes long");
    }

    let key = Key::from_slice(key);
    let cipher = ChaCha20Poly1305::new(key);
    let msg = cleartext;

    cipher
        .encrypt(Nonce::from_slice(nonce), Payload { msg, aad })
        .map_err(anyhow::Error::msg)
}

fn decrypt_internal(key: &[u8], ciphertext: &[u8], aad: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
    if key.len() != 32 {
        bail!("Key must be 32 bytes long");
    }
    if nonce.len() != 12 {
        bail!("Nonce must be 12 bytes long");
    }

    let key = Key::from_slice(key);
    let cipher = ChaCha20Poly1305::new(key);
    let msg = ciphertext;

    cipher
        .decrypt(Nonce::from_slice(nonce), Payload { msg, aad })
        .map_err(anyhow::Error::msg)
}

pub fn encrypt_to_file(
    key: Vec<u8>,
    cleartext: Vec<u8>,
    file_path: String,
    aad: Option<Vec<u8>>,
) -> Result<()> {
    write_file(encrypt(key, cleartext, aad)?, file_path)
}

pub fn decrypt_from_file(key: Vec<u8>, file_path: String, aad: Option<Vec<u8>>) -> Result<Vec<u8>> {
    decrypt(key, read_file(file_path)?, aad)
}

pub fn write_file(data: Vec<u8>, file_path: String) -> Result<()> {
    fs::write(file_path, data)?;
    Ok(())
}

pub fn read_file(file_path: String) -> Result<Vec<u8>> {
    Ok(fs::read(file_path)?)
}

#[cfg(test)]
mod tests {
    use crate::api::{
        decrypt, decrypt_from_file, encrypt, encrypt_to_file, generate_cha_cha_20_key,
    };

    #[test]
    fn encrypt_and_decrypt_works() {
        let original_cleartext = vec![1, 2, 3];
        let key = generate_cha_cha_20_key();

        let ciphertext = encrypt(key.clone(), original_cleartext.clone(), None).unwrap();
        assert_eq!(ciphertext.len(), 3 + 12 + 16); // 12 bytes nonce + 16 bytes MAC
        let cleartext = decrypt(key, ciphertext, None).unwrap();
        assert_eq!(cleartext, original_cleartext);
    }

    #[test]
    fn encrypt_and_decrypt_with_aad_works() {
        let original_cleartext = vec![1, 2, 3];
        let aad = vec![4, 5, 6];
        let key = generate_cha_cha_20_key();

        let ciphertext =
            encrypt(key.clone(), original_cleartext.clone(), Some(aad.clone())).unwrap();
        let cleartext = decrypt(key, ciphertext, Some(aad.clone())).unwrap();
        assert_eq!(cleartext, original_cleartext);
    }

    #[test]
    fn encrypt_and_decrypt_with_file_works() {
        let original_cleartext = vec![1, 2, 3];
        let key = generate_cha_cha_20_key();
        let file_path = "test_file.bin".to_string();

        encrypt_to_file(
            key.clone(),
            original_cleartext.clone(),
            file_path.clone(),
            None,
        )
        .unwrap();
        let cleartext = decrypt_from_file(key.clone(), file_path, None).unwrap();
        assert_eq!(cleartext, original_cleartext);
    }

    #[test]
    fn decrypt_fails_with_wrong_key() {
        let original_cleartext = vec![1, 2, 3];
        let key = generate_cha_cha_20_key();
        let mut key2 = key.clone();
        key2[0] = (key2[0] + 1) % 255; // change byte 0

        let ciphertext = encrypt(key.clone(), original_cleartext.clone(), None).unwrap();
        let result = decrypt(key2, ciphertext, None);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn decrypt_fails_with_different_aad() {
        let original_cleartext = vec![1, 2, 3];
        let aad = vec![4, 5, 6];
        let aad2 = vec![7, 8, 9];
        let key = generate_cha_cha_20_key();

        let ciphertext =
            encrypt(key.clone(), original_cleartext.clone(), Some(aad.clone())).unwrap();
        let result = decrypt(key, ciphertext, Some(aad2.clone()));
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn decrypt_fails_with_short_ciphertext() {
        let short_ciphertext = vec![1, 2, 3];
        let key = generate_cha_cha_20_key();

        let result = decrypt(key, short_ciphertext, None);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn encrypt_fails_with_invalid_key() {
        let original_cleartext = vec![1, 2, 3];
        let key = vec![1, 2, 3, 4, 5, 6, 7];

        let result = encrypt(key.clone(), original_cleartext.clone(), None);
        assert_eq!(result.is_err(), true);
    }
}
