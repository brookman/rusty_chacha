<p align="center">
<a href="https://github.com/brookman/rusty_chacha/actions"><img src="https://github.com/brookman/rusty_chacha/actions/workflows/build.yml/badge.svg" alt="Build Status"></a>
<a href="https://github.com/brookman/rusty_chacha"><img src="https://img.shields.io/github/stars/brookman/rusty_chacha.svg?style=flat&logo=github&colorB=deeppink&label=stars" alt="Github Stars"></a>
<a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/license-MIT-purple.svg" alt="MIT License"></a>
</p>

<p align="center">
<img src="https://github.com/brookman/rusty_chacha/blob/main/assets/banner.jpg?raw=true" width="100%" alt="Rusty ChaCha Banner" />
</p>

# Rusty ChaCha 💃🦀

A Flutter library for fast ChaCha20-Poly1305 encryption, leveraging the capabilities of the
Rust [chacha20poly1305](https://crates.io/crates/chacha20poly1305) crate.

🚧 Under Development: Not recommended for production use yet. 🚧

---

## Features

- Encrypt and decrypt data with ChaCha20-Poly1305 (authenticated)
- [Additional authenticated data (AAD)](https://en.wikipedia.org/wiki/Authenticated_encryption)
- Optional compression using [zstd](https://en.wikipedia.org/wiki/Zstd)

## Blazingly fast 🔥

Thanks to Rust encryption and decryption with ChaCha20-Poly1305 run at 500-1000 MiB/s.
This is up to **50x** faster than packages like [cryptography_flutter](https://pub.dev/packages/cryptography_flutter) or [pointycastle](https://pub.dev/packages/pointycastle).

## Getting Started

- With Flutter, run `flutter pub add rusty_chacha`

## Usage

```dart
import 'package:rusty_chacha/rusty_chacha.dart';

main() async {
  await RustyChaCha.init();


  final key = await generateChaCha20Key(); // generate a random key
  final myData = Uint8List.fromList([1, 2, 3, 4, 5]);


  // basic example:
  final myEncryptedData = await encrypt(key: key, cleartext: myData);
  final myDataAgain1 = await decrypt(key: key, ciphertext: myEncryptedData);


  // compression example:
  final myCompressedAndEncryptedData = await encrypt_compressed(
    key: key,
    zstdCompressionLevel: 3, // moderate compression
    cleartext: myData,
  );
  final myDataAgain2 = await decrypt_compressed(key: key, ciphertext: myCompressedAndEncryptedData);


  // AAD example:
  final additionalData = Uint8List.fromList([1, 2, 3]); // some additional (non-secret) data
  final myEncryptedDataWithAad = await encrypt(
    key: key,
    cleartext: myData,
    aad: additionalData, // pass it in when encrypting
  );
  final myDataAgain3 = await decrypt(
    key: key,
    ciphertext: myEncryptedDataWithAad,
    aad: additionalData, // pass it in to decrypt (decrypt will fail if additionalData is not the same)
  );
}
```

## Supported platforms (for now)

- iOS
- MacOS
- Windows