import 'dart:io' as io;

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

export 'src/api.dart';
export 'src/frb_generated.dart';

/// Fast ChaCha20-Poly1305 and XChaCha20-Poly1305 encryption powered by Rust
class RustyChaCha {
  static bool _initialized = false;

  /// Creates a `RustyChaCha20Poly1305` cipher asynchronously using the provided configuration.
  ///
  /// It uses the ChaCha20-Poly1305 algorithm
  /// for encryption, optionally incorporating the specified compression method.
  ///
  /// **Parameters:**
  /// - `key`: An optional `Uint8List` containing the encryption key (32 bytes). If provided, it will be used
  ///   as the key for the encryption algorithm. If null, a random key will be generated.
  /// - `compression`: An optional `Compression` enum value specifying the compression method to
  ///   use before encryption. If null or Compression.uncompressed(), no compression is applied.
  ///   Compression.zstd(compressionLevel: 0) will apply a default amount of compression (before encryption).
  ///   Compression.zstd(compressionLevel: 19) will apply a high amount of compression (slower).
  ///   Important: Data encrypted with Compression.zstd can only be decrypted correctly if the cipher instance used
  ///   during decryption also has Compression.zstd enabled (the level does not mappter)
  ///
  ///   See zstd manual for more info regarding compressionLevel: https://facebook.github.io/zstd/zstd_manual.html
  ///
  /// **Returns:**
  /// A `Future<RustyChaCha20Poly1305>` that completes with a new instance of `RustyChaCha20Poly1305`
  /// configured with the given key and compression settings.
  ///
  /// **Example Usage:**
  /// ```dart
  /// final cipher = await RustyChaCha20Poly1305.create(
  ///   key: mySecureKey,
  ///   compression: Compression.uncompressed(),
  /// );
  /// ```
  static Future<RustyChaCha20Poly1305> create({
    Uint8List? key,
    Compression? compression,
  }) async {
    await _ensureInitialized();
    return RustyChaCha20Poly1305.createInternal(
        key: key, compression: compression);
  }

  /// Creates a `XRustyChaCha20Poly1305` cipher asynchronously using the provided configuration.
  ///
  /// It uses the XChaCha20-Poly1305 algorithm with an extended 192-bit (24-byte) nonce
  /// for encryption, optionally incorporating the specified compression method.
  ///
  /// **Parameters:**
  /// - `key`: An optional `Uint8List` containing the encryption key (32 bytes). If provided, it will be used
  ///   as the key for the encryption algorithm. If null, a random key will be generated.
  /// - `compression`: An optional `Compression` enum value specifying the compression method to
  ///   use before encryption. If null or Compression.uncompressed(), no compression is applied.
  ///   Compression.zstd() will apply a default amount of compression before encryption.
  ///   Compression.zstd(compressionLevel: 19) will apply a high amount of compression (slower).
  ///   Important: Data encrypted with Compression.zstd can only be decrypted correctly if the cipher instance used
  ///   during decryption also has Compression.zstd enabled.
  ///
  ///   See zstd manual for more info regarding compressionLevel: https://facebook.github.io/zstd/zstd_manual.html
  ///
  /// **Returns:**
  /// A `Future<XRustyChaCha20Poly1305>` that completes with a new instance of `XRustyChaCha20Poly1305`
  /// configured with the given key and compression settings.
  ///
  /// **Example Usage:**
  /// ```dart
  /// final cipher = await RustyChaCha20Poly1305.createX(
  ///   key: mySecureKey,
  ///   compression: Compression.uncompressed(),
  /// );
  /// ```
  static Future<RustyXChaCha20Poly1305> createX({
    Uint8List? key,
    Compression? compression,
  }) async {
    await _ensureInitialized();
    return RustyXChaCha20Poly1305.createInternal(
        key: key, compression: compression);
  }

  static Future<void> _ensureInitialized() async {
    if (_initialized) {
      return;
    }

    if (io.Platform.isIOS || io.Platform.isMacOS) {
      // TODO(brookman): Use dynamic linking for iOS and MacOS?
      final lib = ExternalLibrary.process(iKnowHowToUseIt: true);
      await RustLib.init(externalLibrary: lib);
    } else {
      await RustLib.init();
    }

    _initialized = true;
  }
}
