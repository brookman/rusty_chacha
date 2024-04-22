use zstd::{decode_all, encode_all};

use anyhow::{bail, Ok, Result};
use std::fs;

use chacha20poly1305::{
    aead::{Aead, OsRng, Payload},
    AeadCore, ChaCha20Poly1305, Key, KeyInit, XChaCha20Poly1305,
};

pub struct RustyChaCha20Poly1305 {
    pub key: Vec<u8>,
    pub compression: Compression,
}

pub struct RustyXChaCha20Poly1305 {
    pub key: Vec<u8>,
    pub compression: Compression,
}

pub enum Compression {
    Uncompressed,
    Zstd { compression_level: i32 },
}

impl RustyChaCha20Poly1305 {
    const NONCE_LEN: usize = 12;

    pub fn generate_key() -> Vec<u8> {
        ChaCha20Poly1305::generate_key(&mut OsRng).to_vec()
    }

    /// Important: A nonce must only be used once!
    /// Do not encrypt multiple pieces of data with the same nonce.
    pub fn generate_nonce() -> Vec<u8> {
        ChaCha20Poly1305::generate_nonce(&mut OsRng).to_vec()
    }

    /// Do not use this. Use `create()` instead.
    pub fn create_internal(key: Option<Vec<u8>>, compression: Option<Compression>) -> Result<Self> {
        let key = key.unwrap_or_else(Self::generate_key);
        if key.len() != 32 {
            bail!("Key must be 32 bytes long");
        }
        let compression = compression.unwrap_or(Compression::Uncompressed);
        Ok(Self { key, compression })
    }

    /// Encrypts `cleartext` and returns the ciphertext.
    /// If no `nonce` is given, a random one will be generated.
    /// The nonce is always prepended to the ciphertext (first NONCE_LEN bytes).
    pub fn encrypt(
        &self,
        cleartext: Vec<u8>,
        nonce: Option<Vec<u8>>,
        aad: Option<Vec<u8>>,
    ) -> Result<Vec<u8>> {
        let nonce = nonce.unwrap_or_else(Self::generate_nonce);
        if nonce.len() != Self::NONCE_LEN {
            bail!("Nonce must be {} bytes long", Self::NONCE_LEN);
        }
        let nonce = chacha20poly1305::Nonce::from_slice(&nonce);
        let aad = &aad.unwrap_or_default();

        let cleartext = match self.compression {
            Compression::Uncompressed => cleartext,
            Compression::Zstd { compression_level } => compress(cleartext, compression_level)?,
        };

        let key = Key::from_slice(&self.key);
        let cipher = ChaCha20Poly1305::new(key);
        let msg = &cleartext;
        let ciphertext = cipher
            .encrypt(nonce, Payload { msg, aad })
            .map_err(|e| anyhow::anyhow!(e))?;

        let mut result = nonce.to_vec();
        result.extend(ciphertext); // append the ciphertext to the nonce
        Ok(result)
    }

    /// Decrypts `ciphertext` and returns the cleartext.
    /// The first NONCE_LEN bytes of the `ciphertext` must contain the nonce.
    pub fn decrypt(&self, ciphertext: Vec<u8>, aad: Option<Vec<u8>>) -> Result<Vec<u8>> {
        if ciphertext.len() < Self::NONCE_LEN {
            bail!(
                "Ciphertext too short to contain nonce (must be >= {} bytes)",
                Self::NONCE_LEN
            );
        }

        let nonce = chacha20poly1305::Nonce::from_slice(&ciphertext[..Self::NONCE_LEN]); // the first N bytes
        let ciphertext = &ciphertext[Self::NONCE_LEN..]; // the rest
        let aad = &aad.unwrap_or_default();

        let key = Key::from_slice(&self.key);
        let cipher = ChaCha20Poly1305::new(key);
        let msg = ciphertext;
        let cleartext = cipher
            .decrypt(nonce, Payload { msg, aad })
            .map_err(anyhow::Error::msg)?;

        match self.compression {
            Compression::Uncompressed => Ok(cleartext),
            Compression::Zstd { .. } => decompress(cleartext),
        }
    }

    /// Encrypts `cleartext` and write the result to `file_path`.
    /// If no `nonce` is given, a randomly generated one will be generated.
    /// The nonce is always prepended to the result (first NONCE_LEN bytes).
    pub fn encrypt_to_file(
        &self,
        cleartext: Vec<u8>,
        file_path: String,
        nonce: Option<Vec<u8>>,
        aad: Option<Vec<u8>>,
    ) -> Result<()> {
        let ciphertext = self.encrypt(cleartext, nonce, aad)?;
        fs::write(file_path, ciphertext).map_err(anyhow::Error::msg)
    }

    /// Reads `file_path` decrypts the contents and returns the cleartext.
    /// The first NONCE_LEN bytes of the `ciphertext` must contain the nonce.
    pub fn decrypt_from_file(&self, file_path: String, aad: Option<Vec<u8>>) -> Result<Vec<u8>> {
        let ciphertext = fs::read(file_path)?;
        self.decrypt(ciphertext, aad)
    }
}

impl RustyXChaCha20Poly1305 {
    const NONCE_LEN: usize = 24;

    pub fn generate_key() -> Vec<u8> {
        XChaCha20Poly1305::generate_key(&mut OsRng).to_vec()
    }

    /// Important: A nonce must only be used once!
    /// Do not encrypt multiple pieces of data with the same nonce.
    pub fn generate_nonce() -> Vec<u8> {
        XChaCha20Poly1305::generate_nonce(&mut OsRng).to_vec()
    }

    /// Do not use this. Use `create()` instead.
    pub fn create_internal(key: Option<Vec<u8>>, compression: Option<Compression>) -> Result<Self> {
        let key = key.unwrap_or_else(Self::generate_key);
        if key.len() != 32 {
            bail!("Key must be 32 bytes long");
        }
        let compression = compression.unwrap_or(Compression::Uncompressed);
        Ok(Self { key, compression })
    }

    /// Encrypts `cleartext` and returns the ciphertext.
    /// If no `nonce` is given, a random one will be generated.
    /// The nonce is always prepended to the ciphertext (first NONCE_LEN bytes).
    pub fn encrypt(
        &self,
        cleartext: Vec<u8>,
        nonce: Option<Vec<u8>>,
        aad: Option<Vec<u8>>,
    ) -> Result<Vec<u8>> {
        let nonce = nonce.unwrap_or_else(Self::generate_nonce);
        if nonce.len() != Self::NONCE_LEN {
            bail!("Nonce must be {} bytes long", Self::NONCE_LEN);
        }
        let nonce = chacha20poly1305::XNonce::from_slice(&nonce);
        let aad = &aad.unwrap_or_default();

        let cleartext = match self.compression {
            Compression::Uncompressed => cleartext,
            Compression::Zstd { compression_level } => compress(cleartext, compression_level)?,
        };

        let key = Key::from_slice(&self.key);
        let cipher = XChaCha20Poly1305::new(key);
        let msg = &cleartext;
        let ciphertext = cipher
            .encrypt(nonce, Payload { msg, aad })
            .map_err(|e| anyhow::anyhow!(e))?;

        let mut result = nonce.to_vec();
        result.extend(ciphertext); // append the ciphertext to the nonce
        Ok(result)
    }

    /// Decrypts `ciphertext` and returns the cleartext.
    /// The first NONCE_LEN bytes of the `ciphertext` must contain the nonce.
    pub fn decrypt(&self, ciphertext: Vec<u8>, aad: Option<Vec<u8>>) -> Result<Vec<u8>> {
        if ciphertext.len() < Self::NONCE_LEN {
            bail!(
                "Ciphertext too short to contain nonce (must be >= {} bytes)",
                Self::NONCE_LEN
            );
        }

        let nonce = chacha20poly1305::XNonce::from_slice(&ciphertext[..Self::NONCE_LEN]); // the first N bytes
        let ciphertext = &ciphertext[Self::NONCE_LEN..]; // the rest
        let aad = &aad.unwrap_or_default();

        let key = Key::from_slice(&self.key);
        let cipher = XChaCha20Poly1305::new(key);
        let msg = ciphertext;
        let cleartext = cipher
            .decrypt(nonce, Payload { msg, aad })
            .map_err(anyhow::Error::msg)?;

        match self.compression {
            Compression::Uncompressed => Ok(cleartext),
            Compression::Zstd { .. } => decompress(cleartext),
        }
    }

    /// Encrypts `cleartext` and write the result to `file_path`.
    /// If no `nonce` is given, a randomly generated one will be generated.
    /// The nonce is always prepended to the result (first NONCE_LEN bytes).
    pub fn encrypt_to_file(
        &self,
        cleartext: Vec<u8>,
        file_path: String,
        nonce: Option<Vec<u8>>,
        aad: Option<Vec<u8>>,
    ) -> Result<()> {
        let ciphertext = self.encrypt(cleartext, nonce, aad)?;
        fs::write(file_path, ciphertext).map_err(anyhow::Error::msg)
    }

    /// Reads `file_path` decrypts the contents and returns the cleartext.
    /// The first NONCE_LEN bytes of the `ciphertext` must contain the nonce.
    pub fn decrypt_from_file(&self, file_path: String, aad: Option<Vec<u8>>) -> Result<Vec<u8>> {
        let ciphertext = fs::read(file_path)?;
        self.decrypt(ciphertext, aad)
    }
}

/// public for benchmarking
pub fn compress(data: Vec<u8>, zstd_compression_level: i32) -> Result<Vec<u8>> {
    let compressed = encode_all(data.as_slice(), zstd_compression_level)?;
    Ok(compressed)
}

/// public for benchmarking
pub fn decompress(data: Vec<u8>) -> Result<Vec<u8>> {
    let decompressed = decode_all(data.as_slice())?;
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
mod tests1 {
    use crate::api::{Compression, RustyChaCha20Poly1305};

    const CLEARTEXT: &[u8] = &[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
        12, 13, 14, 15, 16, 42, 42, 42, 42, 42, 42, 42, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
    ];

    #[test]
    fn encrypt_and_decrypt_works() {
        let cipher = RustyChaCha20Poly1305::create_internal(None, None).unwrap();

        let ciphertext = cipher.encrypt(CLEARTEXT.to_vec(), None, None).unwrap();
        assert_eq!(ciphertext.len(), CLEARTEXT.len() + 12 + 16); // 12 bytes nonce + 16 bytes MAC
        let cleartext = cipher.decrypt(ciphertext, None).unwrap();
        assert_eq!(cleartext, CLEARTEXT.to_vec());

        let key = RustyChaCha20Poly1305::generate_key();
        let cipher1 = RustyChaCha20Poly1305::create_internal(Some(key.clone()), None).unwrap();
        let cipher2 = RustyChaCha20Poly1305::create_internal(Some(key.clone()), None).unwrap();

        let ciphertext = cipher1.encrypt(CLEARTEXT.to_vec(), None, None).unwrap();
        assert_eq!(ciphertext.len(), CLEARTEXT.len() + 12 + 16); // 12 bytes nonce + 16 bytes MAC
        let cleartext = cipher2.decrypt(ciphertext, None).unwrap();
        assert_eq!(cleartext, CLEARTEXT.to_vec());
    }

    #[test]
    fn encrypt_and_decrypt_with_nonce_works() {
        let nonce = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let cipher = RustyChaCha20Poly1305::create_internal(None, None).unwrap();

        let ciphertext = cipher
            .encrypt(CLEARTEXT.to_vec(), Some(nonce), None)
            .unwrap();
        assert_eq!(ciphertext.len(), CLEARTEXT.len() + 12 + 16); // 12 bytes nonce + 16 bytes MAC
        let cleartext = cipher.decrypt(ciphertext, None).unwrap();
        assert_eq!(cleartext, CLEARTEXT.to_vec());
    }

    #[test]
    fn encrypt_and_decrypt_with_aad_works() {
        let aad = vec![4, 5, 6];
        let cipher = RustyChaCha20Poly1305::create_internal(None, None).unwrap();

        let ciphertext = cipher
            .encrypt(CLEARTEXT.to_vec(), None, Some(aad.clone()))
            .unwrap();
        let cleartext = cipher.decrypt(ciphertext, Some(aad.clone())).unwrap();
        assert_eq!(cleartext, CLEARTEXT.to_vec());
    }

    #[test]
    fn encrypt_and_decrypt_with_file_works() {
        let cipher = RustyChaCha20Poly1305::create_internal(None, None).unwrap();
        let file_path = "test_file_1.bin".to_string();

        cipher
            .encrypt_to_file(CLEARTEXT.to_vec(), file_path.clone(), None, None)
            .unwrap();
        let cleartext = cipher.decrypt_from_file(file_path, None).unwrap();
        assert_eq!(cleartext, CLEARTEXT.to_vec());
    }

    #[test]
    fn decrypt_fails_with_wrong_key() {
        let cipher1 = RustyChaCha20Poly1305::create_internal(None, None).unwrap();
        let cipher2 = RustyChaCha20Poly1305::create_internal(None, None).unwrap();

        let ciphertext = cipher1.encrypt(CLEARTEXT.to_vec(), None, None).unwrap();
        let result = cipher2.decrypt(ciphertext, None);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn decrypt_fails_with_different_aad() {
        let cipher = RustyChaCha20Poly1305::create_internal(None, None).unwrap();
        let aad = vec![4, 5, 6];
        let aad2 = vec![7, 8, 9];

        let ciphertext = cipher
            .encrypt(CLEARTEXT.to_vec(), None, Some(aad.clone()))
            .unwrap();
        let result = cipher.decrypt(ciphertext, Some(aad2.clone()));
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn decrypt_fails_with_short_ciphertext() {
        let cipher = RustyChaCha20Poly1305::create_internal(None, None).unwrap();
        let short_ciphertext = vec![1, 2, 3];

        let result = cipher.decrypt(short_ciphertext, None);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn encrypt_fails_with_invalid_key() {
        let invalid_key = vec![1, 2, 3, 4, 5, 6, 7];
        let cipher = RustyChaCha20Poly1305::create_internal(Some(invalid_key), None);
        assert_eq!(cipher.is_err(), true);
    }

    #[test]
    fn compression_decompression_works() {
        // create some compressible data
        let mut original_cleartext = vec![0; 10000];
        original_cleartext[123] = 255;
        original_cleartext[5000] = 22;
        original_cleartext[7004] = 54;

        // test different compression levels
        for i in -10..21 {
            let cipher = RustyChaCha20Poly1305::create_internal(
                None,
                Some(Compression::Zstd {
                    compression_level: i,
                }),
            )
            .unwrap();
            let ciphertext = cipher
                .encrypt(original_cleartext.clone(), None, None)
                .unwrap();
            assert_eq!(ciphertext.len() < original_cleartext.len(), true); // should get smaller
            let cleartext = cipher.decrypt(ciphertext, None).unwrap();
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

        let file_path = "test_file_2.bin".to_string();

        // test different compression levels
        for i in -10..21 {
            let cipher = RustyChaCha20Poly1305::create_internal(
                None,
                Some(Compression::Zstd {
                    compression_level: i,
                }),
            )
            .unwrap();
            cipher
                .encrypt_to_file(original_cleartext.clone(), file_path.clone(), None, None)
                .unwrap();
            let cleartext = cipher.decrypt_from_file(file_path.clone(), None).unwrap();
            assert_eq!(cleartext, original_cleartext);
        }
    }
}

#[cfg(test)]
mod tests2 {
    use crate::api::{Compression, RustyXChaCha20Poly1305};

    const CLEARTEXT: &[u8] = &[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
        12, 13, 14, 15, 16, 42, 42, 42, 42, 42, 42, 42, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
    ];

    #[test]
    fn encrypt_and_decrypt_works() {
        let cipher = RustyXChaCha20Poly1305::create_internal(None, None).unwrap();

        let ciphertext = cipher.encrypt(CLEARTEXT.to_vec(), None, None).unwrap();
        assert_eq!(ciphertext.len(), CLEARTEXT.len() + 24 + 16); // 24 bytes nonce + 16 bytes MAC
        let cleartext = cipher.decrypt(ciphertext, None).unwrap();
        assert_eq!(cleartext, CLEARTEXT.to_vec());

        let key = RustyXChaCha20Poly1305::generate_key();
        let cipher1 = RustyXChaCha20Poly1305::create_internal(Some(key.clone()), None).unwrap();
        let cipher2 = RustyXChaCha20Poly1305::create_internal(Some(key.clone()), None).unwrap();

        let ciphertext = cipher1.encrypt(CLEARTEXT.to_vec(), None, None).unwrap();
        assert_eq!(ciphertext.len(), CLEARTEXT.len() + 24 + 16); // 24 bytes nonce + 16 bytes MAC
        let cleartext = cipher2.decrypt(ciphertext, None).unwrap();
        assert_eq!(cleartext, CLEARTEXT.to_vec());
    }

    #[test]
    fn encrypt_and_decrypt_with_nonce_works() {
        let nonce = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        ];
        let cipher = RustyXChaCha20Poly1305::create_internal(None, None).unwrap();

        let ciphertext = cipher
            .encrypt(CLEARTEXT.to_vec(), Some(nonce), None)
            .unwrap();
        assert_eq!(ciphertext.len(), CLEARTEXT.len() + 24 + 16); // 24 bytes nonce + 16 bytes MAC
        let cleartext = cipher.decrypt(ciphertext, None).unwrap();
        assert_eq!(cleartext, CLEARTEXT.to_vec());
    }

    #[test]
    fn encrypt_and_decrypt_with_aad_works() {
        let aad = vec![4, 5, 6];
        let cipher = RustyXChaCha20Poly1305::create_internal(None, None).unwrap();

        let ciphertext = cipher
            .encrypt(CLEARTEXT.to_vec(), None, Some(aad.clone()))
            .unwrap();
        let cleartext = cipher.decrypt(ciphertext, Some(aad.clone())).unwrap();
        assert_eq!(cleartext, CLEARTEXT.to_vec());
    }

    #[test]
    fn encrypt_and_decrypt_with_file_works() {
        let cipher = RustyXChaCha20Poly1305::create_internal(None, None).unwrap();
        let file_path = "test_file_3.bin".to_string();

        cipher
            .encrypt_to_file(CLEARTEXT.to_vec(), file_path.clone(), None, None)
            .unwrap();
        let cleartext = cipher.decrypt_from_file(file_path, None).unwrap();
        assert_eq!(cleartext, CLEARTEXT.to_vec());
    }

    #[test]
    fn decrypt_fails_with_wrong_key() {
        let cipher1 = RustyXChaCha20Poly1305::create_internal(None, None).unwrap();
        let cipher2 = RustyXChaCha20Poly1305::create_internal(None, None).unwrap();

        let ciphertext = cipher1.encrypt(CLEARTEXT.to_vec(), None, None).unwrap();
        let result = cipher2.decrypt(ciphertext, None);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn decrypt_fails_with_different_aad() {
        let cipher = RustyXChaCha20Poly1305::create_internal(None, None).unwrap();
        let aad = vec![4, 5, 6];
        let aad2 = vec![7, 8, 9];

        let ciphertext = cipher
            .encrypt(CLEARTEXT.to_vec(), None, Some(aad.clone()))
            .unwrap();
        let result = cipher.decrypt(ciphertext, Some(aad2.clone()));
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn decrypt_fails_with_short_ciphertext() {
        let cipher = RustyXChaCha20Poly1305::create_internal(None, None).unwrap();
        let short_ciphertext = vec![1, 2, 3];

        let result = cipher.decrypt(short_ciphertext, None);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn encrypt_fails_with_invalid_key() {
        let invalid_key = vec![1, 2, 3, 4, 5, 6, 7];
        let cipher = RustyXChaCha20Poly1305::create_internal(Some(invalid_key), None);
        assert_eq!(cipher.is_err(), true);
    }

    #[test]
    fn compression_decompression_works() {
        // create some compressible data
        let mut original_cleartext = vec![0; 10000];
        original_cleartext[123] = 255;
        original_cleartext[5000] = 22;
        original_cleartext[7004] = 54;

        // test different compression levels
        for i in -10..21 {
            let cipher = RustyXChaCha20Poly1305::create_internal(
                None,
                Some(Compression::Zstd {
                    compression_level: i,
                }),
            )
            .unwrap();
            let ciphertext = cipher
                .encrypt(original_cleartext.clone(), None, None)
                .unwrap();
            assert_eq!(ciphertext.len() < original_cleartext.len(), true); // should get smaller
            let cleartext = cipher.decrypt(ciphertext, None).unwrap();
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

        let file_path = "test_file_4.bin".to_string();

        // test different compression levels
        for i in -10..21 {
            let cipher = RustyXChaCha20Poly1305::create_internal(
                None,
                Some(Compression::Zstd {
                    compression_level: i,
                }),
            )
            .unwrap();
            cipher
                .encrypt_to_file(original_cleartext.clone(), file_path.clone(), None, None)
                .unwrap();
            let cleartext = cipher.decrypt_from_file(file_path.clone(), None).unwrap();
            assert_eq!(cleartext, original_cleartext);
        }
    }
}
