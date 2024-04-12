use anyhow::Result;
use std::fs;

use chacha20poly1305::{
    aead::{Aead, OsRng, Payload},
    AeadCore, ChaCha20Poly1305, KeyInit,
};

pub fn generate_random_cha_cha20_key() -> [u8; 32] {
    ChaCha20Poly1305::generate_key(&mut OsRng).into()
}

pub fn generate_random_cha_cha20_nonce() -> [u8; 12] {
    ChaCha20Poly1305::generate_nonce(&mut OsRng).into() // must be unique per file
}

#[flutter_rust_bridge::frb(opaque)]
pub struct Key {
    key: Vec<u8>,
}

pub fn generate_key() -> Key {
    Key {
        key: generate_random_cha_cha20_key().to_vec(),
    }
}

pub fn read_file(file_path: String) -> Result<Vec<u8>> {
    Ok(fs::read(file_path)?)
}

pub fn write_file(data: Vec<u8>, file_path: String) -> Result<()> {
    fs::write(file_path, data)?;
    Ok(())
}

pub fn encrypt(key: &Key, cleartext: Vec<u8>) -> Result<Vec<u8>> {
    let key = chacha20poly1305::Key::from_slice(&key.key);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let mut result = nonce.to_vec();
    result.extend(
        cipher
            .encrypt(
                &nonce,
                Payload {
                    msg: &cleartext,
                    aad: &[],
                },
            )
            .map_err(anyhow::Error::msg)?,
    );
    Ok(result)
}

pub fn decrypt(key: &Key, encrypted: Vec<u8>) -> Result<Vec<u8>> {
    let key = chacha20poly1305::Key::from_slice(&key.key);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = chacha20poly1305::Nonce::from_slice(&encrypted[..12]);
    let data = cipher
        .decrypt(
            nonce,
            Payload {
                msg: &encrypted[12..],
                aad: &[],
            },
        )
        .map_err(anyhow::Error::msg)?;
    Ok(data)
}

pub fn encrypt_and_write_to_file(key: &Key, cleartext: Vec<u8>, file_path: String) -> Result<()> {
    write_file(encrypt(key, cleartext)?, file_path)
}

pub fn read_from_file_and_decrypt(key: &Key, file_path: String) -> Result<Vec<u8>> {
    decrypt(key, read_file(file_path)?)
}
