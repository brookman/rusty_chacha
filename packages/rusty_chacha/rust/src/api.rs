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

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

#[flutter_rust_bridge::frb(opaque)]
pub struct Secrets {
    key: Vec<u8>,
    nonce: Vec<u8>, // Bad idea
}

pub fn generate_secrets() -> Secrets {
    let key = generate_random_cha_cha20_key().to_vec();
    let nonce = generate_random_cha_cha20_nonce().to_vec();
    Secrets { key, nonce }
}

pub fn read_clear_text(file_path: String) -> Result<Vec<u8>> {
    let data = fs::read(file_path)?;
    Ok(data)
}

pub fn encrypt(secrets: &Secrets, clear: Vec<u8>) -> Result<Vec<u8>> {
    let key = chacha20poly1305::Key::from_slice(&secrets.key);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = chacha20poly1305::Nonce::from_slice(&secrets.nonce);
    let data = cipher
        .encrypt(
            nonce,
            Payload {
                msg: &clear,
                aad: &[],
            },
        )
        .map_err(anyhow::Error::msg)?;
    Ok(data)
}

pub fn write(encrypted: Vec<u8>, file_path: String) -> Result<()> {
    fs::write(file_path, encrypted)?;
    Ok(())
}

pub fn read_encrypted(file_path: String) -> Result<Vec<u8>> {
    let data = fs::read(file_path)?;
    Ok(data)
}

pub fn decrypt(secrets: &Secrets, encrypted: Vec<u8>) -> Result<Vec<u8>> {
    let key = chacha20poly1305::Key::from_slice(&secrets.key);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = chacha20poly1305::Nonce::from_slice(&secrets.nonce);
    let data = cipher
        .decrypt(
            nonce,
            Payload {
                msg: &encrypted,
                aad: &[],
            },
        )
        .map_err(anyhow::Error::msg)?;
    Ok(data)
}

pub fn encrypt_and_write(clear: Vec<u8>, file_path: String) -> Result<Secrets> {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let data = cipher
        .encrypt(
            &nonce,
            Payload {
                msg: &clear,
                aad: &[],
            },
        )
        .map_err(anyhow::Error::msg)?;
    fs::write(file_path, data)?;
    Ok(Secrets {
        key: key.to_vec(),
        nonce: nonce.to_vec(),
    })
}

pub fn read_and_decrypt(secrets: Secrets, file_path: String) -> Result<Vec<u8>> {
    let key = chacha20poly1305::Key::from_slice(&secrets.key);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = chacha20poly1305::Nonce::from_slice(&secrets.nonce);

    let encrypted = fs::read(file_path)?;
    let data = cipher
        .decrypt(
            nonce,
            Payload {
                msg: &encrypted,
                aad: &[],
            },
        )
        .map_err(anyhow::Error::msg)?;
    Ok(data)
}
