// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.39.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables, unused_field

import 'api.dart';
import 'dart:async';
import 'dart:convert';
import 'frb_generated.dart';
import 'frb_generated.io.dart' if (dart.library.html) 'frb_generated.web.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

/// Main entrypoint of the Rust API
class RustLib extends BaseEntrypoint<RustLibApi, RustLibApiImpl, RustLibWire> {
  @internal
  static final instance = RustLib._();

  RustLib._();

  /// Initialize flutter_rust_bridge
  static Future<void> init({
    RustLibApi? api,
    BaseHandler? handler,
    ExternalLibrary? externalLibrary,
  }) async {
    await instance.initImpl(
      api: api,
      handler: handler,
      externalLibrary: externalLibrary,
    );
  }

  /// Dispose flutter_rust_bridge
  ///
  /// The call to this function is optional, since flutter_rust_bridge (and everything else)
  /// is automatically disposed when the app stops.
  static void dispose() => instance.disposeImpl();

  @override
  ApiImplConstructor<RustLibApiImpl, RustLibWire> get apiImplConstructor =>
      RustLibApiImpl.new;

  @override
  WireConstructor<RustLibWire> get wireConstructor =>
      RustLibWire.fromExternalLibrary;

  @override
  Future<void> executeRustInitializers() async {}

  @override
  ExternalLibraryLoaderConfig get defaultExternalLibraryLoaderConfig =>
      kDefaultExternalLibraryLoaderConfig;

  @override
  String get codegenVersion => '2.0.0-dev.39';

  @override
  int get rustContentHash => 1378991708;

  static const kDefaultExternalLibraryLoaderConfig =
      ExternalLibraryLoaderConfig(
    stem: 'embedded_rusty_chacha',
    ioDirectory: 'rust/target/release/',
    webPrefix: 'pkg/',
  );
}

abstract class RustLibApi extends BaseApi {
  Future<Uint8List> crateApiCompress(
      {required List<int> data, required int zstdCompressionLevel});

  Future<Uint8List> crateApiDecompress({required List<int> data});

  Future<RustyChaCha20Poly1305> crateApiRustyChaCha20Poly1305CreateInternal(
      {Uint8List? key, Compression? compression});

  Future<Uint8List> crateApiRustyChaCha20Poly1305Decrypt(
      {required RustyChaCha20Poly1305 that,
      required List<int> ciphertext,
      Uint8List? aad});

  Future<Uint8List> crateApiRustyChaCha20Poly1305DecryptFromFile(
      {required RustyChaCha20Poly1305 that,
      required String filePath,
      Uint8List? aad,
      BigInt? offset});

  Future<Uint8List> crateApiRustyChaCha20Poly1305Encrypt(
      {required RustyChaCha20Poly1305 that,
      required List<int> cleartext,
      Uint8List? nonce,
      Uint8List? aad});

  Future<void> crateApiRustyChaCha20Poly1305EncryptToFile(
      {required RustyChaCha20Poly1305 that,
      required List<int> cleartext,
      required String filePath,
      Uint8List? nonce,
      Uint8List? aad,
      bool? append});

  Future<Uint8List> crateApiRustyChaCha20Poly1305GenerateKey();

  Future<Uint8List> crateApiRustyChaCha20Poly1305GenerateNonce();

  Future<RustyXChaCha20Poly1305> crateApiRustyXChaCha20Poly1305CreateInternal(
      {Uint8List? key, Compression? compression});

  Future<Uint8List> crateApiRustyXChaCha20Poly1305Decrypt(
      {required RustyXChaCha20Poly1305 that,
      required List<int> ciphertext,
      Uint8List? aad});

  Future<Uint8List> crateApiRustyXChaCha20Poly1305DecryptFromFile(
      {required RustyXChaCha20Poly1305 that,
      required String filePath,
      Uint8List? aad,
      BigInt? offset});

  Future<Uint8List> crateApiRustyXChaCha20Poly1305Encrypt(
      {required RustyXChaCha20Poly1305 that,
      required List<int> cleartext,
      Uint8List? nonce,
      Uint8List? aad});

  Future<void> crateApiRustyXChaCha20Poly1305EncryptToFile(
      {required RustyXChaCha20Poly1305 that,
      required List<int> cleartext,
      required String filePath,
      Uint8List? nonce,
      Uint8List? aad,
      bool? append});

  Future<Uint8List> crateApiRustyXChaCha20Poly1305GenerateKey();

  Future<Uint8List> crateApiRustyXChaCha20Poly1305GenerateNonce();
}

class RustLibApiImpl extends RustLibApiImplPlatform implements RustLibApi {
  RustLibApiImpl({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  @override
  Future<Uint8List> crateApiCompress(
      {required List<int> data, required int zstdCompressionLevel}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        var arg0 = cst_encode_list_prim_u_8_loose(data);
        var arg1 = cst_encode_i_32(zstdCompressionLevel);
        return wire.wire__crate__api__compress(port_, arg0, arg1);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_list_prim_u_8_strict,
        decodeErrorData: dco_decode_AnyhowException,
      ),
      constMeta: kCrateApiCompressConstMeta,
      argValues: [data, zstdCompressionLevel],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiCompressConstMeta => const TaskConstMeta(
        debugName: "compress",
        argNames: ["data", "zstdCompressionLevel"],
      );

  @override
  Future<Uint8List> crateApiDecompress({required List<int> data}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        var arg0 = cst_encode_list_prim_u_8_loose(data);
        return wire.wire__crate__api__decompress(port_, arg0);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_list_prim_u_8_strict,
        decodeErrorData: dco_decode_AnyhowException,
      ),
      constMeta: kCrateApiDecompressConstMeta,
      argValues: [data],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiDecompressConstMeta => const TaskConstMeta(
        debugName: "decompress",
        argNames: ["data"],
      );

  @override
  Future<RustyChaCha20Poly1305> crateApiRustyChaCha20Poly1305CreateInternal(
      {Uint8List? key, Compression? compression}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        var arg0 = cst_encode_opt_list_prim_u_8_strict(key);
        var arg1 = cst_encode_opt_box_autoadd_compression(compression);
        return wire
            .wire__crate__api__rusty_cha_cha_20_poly_1305_create_internal(
                port_, arg0, arg1);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_rusty_cha_cha_20_poly_1305,
        decodeErrorData: dco_decode_AnyhowException,
      ),
      constMeta: kCrateApiRustyChaCha20Poly1305CreateInternalConstMeta,
      argValues: [key, compression],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiRustyChaCha20Poly1305CreateInternalConstMeta =>
      const TaskConstMeta(
        debugName: "rusty_cha_cha_20_poly_1305_create_internal",
        argNames: ["key", "compression"],
      );

  @override
  Future<Uint8List> crateApiRustyChaCha20Poly1305Decrypt(
      {required RustyChaCha20Poly1305 that,
      required List<int> ciphertext,
      Uint8List? aad}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        var arg0 = cst_encode_box_autoadd_rusty_cha_cha_20_poly_1305(that);
        var arg1 = cst_encode_list_prim_u_8_loose(ciphertext);
        var arg2 = cst_encode_opt_list_prim_u_8_strict(aad);
        return wire.wire__crate__api__rusty_cha_cha_20_poly_1305_decrypt(
            port_, arg0, arg1, arg2);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_list_prim_u_8_strict,
        decodeErrorData: dco_decode_AnyhowException,
      ),
      constMeta: kCrateApiRustyChaCha20Poly1305DecryptConstMeta,
      argValues: [that, ciphertext, aad],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiRustyChaCha20Poly1305DecryptConstMeta =>
      const TaskConstMeta(
        debugName: "rusty_cha_cha_20_poly_1305_decrypt",
        argNames: ["that", "ciphertext", "aad"],
      );

  @override
  Future<Uint8List> crateApiRustyChaCha20Poly1305DecryptFromFile(
      {required RustyChaCha20Poly1305 that,
      required String filePath,
      Uint8List? aad,
      BigInt? offset}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        var arg0 = cst_encode_box_autoadd_rusty_cha_cha_20_poly_1305(that);
        var arg1 = cst_encode_String(filePath);
        var arg2 = cst_encode_opt_list_prim_u_8_strict(aad);
        var arg3 = cst_encode_opt_box_autoadd_u_64(offset);
        return wire
            .wire__crate__api__rusty_cha_cha_20_poly_1305_decrypt_from_file(
                port_, arg0, arg1, arg2, arg3);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_list_prim_u_8_strict,
        decodeErrorData: dco_decode_AnyhowException,
      ),
      constMeta: kCrateApiRustyChaCha20Poly1305DecryptFromFileConstMeta,
      argValues: [that, filePath, aad, offset],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiRustyChaCha20Poly1305DecryptFromFileConstMeta =>
      const TaskConstMeta(
        debugName: "rusty_cha_cha_20_poly_1305_decrypt_from_file",
        argNames: ["that", "filePath", "aad", "offset"],
      );

  @override
  Future<Uint8List> crateApiRustyChaCha20Poly1305Encrypt(
      {required RustyChaCha20Poly1305 that,
      required List<int> cleartext,
      Uint8List? nonce,
      Uint8List? aad}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        var arg0 = cst_encode_box_autoadd_rusty_cha_cha_20_poly_1305(that);
        var arg1 = cst_encode_list_prim_u_8_loose(cleartext);
        var arg2 = cst_encode_opt_list_prim_u_8_strict(nonce);
        var arg3 = cst_encode_opt_list_prim_u_8_strict(aad);
        return wire.wire__crate__api__rusty_cha_cha_20_poly_1305_encrypt(
            port_, arg0, arg1, arg2, arg3);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_list_prim_u_8_strict,
        decodeErrorData: dco_decode_AnyhowException,
      ),
      constMeta: kCrateApiRustyChaCha20Poly1305EncryptConstMeta,
      argValues: [that, cleartext, nonce, aad],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiRustyChaCha20Poly1305EncryptConstMeta =>
      const TaskConstMeta(
        debugName: "rusty_cha_cha_20_poly_1305_encrypt",
        argNames: ["that", "cleartext", "nonce", "aad"],
      );

  @override
  Future<void> crateApiRustyChaCha20Poly1305EncryptToFile(
      {required RustyChaCha20Poly1305 that,
      required List<int> cleartext,
      required String filePath,
      Uint8List? nonce,
      Uint8List? aad,
      bool? append}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        var arg0 = cst_encode_box_autoadd_rusty_cha_cha_20_poly_1305(that);
        var arg1 = cst_encode_list_prim_u_8_loose(cleartext);
        var arg2 = cst_encode_String(filePath);
        var arg3 = cst_encode_opt_list_prim_u_8_strict(nonce);
        var arg4 = cst_encode_opt_list_prim_u_8_strict(aad);
        var arg5 = cst_encode_opt_box_autoadd_bool(append);
        return wire
            .wire__crate__api__rusty_cha_cha_20_poly_1305_encrypt_to_file(
                port_, arg0, arg1, arg2, arg3, arg4, arg5);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_unit,
        decodeErrorData: dco_decode_AnyhowException,
      ),
      constMeta: kCrateApiRustyChaCha20Poly1305EncryptToFileConstMeta,
      argValues: [that, cleartext, filePath, nonce, aad, append],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiRustyChaCha20Poly1305EncryptToFileConstMeta =>
      const TaskConstMeta(
        debugName: "rusty_cha_cha_20_poly_1305_encrypt_to_file",
        argNames: ["that", "cleartext", "filePath", "nonce", "aad", "append"],
      );

  @override
  Future<Uint8List> crateApiRustyChaCha20Poly1305GenerateKey() {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        return wire
            .wire__crate__api__rusty_cha_cha_20_poly_1305_generate_key(port_);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_list_prim_u_8_strict,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiRustyChaCha20Poly1305GenerateKeyConstMeta,
      argValues: [],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiRustyChaCha20Poly1305GenerateKeyConstMeta =>
      const TaskConstMeta(
        debugName: "rusty_cha_cha_20_poly_1305_generate_key",
        argNames: [],
      );

  @override
  Future<Uint8List> crateApiRustyChaCha20Poly1305GenerateNonce() {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        return wire
            .wire__crate__api__rusty_cha_cha_20_poly_1305_generate_nonce(port_);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_list_prim_u_8_strict,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiRustyChaCha20Poly1305GenerateNonceConstMeta,
      argValues: [],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiRustyChaCha20Poly1305GenerateNonceConstMeta =>
      const TaskConstMeta(
        debugName: "rusty_cha_cha_20_poly_1305_generate_nonce",
        argNames: [],
      );

  @override
  Future<RustyXChaCha20Poly1305> crateApiRustyXChaCha20Poly1305CreateInternal(
      {Uint8List? key, Compression? compression}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        var arg0 = cst_encode_opt_list_prim_u_8_strict(key);
        var arg1 = cst_encode_opt_box_autoadd_compression(compression);
        return wire
            .wire__crate__api__rusty_x_cha_cha_20_poly_1305_create_internal(
                port_, arg0, arg1);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_rusty_x_cha_cha_20_poly_1305,
        decodeErrorData: dco_decode_AnyhowException,
      ),
      constMeta: kCrateApiRustyXChaCha20Poly1305CreateInternalConstMeta,
      argValues: [key, compression],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiRustyXChaCha20Poly1305CreateInternalConstMeta =>
      const TaskConstMeta(
        debugName: "rusty_x_cha_cha_20_poly_1305_create_internal",
        argNames: ["key", "compression"],
      );

  @override
  Future<Uint8List> crateApiRustyXChaCha20Poly1305Decrypt(
      {required RustyXChaCha20Poly1305 that,
      required List<int> ciphertext,
      Uint8List? aad}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        var arg0 = cst_encode_box_autoadd_rusty_x_cha_cha_20_poly_1305(that);
        var arg1 = cst_encode_list_prim_u_8_loose(ciphertext);
        var arg2 = cst_encode_opt_list_prim_u_8_strict(aad);
        return wire.wire__crate__api__rusty_x_cha_cha_20_poly_1305_decrypt(
            port_, arg0, arg1, arg2);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_list_prim_u_8_strict,
        decodeErrorData: dco_decode_AnyhowException,
      ),
      constMeta: kCrateApiRustyXChaCha20Poly1305DecryptConstMeta,
      argValues: [that, ciphertext, aad],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiRustyXChaCha20Poly1305DecryptConstMeta =>
      const TaskConstMeta(
        debugName: "rusty_x_cha_cha_20_poly_1305_decrypt",
        argNames: ["that", "ciphertext", "aad"],
      );

  @override
  Future<Uint8List> crateApiRustyXChaCha20Poly1305DecryptFromFile(
      {required RustyXChaCha20Poly1305 that,
      required String filePath,
      Uint8List? aad,
      BigInt? offset}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        var arg0 = cst_encode_box_autoadd_rusty_x_cha_cha_20_poly_1305(that);
        var arg1 = cst_encode_String(filePath);
        var arg2 = cst_encode_opt_list_prim_u_8_strict(aad);
        var arg3 = cst_encode_opt_box_autoadd_u_64(offset);
        return wire
            .wire__crate__api__rusty_x_cha_cha_20_poly_1305_decrypt_from_file(
                port_, arg0, arg1, arg2, arg3);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_list_prim_u_8_strict,
        decodeErrorData: dco_decode_AnyhowException,
      ),
      constMeta: kCrateApiRustyXChaCha20Poly1305DecryptFromFileConstMeta,
      argValues: [that, filePath, aad, offset],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiRustyXChaCha20Poly1305DecryptFromFileConstMeta =>
      const TaskConstMeta(
        debugName: "rusty_x_cha_cha_20_poly_1305_decrypt_from_file",
        argNames: ["that", "filePath", "aad", "offset"],
      );

  @override
  Future<Uint8List> crateApiRustyXChaCha20Poly1305Encrypt(
      {required RustyXChaCha20Poly1305 that,
      required List<int> cleartext,
      Uint8List? nonce,
      Uint8List? aad}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        var arg0 = cst_encode_box_autoadd_rusty_x_cha_cha_20_poly_1305(that);
        var arg1 = cst_encode_list_prim_u_8_loose(cleartext);
        var arg2 = cst_encode_opt_list_prim_u_8_strict(nonce);
        var arg3 = cst_encode_opt_list_prim_u_8_strict(aad);
        return wire.wire__crate__api__rusty_x_cha_cha_20_poly_1305_encrypt(
            port_, arg0, arg1, arg2, arg3);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_list_prim_u_8_strict,
        decodeErrorData: dco_decode_AnyhowException,
      ),
      constMeta: kCrateApiRustyXChaCha20Poly1305EncryptConstMeta,
      argValues: [that, cleartext, nonce, aad],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiRustyXChaCha20Poly1305EncryptConstMeta =>
      const TaskConstMeta(
        debugName: "rusty_x_cha_cha_20_poly_1305_encrypt",
        argNames: ["that", "cleartext", "nonce", "aad"],
      );

  @override
  Future<void> crateApiRustyXChaCha20Poly1305EncryptToFile(
      {required RustyXChaCha20Poly1305 that,
      required List<int> cleartext,
      required String filePath,
      Uint8List? nonce,
      Uint8List? aad,
      bool? append}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        var arg0 = cst_encode_box_autoadd_rusty_x_cha_cha_20_poly_1305(that);
        var arg1 = cst_encode_list_prim_u_8_loose(cleartext);
        var arg2 = cst_encode_String(filePath);
        var arg3 = cst_encode_opt_list_prim_u_8_strict(nonce);
        var arg4 = cst_encode_opt_list_prim_u_8_strict(aad);
        var arg5 = cst_encode_opt_box_autoadd_bool(append);
        return wire
            .wire__crate__api__rusty_x_cha_cha_20_poly_1305_encrypt_to_file(
                port_, arg0, arg1, arg2, arg3, arg4, arg5);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_unit,
        decodeErrorData: dco_decode_AnyhowException,
      ),
      constMeta: kCrateApiRustyXChaCha20Poly1305EncryptToFileConstMeta,
      argValues: [that, cleartext, filePath, nonce, aad, append],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiRustyXChaCha20Poly1305EncryptToFileConstMeta =>
      const TaskConstMeta(
        debugName: "rusty_x_cha_cha_20_poly_1305_encrypt_to_file",
        argNames: ["that", "cleartext", "filePath", "nonce", "aad", "append"],
      );

  @override
  Future<Uint8List> crateApiRustyXChaCha20Poly1305GenerateKey() {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        return wire
            .wire__crate__api__rusty_x_cha_cha_20_poly_1305_generate_key(port_);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_list_prim_u_8_strict,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiRustyXChaCha20Poly1305GenerateKeyConstMeta,
      argValues: [],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiRustyXChaCha20Poly1305GenerateKeyConstMeta =>
      const TaskConstMeta(
        debugName: "rusty_x_cha_cha_20_poly_1305_generate_key",
        argNames: [],
      );

  @override
  Future<Uint8List> crateApiRustyXChaCha20Poly1305GenerateNonce() {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        return wire
            .wire__crate__api__rusty_x_cha_cha_20_poly_1305_generate_nonce(
                port_);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_list_prim_u_8_strict,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiRustyXChaCha20Poly1305GenerateNonceConstMeta,
      argValues: [],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiRustyXChaCha20Poly1305GenerateNonceConstMeta =>
      const TaskConstMeta(
        debugName: "rusty_x_cha_cha_20_poly_1305_generate_nonce",
        argNames: [],
      );

  @protected
  AnyhowException dco_decode_AnyhowException(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return AnyhowException(raw as String);
  }

  @protected
  String dco_decode_String(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as String;
  }

  @protected
  bool dco_decode_bool(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as bool;
  }

  @protected
  bool dco_decode_box_autoadd_bool(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as bool;
  }

  @protected
  Compression dco_decode_box_autoadd_compression(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return dco_decode_compression(raw);
  }

  @protected
  int dco_decode_box_autoadd_i_32(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as int;
  }

  @protected
  RustyChaCha20Poly1305 dco_decode_box_autoadd_rusty_cha_cha_20_poly_1305(
      dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return dco_decode_rusty_cha_cha_20_poly_1305(raw);
  }

  @protected
  RustyXChaCha20Poly1305 dco_decode_box_autoadd_rusty_x_cha_cha_20_poly_1305(
      dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return dco_decode_rusty_x_cha_cha_20_poly_1305(raw);
  }

  @protected
  BigInt dco_decode_box_autoadd_u_64(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return dco_decode_u_64(raw);
  }

  @protected
  Compression dco_decode_compression(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    switch (raw[0]) {
      case 0:
        return Compression_Uncompressed();
      case 1:
        return Compression_Zstd(
          compressionLevel: dco_decode_opt_box_autoadd_i_32(raw[1]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  @protected
  int dco_decode_i_32(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as int;
  }

  @protected
  List<int> dco_decode_list_prim_u_8_loose(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as List<int>;
  }

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as Uint8List;
  }

  @protected
  bool? dco_decode_opt_box_autoadd_bool(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw == null ? null : dco_decode_box_autoadd_bool(raw);
  }

  @protected
  Compression? dco_decode_opt_box_autoadd_compression(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw == null ? null : dco_decode_box_autoadd_compression(raw);
  }

  @protected
  int? dco_decode_opt_box_autoadd_i_32(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw == null ? null : dco_decode_box_autoadd_i_32(raw);
  }

  @protected
  BigInt? dco_decode_opt_box_autoadd_u_64(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw == null ? null : dco_decode_box_autoadd_u_64(raw);
  }

  @protected
  Uint8List? dco_decode_opt_list_prim_u_8_strict(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw == null ? null : dco_decode_list_prim_u_8_strict(raw);
  }

  @protected
  RustyChaCha20Poly1305 dco_decode_rusty_cha_cha_20_poly_1305(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return RustyChaCha20Poly1305(
      key: dco_decode_list_prim_u_8_strict(arr[0]),
      compression: dco_decode_compression(arr[1]),
    );
  }

  @protected
  RustyXChaCha20Poly1305 dco_decode_rusty_x_cha_cha_20_poly_1305(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return RustyXChaCha20Poly1305(
      key: dco_decode_list_prim_u_8_strict(arr[0]),
      compression: dco_decode_compression(arr[1]),
    );
  }

  @protected
  BigInt dco_decode_u_64(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return dcoDecodeU64(raw);
  }

  @protected
  int dco_decode_u_8(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as int;
  }

  @protected
  void dco_decode_unit(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return;
  }

  @protected
  AnyhowException sse_decode_AnyhowException(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var inner = sse_decode_String(deserializer);
    return AnyhowException(inner);
  }

  @protected
  String sse_decode_String(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var inner = sse_decode_list_prim_u_8_strict(deserializer);
    return utf8.decoder.convert(inner);
  }

  @protected
  bool sse_decode_bool(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getUint8() != 0;
  }

  @protected
  bool sse_decode_box_autoadd_bool(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return (sse_decode_bool(deserializer));
  }

  @protected
  Compression sse_decode_box_autoadd_compression(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return (sse_decode_compression(deserializer));
  }

  @protected
  int sse_decode_box_autoadd_i_32(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return (sse_decode_i_32(deserializer));
  }

  @protected
  RustyChaCha20Poly1305 sse_decode_box_autoadd_rusty_cha_cha_20_poly_1305(
      SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return (sse_decode_rusty_cha_cha_20_poly_1305(deserializer));
  }

  @protected
  RustyXChaCha20Poly1305 sse_decode_box_autoadd_rusty_x_cha_cha_20_poly_1305(
      SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return (sse_decode_rusty_x_cha_cha_20_poly_1305(deserializer));
  }

  @protected
  BigInt sse_decode_box_autoadd_u_64(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return (sse_decode_u_64(deserializer));
  }

  @protected
  Compression sse_decode_compression(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    var tag_ = sse_decode_i_32(deserializer);
    switch (tag_) {
      case 0:
        return Compression_Uncompressed();
      case 1:
        var var_compressionLevel =
            sse_decode_opt_box_autoadd_i_32(deserializer);
        return Compression_Zstd(compressionLevel: var_compressionLevel);
      default:
        throw UnimplementedError('');
    }
  }

  @protected
  int sse_decode_i_32(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getInt32();
  }

  @protected
  List<int> sse_decode_list_prim_u_8_loose(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var len_ = sse_decode_i_32(deserializer);
    return deserializer.buffer.getUint8List(len_);
  }

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var len_ = sse_decode_i_32(deserializer);
    return deserializer.buffer.getUint8List(len_);
  }

  @protected
  bool? sse_decode_opt_box_autoadd_bool(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_bool(deserializer));
    } else {
      return null;
    }
  }

  @protected
  Compression? sse_decode_opt_box_autoadd_compression(
      SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_compression(deserializer));
    } else {
      return null;
    }
  }

  @protected
  int? sse_decode_opt_box_autoadd_i_32(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_i_32(deserializer));
    } else {
      return null;
    }
  }

  @protected
  BigInt? sse_decode_opt_box_autoadd_u_64(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_u_64(deserializer));
    } else {
      return null;
    }
  }

  @protected
  Uint8List? sse_decode_opt_list_prim_u_8_strict(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    if (sse_decode_bool(deserializer)) {
      return (sse_decode_list_prim_u_8_strict(deserializer));
    } else {
      return null;
    }
  }

  @protected
  RustyChaCha20Poly1305 sse_decode_rusty_cha_cha_20_poly_1305(
      SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var var_key = sse_decode_list_prim_u_8_strict(deserializer);
    var var_compression = sse_decode_compression(deserializer);
    return RustyChaCha20Poly1305(key: var_key, compression: var_compression);
  }

  @protected
  RustyXChaCha20Poly1305 sse_decode_rusty_x_cha_cha_20_poly_1305(
      SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var var_key = sse_decode_list_prim_u_8_strict(deserializer);
    var var_compression = sse_decode_compression(deserializer);
    return RustyXChaCha20Poly1305(key: var_key, compression: var_compression);
  }

  @protected
  BigInt sse_decode_u_64(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getBigUint64();
  }

  @protected
  int sse_decode_u_8(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getUint8();
  }

  @protected
  void sse_decode_unit(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
  }

  @protected
  bool cst_encode_bool(bool raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw;
  }

  @protected
  int cst_encode_i_32(int raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw;
  }

  @protected
  int cst_encode_u_8(int raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw;
  }

  @protected
  void cst_encode_unit(void raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw;
  }

  @protected
  void sse_encode_AnyhowException(
      AnyhowException self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_String(self.message, serializer);
  }

  @protected
  void sse_encode_String(String self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_list_prim_u_8_strict(utf8.encoder.convert(self), serializer);
  }

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putUint8(self ? 1 : 0);
  }

  @protected
  void sse_encode_box_autoadd_bool(bool self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_bool(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_compression(
      Compression self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_compression(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_i_32(int self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_i_32(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_rusty_cha_cha_20_poly_1305(
      RustyChaCha20Poly1305 self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_rusty_cha_cha_20_poly_1305(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_rusty_x_cha_cha_20_poly_1305(
      RustyXChaCha20Poly1305 self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_rusty_x_cha_cha_20_poly_1305(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_u_64(BigInt self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_u_64(self, serializer);
  }

  @protected
  void sse_encode_compression(Compression self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    switch (self) {
      case Compression_Uncompressed():
        sse_encode_i_32(0, serializer);
      case Compression_Zstd(compressionLevel: final compressionLevel):
        sse_encode_i_32(1, serializer);
        sse_encode_opt_box_autoadd_i_32(compressionLevel, serializer);
      default:
        throw UnimplementedError('');
    }
  }

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putInt32(self);
  }

  @protected
  void sse_encode_list_prim_u_8_loose(
      List<int> self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_i_32(self.length, serializer);
    serializer.buffer
        .putUint8List(self is Uint8List ? self : Uint8List.fromList(self));
  }

  @protected
  void sse_encode_list_prim_u_8_strict(
      Uint8List self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_i_32(self.length, serializer);
    serializer.buffer.putUint8List(self);
  }

  @protected
  void sse_encode_opt_box_autoadd_bool(bool? self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_bool(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_compression(
      Compression? self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_compression(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_i_32(int? self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_i_32(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_u_64(BigInt? self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_u_64(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_list_prim_u_8_strict(
      Uint8List? self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_list_prim_u_8_strict(self, serializer);
    }
  }

  @protected
  void sse_encode_rusty_cha_cha_20_poly_1305(
      RustyChaCha20Poly1305 self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_list_prim_u_8_strict(self.key, serializer);
    sse_encode_compression(self.compression, serializer);
  }

  @protected
  void sse_encode_rusty_x_cha_cha_20_poly_1305(
      RustyXChaCha20Poly1305 self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_list_prim_u_8_strict(self.key, serializer);
    sse_encode_compression(self.compression, serializer);
  }

  @protected
  void sse_encode_u_64(BigInt self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putBigUint64(self);
  }

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putUint8(self);
  }

  @protected
  void sse_encode_unit(void self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
  }
}