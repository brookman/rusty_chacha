use anyhow::{bail, Result};
use std::fs;

use chacha20poly1305::{
    aead::{Aead, OsRng, Payload},
    AeadCore, ChaCha20Poly1305, Key, KeyInit, Nonce,
};

const NONCE_LEN: usize = 12;

pub fn generate_cha_cha_20_key() -> Vec<u8> {
    ChaCha20Poly1305::generate_key(&mut OsRng).to_vec()
}

/// Important: A nonce must only be used once!
/// Do not encrypt multiple pieces of data with the same nonce.
pub fn generate_cha_cha_20_nonce() -> Vec<u8> {
    ChaCha20Poly1305::generate_nonce(&mut OsRng).to_vec()
}

/// Encrypts `cleartext` using the given ChaCha20 `key` (32 bytes).
/// A randomly generated nonce will be prepended to the result (first 12 bytes).
pub fn encrypt(key: Vec<u8>, cleartext: Vec<u8>, aad: Option<Vec<u8>>) -> Result<Vec<u8>> {
    let aad = aad.unwrap_or_default();
    let mut nonce = generate_cha_cha_20_nonce();
    let ciphertext = encrypt_internal(&key, &cleartext, &aad, &nonce)?;
    nonce.extend(ciphertext); // append the ciphertext to the nonce
    Ok(nonce)
}

/// Decrypts `ciphertext` using the given ChaCha20 `key` (32 bytes).
/// The first 12 bytes of the `ciphertext` must contain the nonce.
/// This is already the case when using `encrypt()`.
pub fn decrypt(key: Vec<u8>, ciphertext: Vec<u8>, aad: Option<Vec<u8>>) -> Result<Vec<u8>> {
    if ciphertext.len() < NONCE_LEN {
        bail!(
            "Ciphertext too short to contain nonce (must be >= {} bytes)",
            NONCE_LEN
        );
    }
    let aad = aad.unwrap_or_default();
    let nonce = &ciphertext[..NONCE_LEN]; // the first N bytes
    let ciphertext = &ciphertext[NONCE_LEN..]; // the rest
    let cleartext = decrypt_internal(&key, ciphertext, &aad, nonce)?;
    Ok(cleartext)
}

/// Encrypts `cleartext` using the given ChaCha20 `key` (32 bytes).
/// A randomly generated nonce will be prepended to the result (first 12 bytes).
/// `cleartext` will be compressed with the given `zstd_compression_level` before encryption
/// (using ZSTD).
pub fn encrypt_compressed(
    key: Vec<u8>,
    cleartext: Vec<u8>,
    zstd_compression_level: i32,
    aad: Option<Vec<u8>>,
) -> Result<Vec<u8>> {
    let aad = aad.unwrap_or_default();
    let mut nonce = generate_cha_cha_20_nonce();
    let compressed = compress(cleartext, zstd_compression_level)?;
    let ciphertext = encrypt_internal(&key, &compressed, &aad, &nonce)?;
    nonce.extend(ciphertext); // append the ciphertext to the nonce
    Ok(nonce)
}

/// Decrypts `ciphertext` using the given ChaCha20 `key` (32 bytes).
/// The first 12 bytes of the `ciphertext` must contain the nonce.
/// Use this function if the cleartext has been compressed with `encrypt_compressed()`.
pub fn decrypt_compressed(
    key: Vec<u8>,
    ciphertext: Vec<u8>,
    aad: Option<Vec<u8>>,
) -> Result<Vec<u8>> {
    if ciphertext.len() < NONCE_LEN {
        bail!(
            "Ciphertext too short to contain nonce (must be >= {} bytes)",
            NONCE_LEN
        );
    }
    let aad = aad.unwrap_or_default();
    let nonce = &ciphertext[..NONCE_LEN]; // the first N bytes
    let ciphertext = &ciphertext[NONCE_LEN..]; // the rest
    let compressed = decrypt_internal(&key, ciphertext, &aad, nonce)?;
    let cleartext = decompress(compressed)?;
    Ok(cleartext)
}

/// Encrypts `cleartext` using the given ChaCha20 `key` (32 bytes).
/// A randomly generated nonce will be prepended to the result (first 12 bytes).
/// The result is written to `file_path`.
pub fn encrypt_to_file(
    key: Vec<u8>,
    cleartext: Vec<u8>,
    file_path: String,
    aad: Option<Vec<u8>>,
) -> Result<()> {
    let ciphertext = encrypt(key, cleartext, aad)?;
    fs::write(file_path, ciphertext).map_err(anyhow::Error::msg)
}

/// Reads `file_path` and decrypts the contents using the given ChaCha20 `key` (32 bytes).
/// The first 12 bytes of the must contain the nonce.
pub fn decrypt_from_file(key: Vec<u8>, file_path: String, aad: Option<Vec<u8>>) -> Result<Vec<u8>> {
    let ciphertext = fs::read(file_path)?;
    decrypt(key, ciphertext, aad)
}

/// Encrypts `cleartext` using the given ChaCha20 `key` (32 bytes).
/// A randomly generated nonce will be prepended to the result (first 12 bytes).
/// `cleartext` will be compressed with the given `zstd_compression_level` before encryption
/// (using ZSTD).
/// The result is written to `file_path`.
pub fn encrypt_to_file_compressed(
    key: Vec<u8>,
    cleartext: Vec<u8>,
    file_path: String,
    zstd_compression_level: i32,
    aad: Option<Vec<u8>>,
) -> Result<()> {
    let ciphertext = encrypt_compressed(key, cleartext, zstd_compression_level, aad)?;
    fs::write(file_path, ciphertext).map_err(anyhow::Error::msg)
}

/// Reads `file_path` and decrypts the contents using the given ChaCha20 `key` (32 bytes)
/// and decompresses it using ZSTD.
/// The first 12 bytes of the must contain the nonce.
pub fn decrypt_from_file_compressed(
    key: Vec<u8>,
    file_path: String,
    aad: Option<Vec<u8>>,
) -> Result<Vec<u8>> {
    let ciphertext = fs::read(file_path)?;
    decrypt_compressed(key, ciphertext, aad)
}

/// public for benchmarking
pub fn compress(data: Vec<u8>, zstd_compression_level: i32) -> Result<Vec<u8>> {
    let compressed = zstd::stream::encode_all(data.as_slice(), zstd_compression_level)?;
    Ok(compressed)
}

/// public for benchmarking
pub fn decompress(data: Vec<u8>) -> Result<Vec<u8>> {
    let decompressed = zstd::stream::decode_all(data.as_slice())?;
    Ok(decompressed)
}

fn encrypt_internal(key: &[u8], cleartext: &[u8], aad: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
    if key.len() != 32 {
        bail!("Key must be 32 bytes long");
    }
    if nonce.len() != NONCE_LEN {
        bail!("Nonce must be {} bytes long", NONCE_LEN);
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
    if nonce.len() != NONCE_LEN {
        bail!("Nonce must be {} bytes long", NONCE_LEN);
    }
    let key = Key::from_slice(key);
    let cipher = ChaCha20Poly1305::new(key);
    let msg = ciphertext;
    cipher
        .decrypt(Nonce::from_slice(nonce), Payload { msg, aad })
        .map_err(anyhow::Error::msg)
}

// pub fn compress_to_stream_experimental(
//     data: Vec<u8>,
//     sink: StreamSink<Vec<u8>>,
//     zstd_compression_level: Option<i32>,
// ) -> Result<()> {
//     let mut reader = BufReader::new(Cursor::new(data));
//     let mut writer = BufWriter::new(sink);

//     if let Some(level) = zstd_compression_level {
//         zstd::stream::copy_encode(&mut reader, &mut writer, level)?;
//     } else {
//         io::copy(&mut reader, &mut writer)?;
//     };

//     Ok(())
// }

// impl Write for StreamSink<Vec<u8>> {
//     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
//         let data = buf.to_vec();
//         match self.add(data) {
//             Ok(_) => Ok(buf.len()),
//             Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
//         }
//     }

//     fn flush(&mut self) -> io::Result<()> {
//         // Assuming there's no explicit flush mechanism in StreamSink
//         Ok(())
//     }
// }

#[cfg(test)]
mod tests {
    use crate::api::{
        decrypt, decrypt_compressed, decrypt_from_file, decrypt_from_file_compressed, encrypt,
        encrypt_compressed, encrypt_to_file, encrypt_to_file_compressed, generate_cha_cha_20_key,
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
        let file_path = "test_file_1.bin".to_string();

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

    #[test]
    fn compression_decompression_works() {
        // create some compressible data
        let mut original_cleartext = vec![0; 10000];
        original_cleartext[123] = 255;
        original_cleartext[5000] = 22;
        original_cleartext[7004] = 54;

        let key = generate_cha_cha_20_key();

        // test different compression levels
        for i in -10..21 {
            let ciphertext =
                encrypt_compressed(key.clone(), original_cleartext.clone(), i, None).unwrap();
            assert_eq!(ciphertext.len() < original_cleartext.len(), true); // should get smaller
            let cleartext = decrypt_compressed(key.clone(), ciphertext, None).unwrap();
            assert_eq!(cleartext, original_cleartext);
        }
    }

    #[test]
    fn compression_decompression_to_file_works() {
        // create some compressible data
        let mut original_cleartext = vec![0; 10000];
        original_cleartext[123] = 255;
        original_cleartext[5000] = 22;
        original_cleartext[7004] = 54;

        let key = generate_cha_cha_20_key();
        let file_path = "test_file_2.bin".to_string();

        // test different compression levels
        for i in -10..21 {
            encrypt_to_file_compressed(
                key.clone(),
                original_cleartext.clone(),
                file_path.clone(),
                i,
                None,
            )
            .unwrap();
            let cleartext =
                decrypt_from_file_compressed(key.clone(), file_path.clone(), None).unwrap();
            assert_eq!(cleartext, original_cleartext);
        }
    }
}
