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
import 'package:collection/collection.dart';
import 'package:rusty_chacha/rusty_chacha.dart';

main() async {
  
  final data = Uint8List.fromList([1, 2, 3, 4, 5]);


  // Create and use a ChaCha20Poly1305 cipher with a random key:
  RustyChaCha20Poly1305 cipher = await RustyChaCha.createChaCha20Poly1305();
  Uint8List myEncryptedData = await cipher.encrypt(cleartext: data);
  Uint8List decrypted1 = await cipher.decrypt(ciphertext: myEncryptedData);

  assert(const ListEquality().equals(data, decrypted1));


  // Or with explicit key:
  Uint8List key = await RustyChaCha20Poly1305.generateKey();
  cipher = await RustyChaCha.createChaCha20Poly1305(key: key);


  // Compression example:
  // If compression is used during encryption, it also has to be set for decryption!
  // However compressionLevel does not matter.
  cipher = await RustyChaCha.createChaCha20Poly1305(
    key: key,
    compression: const Compression.zstd(compressionLevel: 3), // moderate compression
  );
  Uint8List myCompressedAndEncryptedData = await cipher.encrypt(cleartext: data);
  Uint8List decrypted2 = await cipher.decrypt(ciphertext: myCompressedAndEncryptedData);

  assert(const ListEquality().equals(data, decrypted2));


  // AAD example:
  cipher = await RustyChaCha.createChaCha20Poly1305();
  Uint8List additionalData = Uint8List.fromList([1, 2, 3]); // some additional (non-secret) data
  Uint8List myEncryptedDataWithAad = await cipher.encrypt(
    cleartext: data,
    aad: additionalData, // pass it in when encrypting
  );
  Uint8List decrypted3 = await cipher.decrypt(
    ciphertext: myEncryptedDataWithAad,
    aad: additionalData, // pass it in to decrypt (decrypt will fail if additionalData is not the same)
  );

  assert(const ListEquality().equals(data, decrypted3));

  // Create and use a XChaCha20Poly1305 (24 byte nonce instead of 12) cipher with a random key:
  RustyXChaCha20Poly1305 cipherX = await RustyChaCha.createXChaCha20Poly1305();
}
```

## Supported platforms (for now)

- iOS
- MacOS
- Windows