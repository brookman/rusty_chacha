// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.39.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'api.freezed.dart';

/// public for benchmarking
Future<Uint8List> compress(
        {required List<int> data, required int zstdCompressionLevel}) =>
    RustLib.instance.api.crateApiCompress(
        data: data, zstdCompressionLevel: zstdCompressionLevel);

/// public for benchmarking
Future<Uint8List> decompress({required List<int> data}) =>
    RustLib.instance.api.crateApiDecompress(data: data);

@freezed
sealed class Compression with _$Compression {
  const Compression._();

  const factory Compression.uncompressed() = Compression_Uncompressed;
  const factory Compression.zstd({
    int? compressionLevel,
  }) = Compression_Zstd;
}

class RustyChaCha20Poly1305 {
  final Uint8List key;
  final Compression compression;

  const RustyChaCha20Poly1305({
    required this.key,
    required this.compression,
  });

  /// Do not use this. Use `create()` instead.
  static Future<RustyChaCha20Poly1305> createInternal(
          {Uint8List? key, Compression? compression}) =>
      RustLib.instance.api.crateApiRustyChaCha20Poly1305CreateInternal(
          key: key, compression: compression);

  /// Decrypts `ciphertext` and returns the cleartext.
  /// The first NONCE_LEN bytes of the `ciphertext` must contain the nonce.
  Future<Uint8List> decrypt({required List<int> ciphertext, Uint8List? aad}) =>
      RustLib.instance.api.crateApiRustyChaCha20Poly1305Decrypt(
          that: this, ciphertext: ciphertext, aad: aad);

  /// Reads `file_path` decrypts the contents and returns the cleartext.
  /// The first NONCE_LEN bytes of the `ciphertext` must contain the nonce.
  Future<Uint8List> decryptFromFile(
          {required String filePath, Uint8List? aad, BigInt? offset}) =>
      RustLib.instance.api.crateApiRustyChaCha20Poly1305DecryptFromFile(
          that: this, filePath: filePath, aad: aad, offset: offset);

  /// Encrypts `cleartext` and returns the ciphertext.
  /// If no `nonce` is given, a random one will be generated.
  /// The nonce is always prepended to the ciphertext (first NONCE_LEN bytes).
  Future<Uint8List> encrypt(
          {required List<int> cleartext, Uint8List? nonce, Uint8List? aad}) =>
      RustLib.instance.api.crateApiRustyChaCha20Poly1305Encrypt(
          that: this, cleartext: cleartext, nonce: nonce, aad: aad);

  /// Encrypts `cleartext` and write the result to `file_path`.
  /// If no `nonce` is given, a randomly generated one will be generated.
  /// The nonce is always prepended to the result (first NONCE_LEN bytes).
  Future<void> encryptToFile(
          {required List<int> cleartext,
          required String filePath,
          Uint8List? nonce,
          Uint8List? aad,
          bool? append}) =>
      RustLib.instance.api.crateApiRustyChaCha20Poly1305EncryptToFile(
          that: this,
          cleartext: cleartext,
          filePath: filePath,
          nonce: nonce,
          aad: aad,
          append: append);

  static Future<Uint8List> generateKey() =>
      RustLib.instance.api.crateApiRustyChaCha20Poly1305GenerateKey();

  /// Important: A nonce must only be used once.
  /// Do not encrypt multiple pieces of data with the same nonce or the key is compromised.
  static Future<Uint8List> generateNonce() =>
      RustLib.instance.api.crateApiRustyChaCha20Poly1305GenerateNonce();

  @override
  int get hashCode => key.hashCode ^ compression.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RustyChaCha20Poly1305 &&
          runtimeType == other.runtimeType &&
          key == other.key &&
          compression == other.compression;
}

class RustyXChaCha20Poly1305 {
  final Uint8List key;
  final Compression compression;

  const RustyXChaCha20Poly1305({
    required this.key,
    required this.compression,
  });

  /// Do not use this. Use `create()` instead.
  static Future<RustyXChaCha20Poly1305> createInternal(
          {Uint8List? key, Compression? compression}) =>
      RustLib.instance.api.crateApiRustyXChaCha20Poly1305CreateInternal(
          key: key, compression: compression);

  /// Decrypts `ciphertext` and returns the cleartext.
  /// The first NONCE_LEN bytes of the `ciphertext` must contain the nonce.
  Future<Uint8List> decrypt({required List<int> ciphertext, Uint8List? aad}) =>
      RustLib.instance.api.crateApiRustyXChaCha20Poly1305Decrypt(
          that: this, ciphertext: ciphertext, aad: aad);

  /// Reads `file_path` decrypts the contents and returns the cleartext.
  /// The first NONCE_LEN bytes of the `ciphertext` must contain the nonce.
  Future<Uint8List> decryptFromFile(
          {required String filePath, Uint8List? aad, BigInt? offset}) =>
      RustLib.instance.api.crateApiRustyXChaCha20Poly1305DecryptFromFile(
          that: this, filePath: filePath, aad: aad, offset: offset);

  /// Encrypts `cleartext` and returns the ciphertext.
  /// If no `nonce` is given, a random one will be generated.
  /// The nonce is always prepended to the ciphertext (first NONCE_LEN bytes).
  Future<Uint8List> encrypt(
          {required List<int> cleartext, Uint8List? nonce, Uint8List? aad}) =>
      RustLib.instance.api.crateApiRustyXChaCha20Poly1305Encrypt(
          that: this, cleartext: cleartext, nonce: nonce, aad: aad);

  /// Encrypts `cleartext` and write the result to `file_path`.
  /// If no `nonce` is given, a randomly generated one will be generated.
  /// The nonce is always prepended to the result (first NONCE_LEN bytes).
  Future<void> encryptToFile(
          {required List<int> cleartext,
          required String filePath,
          Uint8List? nonce,
          Uint8List? aad,
          bool? append}) =>
      RustLib.instance.api.crateApiRustyXChaCha20Poly1305EncryptToFile(
          that: this,
          cleartext: cleartext,
          filePath: filePath,
          nonce: nonce,
          aad: aad,
          append: append);

  static Future<Uint8List> generateKey() =>
      RustLib.instance.api.crateApiRustyXChaCha20Poly1305GenerateKey();

  /// Important: A nonce must only be used once.
  /// Do not encrypt multiple pieces of data with the same nonce.
  static Future<Uint8List> generateNonce() =>
      RustLib.instance.api.crateApiRustyXChaCha20Poly1305GenerateNonce();

  @override
  int get hashCode => key.hashCode ^ compression.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RustyXChaCha20Poly1305 &&
          runtimeType == other.runtimeType &&
          key == other.key &&
          compression == other.compression;
}