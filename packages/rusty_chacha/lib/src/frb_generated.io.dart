// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.36.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables, unused_field

import 'api.dart';
import 'dart:async';
import 'dart:convert';
import 'dart:ffi' as ffi;
import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  @protected
  AnyhowException dco_decode_AnyhowException(dynamic raw);

  @protected
  String dco_decode_String(dynamic raw);

  @protected
  bool dco_decode_bool(dynamic raw);

  @protected
  bool dco_decode_box_autoadd_bool(dynamic raw);

  @protected
  Compression dco_decode_box_autoadd_compression(dynamic raw);

  @protected
  int dco_decode_box_autoadd_i_32(dynamic raw);

  @protected
  RustyChaCha20Poly1305 dco_decode_box_autoadd_rusty_cha_cha_20_poly_1305(
      dynamic raw);

  @protected
  RustyXChaCha20Poly1305 dco_decode_box_autoadd_rusty_x_cha_cha_20_poly_1305(
      dynamic raw);

  @protected
  BigInt dco_decode_box_autoadd_u_64(dynamic raw);

  @protected
  Compression dco_decode_compression(dynamic raw);

  @protected
  int dco_decode_i_32(dynamic raw);

  @protected
  List<int> dco_decode_list_prim_u_8_loose(dynamic raw);

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw);

  @protected
  bool? dco_decode_opt_box_autoadd_bool(dynamic raw);

  @protected
  Compression? dco_decode_opt_box_autoadd_compression(dynamic raw);

  @protected
  int? dco_decode_opt_box_autoadd_i_32(dynamic raw);

  @protected
  BigInt? dco_decode_opt_box_autoadd_u_64(dynamic raw);

  @protected
  Uint8List? dco_decode_opt_list_prim_u_8_strict(dynamic raw);

  @protected
  RustyChaCha20Poly1305 dco_decode_rusty_cha_cha_20_poly_1305(dynamic raw);

  @protected
  RustyXChaCha20Poly1305 dco_decode_rusty_x_cha_cha_20_poly_1305(dynamic raw);

  @protected
  BigInt dco_decode_u_64(dynamic raw);

  @protected
  int dco_decode_u_8(dynamic raw);

  @protected
  void dco_decode_unit(dynamic raw);

  @protected
  AnyhowException sse_decode_AnyhowException(SseDeserializer deserializer);

  @protected
  String sse_decode_String(SseDeserializer deserializer);

  @protected
  bool sse_decode_bool(SseDeserializer deserializer);

  @protected
  bool sse_decode_box_autoadd_bool(SseDeserializer deserializer);

  @protected
  Compression sse_decode_box_autoadd_compression(SseDeserializer deserializer);

  @protected
  int sse_decode_box_autoadd_i_32(SseDeserializer deserializer);

  @protected
  RustyChaCha20Poly1305 sse_decode_box_autoadd_rusty_cha_cha_20_poly_1305(
      SseDeserializer deserializer);

  @protected
  RustyXChaCha20Poly1305 sse_decode_box_autoadd_rusty_x_cha_cha_20_poly_1305(
      SseDeserializer deserializer);

  @protected
  BigInt sse_decode_box_autoadd_u_64(SseDeserializer deserializer);

  @protected
  Compression sse_decode_compression(SseDeserializer deserializer);

  @protected
  int sse_decode_i_32(SseDeserializer deserializer);

  @protected
  List<int> sse_decode_list_prim_u_8_loose(SseDeserializer deserializer);

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  bool? sse_decode_opt_box_autoadd_bool(SseDeserializer deserializer);

  @protected
  Compression? sse_decode_opt_box_autoadd_compression(
      SseDeserializer deserializer);

  @protected
  int? sse_decode_opt_box_autoadd_i_32(SseDeserializer deserializer);

  @protected
  BigInt? sse_decode_opt_box_autoadd_u_64(SseDeserializer deserializer);

  @protected
  Uint8List? sse_decode_opt_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  RustyChaCha20Poly1305 sse_decode_rusty_cha_cha_20_poly_1305(
      SseDeserializer deserializer);

  @protected
  RustyXChaCha20Poly1305 sse_decode_rusty_x_cha_cha_20_poly_1305(
      SseDeserializer deserializer);

  @protected
  BigInt sse_decode_u_64(SseDeserializer deserializer);

  @protected
  int sse_decode_u_8(SseDeserializer deserializer);

  @protected
  void sse_decode_unit(SseDeserializer deserializer);

  @protected
  ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_encode_AnyhowException(
      AnyhowException raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    throw UnimplementedError();
  }

  @protected
  ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_encode_String(String raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return cst_encode_list_prim_u_8_strict(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<ffi.Bool> cst_encode_box_autoadd_bool(bool raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return wire.cst_new_box_autoadd_bool(cst_encode_bool(raw));
  }

  @protected
  ffi.Pointer<wire_cst_compression> cst_encode_box_autoadd_compression(
      Compression raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    final ptr = wire.cst_new_box_autoadd_compression();
    cst_api_fill_to_wire_compression(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<ffi.Int32> cst_encode_box_autoadd_i_32(int raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return wire.cst_new_box_autoadd_i_32(cst_encode_i_32(raw));
  }

  @protected
  ffi.Pointer<wire_cst_rusty_cha_cha_20_poly_1305>
      cst_encode_box_autoadd_rusty_cha_cha_20_poly_1305(
          RustyChaCha20Poly1305 raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    final ptr = wire.cst_new_box_autoadd_rusty_cha_cha_20_poly_1305();
    cst_api_fill_to_wire_rusty_cha_cha_20_poly_1305(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_cst_rusty_x_cha_cha_20_poly_1305>
      cst_encode_box_autoadd_rusty_x_cha_cha_20_poly_1305(
          RustyXChaCha20Poly1305 raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    final ptr = wire.cst_new_box_autoadd_rusty_x_cha_cha_20_poly_1305();
    cst_api_fill_to_wire_rusty_x_cha_cha_20_poly_1305(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<ffi.Uint64> cst_encode_box_autoadd_u_64(BigInt raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return wire.cst_new_box_autoadd_u_64(cst_encode_u_64(raw));
  }

  @protected
  ffi.Pointer<wire_cst_list_prim_u_8_loose> cst_encode_list_prim_u_8_loose(
      List<int> raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    final ans = wire.cst_new_list_prim_u_8_loose(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  @protected
  ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_encode_list_prim_u_8_strict(
      Uint8List raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    final ans = wire.cst_new_list_prim_u_8_strict(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  @protected
  ffi.Pointer<ffi.Bool> cst_encode_opt_box_autoadd_bool(bool? raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw == null ? ffi.nullptr : cst_encode_box_autoadd_bool(raw);
  }

  @protected
  ffi.Pointer<wire_cst_compression> cst_encode_opt_box_autoadd_compression(
      Compression? raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw == null ? ffi.nullptr : cst_encode_box_autoadd_compression(raw);
  }

  @protected
  ffi.Pointer<ffi.Int32> cst_encode_opt_box_autoadd_i_32(int? raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw == null ? ffi.nullptr : cst_encode_box_autoadd_i_32(raw);
  }

  @protected
  ffi.Pointer<ffi.Uint64> cst_encode_opt_box_autoadd_u_64(BigInt? raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw == null ? ffi.nullptr : cst_encode_box_autoadd_u_64(raw);
  }

  @protected
  ffi.Pointer<wire_cst_list_prim_u_8_strict>
      cst_encode_opt_list_prim_u_8_strict(Uint8List? raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw == null ? ffi.nullptr : cst_encode_list_prim_u_8_strict(raw);
  }

  @protected
  int cst_encode_u_64(BigInt raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw.toSigned(64).toInt();
  }

  @protected
  void cst_api_fill_to_wire_box_autoadd_compression(
      Compression apiObj, ffi.Pointer<wire_cst_compression> wireObj) {
    cst_api_fill_to_wire_compression(apiObj, wireObj.ref);
  }

  @protected
  void cst_api_fill_to_wire_box_autoadd_rusty_cha_cha_20_poly_1305(
      RustyChaCha20Poly1305 apiObj,
      ffi.Pointer<wire_cst_rusty_cha_cha_20_poly_1305> wireObj) {
    cst_api_fill_to_wire_rusty_cha_cha_20_poly_1305(apiObj, wireObj.ref);
  }

  @protected
  void cst_api_fill_to_wire_box_autoadd_rusty_x_cha_cha_20_poly_1305(
      RustyXChaCha20Poly1305 apiObj,
      ffi.Pointer<wire_cst_rusty_x_cha_cha_20_poly_1305> wireObj) {
    cst_api_fill_to_wire_rusty_x_cha_cha_20_poly_1305(apiObj, wireObj.ref);
  }

  @protected
  void cst_api_fill_to_wire_compression(
      Compression apiObj, wire_cst_compression wireObj) {
    if (apiObj is Compression_Uncompressed) {
      wireObj.tag = 0;
      return;
    }
    if (apiObj is Compression_Zstd) {
      var pre_compression_level =
          cst_encode_opt_box_autoadd_i_32(apiObj.compressionLevel);
      wireObj.tag = 1;
      wireObj.kind.Zstd.compression_level = pre_compression_level;
      return;
    }
  }

  @protected
  void cst_api_fill_to_wire_rusty_cha_cha_20_poly_1305(
      RustyChaCha20Poly1305 apiObj,
      wire_cst_rusty_cha_cha_20_poly_1305 wireObj) {
    wireObj.key = cst_encode_list_prim_u_8_strict(apiObj.key);
    cst_api_fill_to_wire_compression(apiObj.compression, wireObj.compression);
  }

  @protected
  void cst_api_fill_to_wire_rusty_x_cha_cha_20_poly_1305(
      RustyXChaCha20Poly1305 apiObj,
      wire_cst_rusty_x_cha_cha_20_poly_1305 wireObj) {
    wireObj.key = cst_encode_list_prim_u_8_strict(apiObj.key);
    cst_api_fill_to_wire_compression(apiObj.compression, wireObj.compression);
  }

  @protected
  bool cst_encode_bool(bool raw);

  @protected
  int cst_encode_i_32(int raw);

  @protected
  int cst_encode_u_8(int raw);

  @protected
  void cst_encode_unit(void raw);

  @protected
  void sse_encode_AnyhowException(
      AnyhowException self, SseSerializer serializer);

  @protected
  void sse_encode_String(String self, SseSerializer serializer);

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_bool(bool self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_compression(
      Compression self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_i_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_rusty_cha_cha_20_poly_1305(
      RustyChaCha20Poly1305 self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_rusty_x_cha_cha_20_poly_1305(
      RustyXChaCha20Poly1305 self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_u_64(BigInt self, SseSerializer serializer);

  @protected
  void sse_encode_compression(Compression self, SseSerializer serializer);

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_u_8_loose(List<int> self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_u_8_strict(
      Uint8List self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_bool(bool? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_compression(
      Compression? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_i_32(int? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_u_64(BigInt? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_list_prim_u_8_strict(
      Uint8List? self, SseSerializer serializer);

  @protected
  void sse_encode_rusty_cha_cha_20_poly_1305(
      RustyChaCha20Poly1305 self, SseSerializer serializer);

  @protected
  void sse_encode_rusty_x_cha_cha_20_poly_1305(
      RustyXChaCha20Poly1305 self, SseSerializer serializer);

  @protected
  void sse_encode_u_64(BigInt self, SseSerializer serializer);

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer);

  @protected
  void sse_encode_unit(void self, SseSerializer serializer);
}

// Section: wire_class

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names
// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.
// ignore_for_file: type=lint

/// generated by flutter_rust_bridge
class RustLibWire implements BaseWire {
  factory RustLibWire.fromExternalLibrary(ExternalLibrary lib) =>
      RustLibWire(lib.ffiDynamicLibrary);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  RustLibWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  RustLibWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>(
          'store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr
      .asFunction<void Function(DartPostCObjectFnType)>();

  void wire__crate__api__compress(
    int port_,
    ffi.Pointer<wire_cst_list_prim_u_8_loose> data,
    int zstd_compression_level,
  ) {
    return _wire__crate__api__compress(
      port_,
      data,
      zstd_compression_level,
    );
  }

  late final _wire__crate__api__compressPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              ffi.Pointer<wire_cst_list_prim_u_8_loose>,
              ffi.Int32)>>('frbgen_rusty_chacha_wire__crate__api__compress');
  late final _wire__crate__api__compress =
      _wire__crate__api__compressPtr.asFunction<
          void Function(int, ffi.Pointer<wire_cst_list_prim_u_8_loose>, int)>();

  void wire__crate__api__decompress(
    int port_,
    ffi.Pointer<wire_cst_list_prim_u_8_loose> data,
  ) {
    return _wire__crate__api__decompress(
      port_,
      data,
    );
  }

  late final _wire__crate__api__decompressPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64, ffi.Pointer<wire_cst_list_prim_u_8_loose>)>>(
      'frbgen_rusty_chacha_wire__crate__api__decompress');
  late final _wire__crate__api__decompress =
      _wire__crate__api__decompressPtr.asFunction<
          void Function(int, ffi.Pointer<wire_cst_list_prim_u_8_loose>)>();

  void wire__crate__api__rusty_cha_cha_20_poly_1305_create_internal(
    int port_,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> key,
    ffi.Pointer<wire_cst_compression> compression,
  ) {
    return _wire__crate__api__rusty_cha_cha_20_poly_1305_create_internal(
      port_,
      key,
      compression,
    );
  }

  late final _wire__crate__api__rusty_cha_cha_20_poly_1305_create_internalPtr =
      _lookup<
              ffi.NativeFunction<
                  ffi.Void Function(
                      ffi.Int64,
                      ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                      ffi.Pointer<wire_cst_compression>)>>(
          'frbgen_rusty_chacha_wire__crate__api__rusty_cha_cha_20_poly_1305_create_internal');
  late final _wire__crate__api__rusty_cha_cha_20_poly_1305_create_internal =
      _wire__crate__api__rusty_cha_cha_20_poly_1305_create_internalPtr
          .asFunction<
              void Function(int, ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                  ffi.Pointer<wire_cst_compression>)>();

  void wire__crate__api__rusty_cha_cha_20_poly_1305_decrypt(
    int port_,
    ffi.Pointer<wire_cst_rusty_cha_cha_20_poly_1305> that,
    ffi.Pointer<wire_cst_list_prim_u_8_loose> ciphertext,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> aad,
  ) {
    return _wire__crate__api__rusty_cha_cha_20_poly_1305_decrypt(
      port_,
      that,
      ciphertext,
      aad,
    );
  }

  late final _wire__crate__api__rusty_cha_cha_20_poly_1305_decryptPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64,
                  ffi.Pointer<wire_cst_rusty_cha_cha_20_poly_1305>,
                  ffi.Pointer<wire_cst_list_prim_u_8_loose>,
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>)>>(
      'frbgen_rusty_chacha_wire__crate__api__rusty_cha_cha_20_poly_1305_decrypt');
  late final _wire__crate__api__rusty_cha_cha_20_poly_1305_decrypt =
      _wire__crate__api__rusty_cha_cha_20_poly_1305_decryptPtr.asFunction<
          void Function(
              int,
              ffi.Pointer<wire_cst_rusty_cha_cha_20_poly_1305>,
              ffi.Pointer<wire_cst_list_prim_u_8_loose>,
              ffi.Pointer<wire_cst_list_prim_u_8_strict>)>();

  void wire__crate__api__rusty_cha_cha_20_poly_1305_decrypt_from_file(
    int port_,
    ffi.Pointer<wire_cst_rusty_cha_cha_20_poly_1305> that,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> file_path,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> aad,
    ffi.Pointer<ffi.Uint64> offset,
  ) {
    return _wire__crate__api__rusty_cha_cha_20_poly_1305_decrypt_from_file(
      port_,
      that,
      file_path,
      aad,
      offset,
    );
  }

  late final _wire__crate__api__rusty_cha_cha_20_poly_1305_decrypt_from_filePtr =
      _lookup<
              ffi.NativeFunction<
                  ffi.Void Function(
                      ffi.Int64,
                      ffi.Pointer<wire_cst_rusty_cha_cha_20_poly_1305>,
                      ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                      ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                      ffi.Pointer<ffi.Uint64>)>>(
          'frbgen_rusty_chacha_wire__crate__api__rusty_cha_cha_20_poly_1305_decrypt_from_file');
  late final _wire__crate__api__rusty_cha_cha_20_poly_1305_decrypt_from_file =
      _wire__crate__api__rusty_cha_cha_20_poly_1305_decrypt_from_filePtr
          .asFunction<
              void Function(
                  int,
                  ffi.Pointer<wire_cst_rusty_cha_cha_20_poly_1305>,
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                  ffi.Pointer<ffi.Uint64>)>();

  void wire__crate__api__rusty_cha_cha_20_poly_1305_encrypt(
    int port_,
    ffi.Pointer<wire_cst_rusty_cha_cha_20_poly_1305> that,
    ffi.Pointer<wire_cst_list_prim_u_8_loose> cleartext,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> nonce,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> aad,
  ) {
    return _wire__crate__api__rusty_cha_cha_20_poly_1305_encrypt(
      port_,
      that,
      cleartext,
      nonce,
      aad,
    );
  }

  late final _wire__crate__api__rusty_cha_cha_20_poly_1305_encryptPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64,
                  ffi.Pointer<wire_cst_rusty_cha_cha_20_poly_1305>,
                  ffi.Pointer<wire_cst_list_prim_u_8_loose>,
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>)>>(
      'frbgen_rusty_chacha_wire__crate__api__rusty_cha_cha_20_poly_1305_encrypt');
  late final _wire__crate__api__rusty_cha_cha_20_poly_1305_encrypt =
      _wire__crate__api__rusty_cha_cha_20_poly_1305_encryptPtr.asFunction<
          void Function(
              int,
              ffi.Pointer<wire_cst_rusty_cha_cha_20_poly_1305>,
              ffi.Pointer<wire_cst_list_prim_u_8_loose>,
              ffi.Pointer<wire_cst_list_prim_u_8_strict>,
              ffi.Pointer<wire_cst_list_prim_u_8_strict>)>();

  void wire__crate__api__rusty_cha_cha_20_poly_1305_encrypt_to_file(
    int port_,
    ffi.Pointer<wire_cst_rusty_cha_cha_20_poly_1305> that,
    ffi.Pointer<wire_cst_list_prim_u_8_loose> cleartext,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> file_path,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> nonce,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> aad,
    ffi.Pointer<ffi.Bool> append,
  ) {
    return _wire__crate__api__rusty_cha_cha_20_poly_1305_encrypt_to_file(
      port_,
      that,
      cleartext,
      file_path,
      nonce,
      aad,
      append,
    );
  }

  late final _wire__crate__api__rusty_cha_cha_20_poly_1305_encrypt_to_filePtr =
      _lookup<
              ffi.NativeFunction<
                  ffi.Void Function(
                      ffi.Int64,
                      ffi.Pointer<wire_cst_rusty_cha_cha_20_poly_1305>,
                      ffi.Pointer<wire_cst_list_prim_u_8_loose>,
                      ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                      ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                      ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                      ffi.Pointer<ffi.Bool>)>>(
          'frbgen_rusty_chacha_wire__crate__api__rusty_cha_cha_20_poly_1305_encrypt_to_file');
  late final _wire__crate__api__rusty_cha_cha_20_poly_1305_encrypt_to_file =
      _wire__crate__api__rusty_cha_cha_20_poly_1305_encrypt_to_filePtr
          .asFunction<
              void Function(
                  int,
                  ffi.Pointer<wire_cst_rusty_cha_cha_20_poly_1305>,
                  ffi.Pointer<wire_cst_list_prim_u_8_loose>,
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                  ffi.Pointer<ffi.Bool>)>();

  void wire__crate__api__rusty_cha_cha_20_poly_1305_generate_key(
    int port_,
  ) {
    return _wire__crate__api__rusty_cha_cha_20_poly_1305_generate_key(
      port_,
    );
  }

  late final _wire__crate__api__rusty_cha_cha_20_poly_1305_generate_keyPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'frbgen_rusty_chacha_wire__crate__api__rusty_cha_cha_20_poly_1305_generate_key');
  late final _wire__crate__api__rusty_cha_cha_20_poly_1305_generate_key =
      _wire__crate__api__rusty_cha_cha_20_poly_1305_generate_keyPtr
          .asFunction<void Function(int)>();

  void wire__crate__api__rusty_cha_cha_20_poly_1305_generate_nonce(
    int port_,
  ) {
    return _wire__crate__api__rusty_cha_cha_20_poly_1305_generate_nonce(
      port_,
    );
  }

  late final _wire__crate__api__rusty_cha_cha_20_poly_1305_generate_noncePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'frbgen_rusty_chacha_wire__crate__api__rusty_cha_cha_20_poly_1305_generate_nonce');
  late final _wire__crate__api__rusty_cha_cha_20_poly_1305_generate_nonce =
      _wire__crate__api__rusty_cha_cha_20_poly_1305_generate_noncePtr
          .asFunction<void Function(int)>();

  void wire__crate__api__rusty_x_cha_cha_20_poly_1305_create_internal(
    int port_,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> key,
    ffi.Pointer<wire_cst_compression> compression,
  ) {
    return _wire__crate__api__rusty_x_cha_cha_20_poly_1305_create_internal(
      port_,
      key,
      compression,
    );
  }

  late final _wire__crate__api__rusty_x_cha_cha_20_poly_1305_create_internalPtr =
      _lookup<
              ffi.NativeFunction<
                  ffi.Void Function(
                      ffi.Int64,
                      ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                      ffi.Pointer<wire_cst_compression>)>>(
          'frbgen_rusty_chacha_wire__crate__api__rusty_x_cha_cha_20_poly_1305_create_internal');
  late final _wire__crate__api__rusty_x_cha_cha_20_poly_1305_create_internal =
      _wire__crate__api__rusty_x_cha_cha_20_poly_1305_create_internalPtr
          .asFunction<
              void Function(int, ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                  ffi.Pointer<wire_cst_compression>)>();

  void wire__crate__api__rusty_x_cha_cha_20_poly_1305_decrypt(
    int port_,
    ffi.Pointer<wire_cst_rusty_x_cha_cha_20_poly_1305> that,
    ffi.Pointer<wire_cst_list_prim_u_8_loose> ciphertext,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> aad,
  ) {
    return _wire__crate__api__rusty_x_cha_cha_20_poly_1305_decrypt(
      port_,
      that,
      ciphertext,
      aad,
    );
  }

  late final _wire__crate__api__rusty_x_cha_cha_20_poly_1305_decryptPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64,
                  ffi.Pointer<wire_cst_rusty_x_cha_cha_20_poly_1305>,
                  ffi.Pointer<wire_cst_list_prim_u_8_loose>,
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>)>>(
      'frbgen_rusty_chacha_wire__crate__api__rusty_x_cha_cha_20_poly_1305_decrypt');
  late final _wire__crate__api__rusty_x_cha_cha_20_poly_1305_decrypt =
      _wire__crate__api__rusty_x_cha_cha_20_poly_1305_decryptPtr.asFunction<
          void Function(
              int,
              ffi.Pointer<wire_cst_rusty_x_cha_cha_20_poly_1305>,
              ffi.Pointer<wire_cst_list_prim_u_8_loose>,
              ffi.Pointer<wire_cst_list_prim_u_8_strict>)>();

  void wire__crate__api__rusty_x_cha_cha_20_poly_1305_decrypt_from_file(
    int port_,
    ffi.Pointer<wire_cst_rusty_x_cha_cha_20_poly_1305> that,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> file_path,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> aad,
    ffi.Pointer<ffi.Uint64> offset,
  ) {
    return _wire__crate__api__rusty_x_cha_cha_20_poly_1305_decrypt_from_file(
      port_,
      that,
      file_path,
      aad,
      offset,
    );
  }

  late final _wire__crate__api__rusty_x_cha_cha_20_poly_1305_decrypt_from_filePtr =
      _lookup<
              ffi.NativeFunction<
                  ffi.Void Function(
                      ffi.Int64,
                      ffi.Pointer<wire_cst_rusty_x_cha_cha_20_poly_1305>,
                      ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                      ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                      ffi.Pointer<ffi.Uint64>)>>(
          'frbgen_rusty_chacha_wire__crate__api__rusty_x_cha_cha_20_poly_1305_decrypt_from_file');
  late final _wire__crate__api__rusty_x_cha_cha_20_poly_1305_decrypt_from_file =
      _wire__crate__api__rusty_x_cha_cha_20_poly_1305_decrypt_from_filePtr
          .asFunction<
              void Function(
                  int,
                  ffi.Pointer<wire_cst_rusty_x_cha_cha_20_poly_1305>,
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                  ffi.Pointer<ffi.Uint64>)>();

  void wire__crate__api__rusty_x_cha_cha_20_poly_1305_encrypt(
    int port_,
    ffi.Pointer<wire_cst_rusty_x_cha_cha_20_poly_1305> that,
    ffi.Pointer<wire_cst_list_prim_u_8_loose> cleartext,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> nonce,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> aad,
  ) {
    return _wire__crate__api__rusty_x_cha_cha_20_poly_1305_encrypt(
      port_,
      that,
      cleartext,
      nonce,
      aad,
    );
  }

  late final _wire__crate__api__rusty_x_cha_cha_20_poly_1305_encryptPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64,
                  ffi.Pointer<wire_cst_rusty_x_cha_cha_20_poly_1305>,
                  ffi.Pointer<wire_cst_list_prim_u_8_loose>,
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>)>>(
      'frbgen_rusty_chacha_wire__crate__api__rusty_x_cha_cha_20_poly_1305_encrypt');
  late final _wire__crate__api__rusty_x_cha_cha_20_poly_1305_encrypt =
      _wire__crate__api__rusty_x_cha_cha_20_poly_1305_encryptPtr.asFunction<
          void Function(
              int,
              ffi.Pointer<wire_cst_rusty_x_cha_cha_20_poly_1305>,
              ffi.Pointer<wire_cst_list_prim_u_8_loose>,
              ffi.Pointer<wire_cst_list_prim_u_8_strict>,
              ffi.Pointer<wire_cst_list_prim_u_8_strict>)>();

  void wire__crate__api__rusty_x_cha_cha_20_poly_1305_encrypt_to_file(
    int port_,
    ffi.Pointer<wire_cst_rusty_x_cha_cha_20_poly_1305> that,
    ffi.Pointer<wire_cst_list_prim_u_8_loose> cleartext,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> file_path,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> nonce,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> aad,
    ffi.Pointer<ffi.Bool> append,
  ) {
    return _wire__crate__api__rusty_x_cha_cha_20_poly_1305_encrypt_to_file(
      port_,
      that,
      cleartext,
      file_path,
      nonce,
      aad,
      append,
    );
  }

  late final _wire__crate__api__rusty_x_cha_cha_20_poly_1305_encrypt_to_filePtr =
      _lookup<
              ffi.NativeFunction<
                  ffi.Void Function(
                      ffi.Int64,
                      ffi.Pointer<wire_cst_rusty_x_cha_cha_20_poly_1305>,
                      ffi.Pointer<wire_cst_list_prim_u_8_loose>,
                      ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                      ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                      ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                      ffi.Pointer<ffi.Bool>)>>(
          'frbgen_rusty_chacha_wire__crate__api__rusty_x_cha_cha_20_poly_1305_encrypt_to_file');
  late final _wire__crate__api__rusty_x_cha_cha_20_poly_1305_encrypt_to_file =
      _wire__crate__api__rusty_x_cha_cha_20_poly_1305_encrypt_to_filePtr
          .asFunction<
              void Function(
                  int,
                  ffi.Pointer<wire_cst_rusty_x_cha_cha_20_poly_1305>,
                  ffi.Pointer<wire_cst_list_prim_u_8_loose>,
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>,
                  ffi.Pointer<ffi.Bool>)>();

  void wire__crate__api__rusty_x_cha_cha_20_poly_1305_generate_key(
    int port_,
  ) {
    return _wire__crate__api__rusty_x_cha_cha_20_poly_1305_generate_key(
      port_,
    );
  }

  late final _wire__crate__api__rusty_x_cha_cha_20_poly_1305_generate_keyPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'frbgen_rusty_chacha_wire__crate__api__rusty_x_cha_cha_20_poly_1305_generate_key');
  late final _wire__crate__api__rusty_x_cha_cha_20_poly_1305_generate_key =
      _wire__crate__api__rusty_x_cha_cha_20_poly_1305_generate_keyPtr
          .asFunction<void Function(int)>();

  void wire__crate__api__rusty_x_cha_cha_20_poly_1305_generate_nonce(
    int port_,
  ) {
    return _wire__crate__api__rusty_x_cha_cha_20_poly_1305_generate_nonce(
      port_,
    );
  }

  late final _wire__crate__api__rusty_x_cha_cha_20_poly_1305_generate_noncePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'frbgen_rusty_chacha_wire__crate__api__rusty_x_cha_cha_20_poly_1305_generate_nonce');
  late final _wire__crate__api__rusty_x_cha_cha_20_poly_1305_generate_nonce =
      _wire__crate__api__rusty_x_cha_cha_20_poly_1305_generate_noncePtr
          .asFunction<void Function(int)>();

  ffi.Pointer<ffi.Bool> cst_new_box_autoadd_bool(
    bool value,
  ) {
    return _cst_new_box_autoadd_bool(
      value,
    );
  }

  late final _cst_new_box_autoadd_boolPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Bool> Function(ffi.Bool)>>(
          'frbgen_rusty_chacha_cst_new_box_autoadd_bool');
  late final _cst_new_box_autoadd_bool = _cst_new_box_autoadd_boolPtr
      .asFunction<ffi.Pointer<ffi.Bool> Function(bool)>();

  ffi.Pointer<wire_cst_compression> cst_new_box_autoadd_compression() {
    return _cst_new_box_autoadd_compression();
  }

  late final _cst_new_box_autoadd_compressionPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_cst_compression> Function()>>(
          'frbgen_rusty_chacha_cst_new_box_autoadd_compression');
  late final _cst_new_box_autoadd_compression =
      _cst_new_box_autoadd_compressionPtr
          .asFunction<ffi.Pointer<wire_cst_compression> Function()>();

  ffi.Pointer<ffi.Int32> cst_new_box_autoadd_i_32(
    int value,
  ) {
    return _cst_new_box_autoadd_i_32(
      value,
    );
  }

  late final _cst_new_box_autoadd_i_32Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Int32> Function(ffi.Int32)>>(
          'frbgen_rusty_chacha_cst_new_box_autoadd_i_32');
  late final _cst_new_box_autoadd_i_32 = _cst_new_box_autoadd_i_32Ptr
      .asFunction<ffi.Pointer<ffi.Int32> Function(int)>();

  ffi.Pointer<wire_cst_rusty_cha_cha_20_poly_1305>
      cst_new_box_autoadd_rusty_cha_cha_20_poly_1305() {
    return _cst_new_box_autoadd_rusty_cha_cha_20_poly_1305();
  }

  late final _cst_new_box_autoadd_rusty_cha_cha_20_poly_1305Ptr = _lookup<
          ffi.NativeFunction<
              ffi.Pointer<wire_cst_rusty_cha_cha_20_poly_1305> Function()>>(
      'frbgen_rusty_chacha_cst_new_box_autoadd_rusty_cha_cha_20_poly_1305');
  late final _cst_new_box_autoadd_rusty_cha_cha_20_poly_1305 =
      _cst_new_box_autoadd_rusty_cha_cha_20_poly_1305Ptr.asFunction<
          ffi.Pointer<wire_cst_rusty_cha_cha_20_poly_1305> Function()>();

  ffi.Pointer<wire_cst_rusty_x_cha_cha_20_poly_1305>
      cst_new_box_autoadd_rusty_x_cha_cha_20_poly_1305() {
    return _cst_new_box_autoadd_rusty_x_cha_cha_20_poly_1305();
  }

  late final _cst_new_box_autoadd_rusty_x_cha_cha_20_poly_1305Ptr = _lookup<
          ffi.NativeFunction<
              ffi.Pointer<wire_cst_rusty_x_cha_cha_20_poly_1305> Function()>>(
      'frbgen_rusty_chacha_cst_new_box_autoadd_rusty_x_cha_cha_20_poly_1305');
  late final _cst_new_box_autoadd_rusty_x_cha_cha_20_poly_1305 =
      _cst_new_box_autoadd_rusty_x_cha_cha_20_poly_1305Ptr.asFunction<
          ffi.Pointer<wire_cst_rusty_x_cha_cha_20_poly_1305> Function()>();

  ffi.Pointer<ffi.Uint64> cst_new_box_autoadd_u_64(
    int value,
  ) {
    return _cst_new_box_autoadd_u_64(
      value,
    );
  }

  late final _cst_new_box_autoadd_u_64Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Uint64> Function(ffi.Uint64)>>(
          'frbgen_rusty_chacha_cst_new_box_autoadd_u_64');
  late final _cst_new_box_autoadd_u_64 = _cst_new_box_autoadd_u_64Ptr
      .asFunction<ffi.Pointer<ffi.Uint64> Function(int)>();

  ffi.Pointer<wire_cst_list_prim_u_8_loose> cst_new_list_prim_u_8_loose(
    int len,
  ) {
    return _cst_new_list_prim_u_8_loose(
      len,
    );
  }

  late final _cst_new_list_prim_u_8_loosePtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_cst_list_prim_u_8_loose> Function(
              ffi.Int32)>>('frbgen_rusty_chacha_cst_new_list_prim_u_8_loose');
  late final _cst_new_list_prim_u_8_loose = _cst_new_list_prim_u_8_loosePtr
      .asFunction<ffi.Pointer<wire_cst_list_prim_u_8_loose> Function(int)>();

  ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_new_list_prim_u_8_strict(
    int len,
  ) {
    return _cst_new_list_prim_u_8_strict(
      len,
    );
  }

  late final _cst_new_list_prim_u_8_strictPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_cst_list_prim_u_8_strict> Function(
              ffi.Int32)>>('frbgen_rusty_chacha_cst_new_list_prim_u_8_strict');
  late final _cst_new_list_prim_u_8_strict = _cst_new_list_prim_u_8_strictPtr
      .asFunction<ffi.Pointer<wire_cst_list_prim_u_8_strict> Function(int)>();

  int dummy_method_to_enforce_bundling() {
    return _dummy_method_to_enforce_bundling();
  }

  late final _dummy_method_to_enforce_bundlingPtr =
      _lookup<ffi.NativeFunction<ffi.Int64 Function()>>(
          'dummy_method_to_enforce_bundling');
  late final _dummy_method_to_enforce_bundling =
      _dummy_method_to_enforce_bundlingPtr.asFunction<int Function()>();
}

typedef DartPostCObjectFnType
    = ffi.Pointer<ffi.NativeFunction<DartPostCObjectFnTypeFunction>>;
typedef DartPostCObjectFnTypeFunction = ffi.Bool Function(
    DartPort port_id, ffi.Pointer<ffi.Void> message);
typedef DartDartPostCObjectFnTypeFunction = bool Function(
    DartDartPort port_id, ffi.Pointer<ffi.Void> message);
typedef DartPort = ffi.Int64;
typedef DartDartPort = int;

final class wire_cst_list_prim_u_8_loose extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_cst_list_prim_u_8_strict extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_cst_Compression_Zstd extends ffi.Struct {
  external ffi.Pointer<ffi.Int32> compression_level;
}

final class CompressionKind extends ffi.Union {
  external wire_cst_Compression_Zstd Zstd;
}

final class wire_cst_compression extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external CompressionKind kind;
}

final class wire_cst_rusty_cha_cha_20_poly_1305 extends ffi.Struct {
  external ffi.Pointer<wire_cst_list_prim_u_8_strict> key;

  external wire_cst_compression compression;
}

final class wire_cst_rusty_x_cha_cha_20_poly_1305 extends ffi.Struct {
  external ffi.Pointer<wire_cst_list_prim_u_8_strict> key;

  external wire_cst_compression compression;
}
