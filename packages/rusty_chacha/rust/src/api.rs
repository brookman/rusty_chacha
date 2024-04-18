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
    ChaCha20Poly1305::generate_nonce(&mut OsRng).to_vec()
}

/// Encrypts `cleartext` using the given ChaCha20 `key` (32 bytes).
/// A randomly generated nonce is prepended to the result (first 12 bytes).
/// Optionally providing a zstd_compression_level will compress `cleartext`
/// before encryption (using ZSTD).
pub fn encrypt(
    key: Vec<u8>,
    cleartext: Vec<u8>,
    aad: Option<Vec<u8>>,
    zstd_compression_level: Option<i32>,
) -> Result<Vec<u8>> {
    let aad = aad.unwrap_or_default();
    let nonce = generate_cha_cha_20_nonce();

    let cleartext = if let Some(level) = zstd_compression_level {
        zstd::stream::encode_all(cleartext.as_slice(), level)?
    } else {
        cleartext
    };

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

    let mut cleartext = decrypt_internal(&key, ciphertext, &aad, nonce)?;

    // Check if the cleartext starts with the ZSTD magic number and try to decompress it.
    // In case decompression fails we just return the original data
    if cleartext.len() > 4 && cleartext[..4] == [0x28, 0xB5, 0x2F, 0xFD] {
        if let Ok(decompressed) = zstd::stream::decode_all(cleartext.as_slice()) {
            cleartext = decompressed;
        }
    }

    Ok(cleartext)
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
    zstd_compression_level: Option<i32>,
) -> Result<()> {
    write_file(
        encrypt(key, cleartext, aad, zstd_compression_level)?,
        file_path,
    )
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

pub fn compress(data: Vec<u8>, zstd_compression_level: i32) -> Result<Vec<u8>> {
    let compressed = zstd::stream::encode_all(data.as_slice(), zstd_compression_level)?;
    Ok(compressed)
}

pub fn decompress(data: Vec<u8>) -> Result<Vec<u8>> {
    let decompressed = zstd::stream::decode_all(data.as_slice())?;
    Ok(decompressed)
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
        decrypt, decrypt_from_file, encrypt, encrypt_to_file, generate_cha_cha_20_key,
    };

    #[test]
    fn encrypt_and_decrypt_works() {
        let original_cleartext = vec![1, 2, 3];
        let key = generate_cha_cha_20_key();

        let ciphertext = encrypt(key.clone(), original_cleartext.clone(), None, None).unwrap();
        assert_eq!(ciphertext.len(), 3 + 12 + 16); // 12 bytes nonce + 16 bytes MAC
        let cleartext = decrypt(key, ciphertext, None).unwrap();
        assert_eq!(cleartext, original_cleartext);
    }

    #[test]
    fn encrypt_and_decrypt_with_aad_works() {
        let original_cleartext = vec![1, 2, 3];
        let aad = vec![4, 5, 6];
        let key = generate_cha_cha_20_key();

        let ciphertext = encrypt(
            key.clone(),
            original_cleartext.clone(),
            Some(aad.clone()),
            None,
        )
        .unwrap();
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

        let ciphertext = encrypt(key.clone(), original_cleartext.clone(), None, None).unwrap();
        let result = decrypt(key2, ciphertext, None);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn decrypt_fails_with_different_aad() {
        let original_cleartext = vec![1, 2, 3];
        let aad = vec![4, 5, 6];
        let aad2 = vec![7, 8, 9];
        let key = generate_cha_cha_20_key();

        let ciphertext = encrypt(
            key.clone(),
            original_cleartext.clone(),
            Some(aad.clone()),
            None,
        )
        .unwrap();
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

        let result = encrypt(key.clone(), original_cleartext.clone(), None, None);
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
                encrypt(key.clone(), original_cleartext.clone(), None, Some(i)).unwrap();
            assert_eq!(ciphertext.len() < original_cleartext.len(), true); // should get smaller
            let cleartext = decrypt(key.clone(), ciphertext, None).unwrap();
            assert_eq!(cleartext, original_cleartext);
        }
    }

    #[test]
    fn decrypt_works_when_cleartext_starts_with_zstd_magic_bytes() {
        let original_cleartext = vec![0x28, 0xB5, 0x2F, 0xFD, 0, 1, 2, 3, 4];
        let key = generate_cha_cha_20_key();

        let ciphertext = encrypt(key.clone(), original_cleartext.clone(), None, None).unwrap();
        let cleartext = decrypt(key, ciphertext, None).unwrap();
        assert_eq!(cleartext, original_cleartext);
    }
}
