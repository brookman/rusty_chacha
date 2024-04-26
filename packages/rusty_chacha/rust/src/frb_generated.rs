// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.32.

#![allow(
    non_camel_case_types,
    unused,
    non_snake_case,
    clippy::needless_return,
    clippy::redundant_closure_call,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::unused_unit,
    clippy::double_parens,
    clippy::let_and_return,
    clippy::too_many_arguments,
    clippy::match_single_binding
)]

// Section: imports

use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate!(
    default_stream_sink_codec = DcoCodec,
    default_rust_opaque = RustOpaqueNom,
    default_rust_auto_opaque = RustAutoOpaqueNom,
);
pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_VERSION: &str = "2.0.0-dev.32";
pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_CONTENT_HASH: i32 = 1378991708;

// Section: executor

flutter_rust_bridge::frb_generated_default_handler!();

// Section: wire_funcs

fn wire_compress_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    data: impl CstDecode<Vec<u8>>,
    zstd_compression_level: impl CstDecode<i32>,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "compress",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_data = data.cst_decode();
            let api_zstd_compression_level = zstd_compression_level.cst_decode();
            move |context| {
                transform_result_dco((move || {
                    crate::api::compress(api_data, api_zstd_compression_level)
                })())
            }
        },
    )
}
fn wire_decompress_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    data: impl CstDecode<Vec<u8>>,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "decompress",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_data = data.cst_decode();
            move |context| transform_result_dco((move || crate::api::decompress(api_data))())
        },
    )
}
fn wire_rusty_cha_cha_20_poly_1305_create_internal_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    key: impl CstDecode<Option<Vec<u8>>>,
    compression: impl CstDecode<Option<crate::api::Compression>>,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "rusty_cha_cha_20_poly_1305_create_internal",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_key = key.cst_decode();
            let api_compression = compression.cst_decode();
            move |context| {
                transform_result_dco((move || {
                    crate::api::RustyChaCha20Poly1305::create_internal(api_key, api_compression)
                })())
            }
        },
    )
}
fn wire_rusty_cha_cha_20_poly_1305_decrypt_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: impl CstDecode<crate::api::RustyChaCha20Poly1305>,
    ciphertext: impl CstDecode<Vec<u8>>,
    aad: impl CstDecode<Option<Vec<u8>>>,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "rusty_cha_cha_20_poly_1305_decrypt",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_that = that.cst_decode();
            let api_ciphertext = ciphertext.cst_decode();
            let api_aad = aad.cst_decode();
            move |context| {
                transform_result_dco((move || {
                    crate::api::RustyChaCha20Poly1305::decrypt(&api_that, api_ciphertext, api_aad)
                })())
            }
        },
    )
}
fn wire_rusty_cha_cha_20_poly_1305_decrypt_from_file_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: impl CstDecode<crate::api::RustyChaCha20Poly1305>,
    file_path: impl CstDecode<String>,
    aad: impl CstDecode<Option<Vec<u8>>>,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "rusty_cha_cha_20_poly_1305_decrypt_from_file",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_that = that.cst_decode();
            let api_file_path = file_path.cst_decode();
            let api_aad = aad.cst_decode();
            move |context| {
                transform_result_dco((move || {
                    crate::api::RustyChaCha20Poly1305::decrypt_from_file(
                        &api_that,
                        api_file_path,
                        api_aad,
                    )
                })())
            }
        },
    )
}
fn wire_rusty_cha_cha_20_poly_1305_encrypt_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: impl CstDecode<crate::api::RustyChaCha20Poly1305>,
    cleartext: impl CstDecode<Vec<u8>>,
    nonce: impl CstDecode<Option<Vec<u8>>>,
    aad: impl CstDecode<Option<Vec<u8>>>,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "rusty_cha_cha_20_poly_1305_encrypt",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_that = that.cst_decode();
            let api_cleartext = cleartext.cst_decode();
            let api_nonce = nonce.cst_decode();
            let api_aad = aad.cst_decode();
            move |context| {
                transform_result_dco((move || {
                    crate::api::RustyChaCha20Poly1305::encrypt(
                        &api_that,
                        api_cleartext,
                        api_nonce,
                        api_aad,
                    )
                })())
            }
        },
    )
}
fn wire_rusty_cha_cha_20_poly_1305_encrypt_to_file_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: impl CstDecode<crate::api::RustyChaCha20Poly1305>,
    cleartext: impl CstDecode<Vec<u8>>,
    file_path: impl CstDecode<String>,
    nonce: impl CstDecode<Option<Vec<u8>>>,
    aad: impl CstDecode<Option<Vec<u8>>>,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "rusty_cha_cha_20_poly_1305_encrypt_to_file",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_that = that.cst_decode();
            let api_cleartext = cleartext.cst_decode();
            let api_file_path = file_path.cst_decode();
            let api_nonce = nonce.cst_decode();
            let api_aad = aad.cst_decode();
            move |context| {
                transform_result_dco((move || {
                    crate::api::RustyChaCha20Poly1305::encrypt_to_file(
                        &api_that,
                        api_cleartext,
                        api_file_path,
                        api_nonce,
                        api_aad,
                    )
                })())
            }
        },
    )
}
fn wire_rusty_cha_cha_20_poly_1305_generate_key_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "rusty_cha_cha_20_poly_1305_generate_key",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            move |context| {
                transform_result_dco((move || {
                    Result::<_, ()>::Ok(crate::api::RustyChaCha20Poly1305::generate_key())
                })())
            }
        },
    )
}
fn wire_rusty_cha_cha_20_poly_1305_generate_nonce_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "rusty_cha_cha_20_poly_1305_generate_nonce",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            move |context| {
                transform_result_dco((move || {
                    Result::<_, ()>::Ok(crate::api::RustyChaCha20Poly1305::generate_nonce())
                })())
            }
        },
    )
}
fn wire_rusty_x_cha_cha_20_poly_1305_create_internal_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    key: impl CstDecode<Option<Vec<u8>>>,
    compression: impl CstDecode<Option<crate::api::Compression>>,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "rusty_x_cha_cha_20_poly_1305_create_internal",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_key = key.cst_decode();
            let api_compression = compression.cst_decode();
            move |context| {
                transform_result_dco((move || {
                    crate::api::RustyXChaCha20Poly1305::create_internal(api_key, api_compression)
                })())
            }
        },
    )
}
fn wire_rusty_x_cha_cha_20_poly_1305_decrypt_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: impl CstDecode<crate::api::RustyXChaCha20Poly1305>,
    ciphertext: impl CstDecode<Vec<u8>>,
    aad: impl CstDecode<Option<Vec<u8>>>,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "rusty_x_cha_cha_20_poly_1305_decrypt",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_that = that.cst_decode();
            let api_ciphertext = ciphertext.cst_decode();
            let api_aad = aad.cst_decode();
            move |context| {
                transform_result_dco((move || {
                    crate::api::RustyXChaCha20Poly1305::decrypt(&api_that, api_ciphertext, api_aad)
                })())
            }
        },
    )
}
fn wire_rusty_x_cha_cha_20_poly_1305_decrypt_from_file_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: impl CstDecode<crate::api::RustyXChaCha20Poly1305>,
    file_path: impl CstDecode<String>,
    aad: impl CstDecode<Option<Vec<u8>>>,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "rusty_x_cha_cha_20_poly_1305_decrypt_from_file",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_that = that.cst_decode();
            let api_file_path = file_path.cst_decode();
            let api_aad = aad.cst_decode();
            move |context| {
                transform_result_dco((move || {
                    crate::api::RustyXChaCha20Poly1305::decrypt_from_file(
                        &api_that,
                        api_file_path,
                        api_aad,
                    )
                })())
            }
        },
    )
}
fn wire_rusty_x_cha_cha_20_poly_1305_encrypt_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: impl CstDecode<crate::api::RustyXChaCha20Poly1305>,
    cleartext: impl CstDecode<Vec<u8>>,
    nonce: impl CstDecode<Option<Vec<u8>>>,
    aad: impl CstDecode<Option<Vec<u8>>>,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "rusty_x_cha_cha_20_poly_1305_encrypt",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_that = that.cst_decode();
            let api_cleartext = cleartext.cst_decode();
            let api_nonce = nonce.cst_decode();
            let api_aad = aad.cst_decode();
            move |context| {
                transform_result_dco((move || {
                    crate::api::RustyXChaCha20Poly1305::encrypt(
                        &api_that,
                        api_cleartext,
                        api_nonce,
                        api_aad,
                    )
                })())
            }
        },
    )
}
fn wire_rusty_x_cha_cha_20_poly_1305_encrypt_to_file_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: impl CstDecode<crate::api::RustyXChaCha20Poly1305>,
    cleartext: impl CstDecode<Vec<u8>>,
    file_path: impl CstDecode<String>,
    nonce: impl CstDecode<Option<Vec<u8>>>,
    aad: impl CstDecode<Option<Vec<u8>>>,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "rusty_x_cha_cha_20_poly_1305_encrypt_to_file",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_that = that.cst_decode();
            let api_cleartext = cleartext.cst_decode();
            let api_file_path = file_path.cst_decode();
            let api_nonce = nonce.cst_decode();
            let api_aad = aad.cst_decode();
            move |context| {
                transform_result_dco((move || {
                    crate::api::RustyXChaCha20Poly1305::encrypt_to_file(
                        &api_that,
                        api_cleartext,
                        api_file_path,
                        api_nonce,
                        api_aad,
                    )
                })())
            }
        },
    )
}
fn wire_rusty_x_cha_cha_20_poly_1305_generate_key_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "rusty_x_cha_cha_20_poly_1305_generate_key",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            move |context| {
                transform_result_dco((move || {
                    Result::<_, ()>::Ok(crate::api::RustyXChaCha20Poly1305::generate_key())
                })())
            }
        },
    )
}
fn wire_rusty_x_cha_cha_20_poly_1305_generate_nonce_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "rusty_x_cha_cha_20_poly_1305_generate_nonce",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            move |context| {
                transform_result_dco((move || {
                    Result::<_, ()>::Ok(crate::api::RustyXChaCha20Poly1305::generate_nonce())
                })())
            }
        },
    )
}

// Section: dart2rust

impl CstDecode<i32> for i32 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> i32 {
        self
    }
}
impl CstDecode<u8> for u8 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> u8 {
        self
    }
}
impl SseDecode for flutter_rust_bridge::for_generated::anyhow::Error {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        unreachable!("");
    }
}

impl SseDecode for String {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <Vec<u8>>::sse_decode(deserializer);
        return String::from_utf8(inner).unwrap();
    }
}

impl SseDecode for crate::api::Compression {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut tag_ = <i32>::sse_decode(deserializer);
        match tag_ {
            0 => {
                return crate::api::Compression::Uncompressed;
            }
            1 => {
                let mut var_compressionLevel = <Option<i32>>::sse_decode(deserializer);
                return crate::api::Compression::Zstd {
                    compression_level: var_compressionLevel,
                };
            }
            _ => {
                unimplemented!("");
            }
        }
    }
}

impl SseDecode for i32 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_i32::<NativeEndian>().unwrap()
    }
}

impl SseDecode for Vec<u8> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = vec![];
        for idx_ in 0..len_ {
            ans_.push(<u8>::sse_decode(deserializer));
        }
        return ans_;
    }
}

impl SseDecode for Option<crate::api::Compression> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<crate::api::Compression>::sse_decode(deserializer));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<i32> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<i32>::sse_decode(deserializer));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<Vec<u8>> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<Vec<u8>>::sse_decode(deserializer));
        } else {
            return None;
        }
    }
}

impl SseDecode for crate::api::RustyChaCha20Poly1305 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_key = <Vec<u8>>::sse_decode(deserializer);
        let mut var_compression = <crate::api::Compression>::sse_decode(deserializer);
        return crate::api::RustyChaCha20Poly1305 {
            key: var_key,
            compression: var_compression,
        };
    }
}

impl SseDecode for crate::api::RustyXChaCha20Poly1305 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_key = <Vec<u8>>::sse_decode(deserializer);
        let mut var_compression = <crate::api::Compression>::sse_decode(deserializer);
        return crate::api::RustyXChaCha20Poly1305 {
            key: var_key,
            compression: var_compression,
        };
    }
}

impl SseDecode for u8 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u8().unwrap()
    }
}

impl SseDecode for () {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {}
}

impl SseDecode for bool {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u8().unwrap() != 0
    }
}

fn pde_ffi_dispatcher_primary_impl(
    func_id: i32,
    port: flutter_rust_bridge::for_generated::MessagePort,
    ptr: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len: i32,
    data_len: i32,
) {
    // Codec=Pde (Serialization + dispatch), see doc to use other codecs
    match func_id {
        _ => unreachable!(),
    }
}

fn pde_ffi_dispatcher_sync_impl(
    func_id: i32,
    ptr: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len: i32,
    data_len: i32,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartSse {
    // Codec=Pde (Serialization + dispatch), see doc to use other codecs
    match func_id {
        _ => unreachable!(),
    }
}

// Section: rust2dart

// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for crate::api::Compression {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self {
            crate::api::Compression::Uncompressed => [0.into_dart()].into_dart(),
            crate::api::Compression::Zstd { compression_level } => [
                1.into_dart(),
                compression_level.into_into_dart().into_dart(),
            ]
            .into_dart(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive for crate::api::Compression {}
impl flutter_rust_bridge::IntoIntoDart<crate::api::Compression> for crate::api::Compression {
    fn into_into_dart(self) -> crate::api::Compression {
        self
    }
}
// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for crate::api::RustyChaCha20Poly1305 {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.key.into_into_dart().into_dart(),
            self.compression.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::api::RustyChaCha20Poly1305
{
}
impl flutter_rust_bridge::IntoIntoDart<crate::api::RustyChaCha20Poly1305>
    for crate::api::RustyChaCha20Poly1305
{
    fn into_into_dart(self) -> crate::api::RustyChaCha20Poly1305 {
        self
    }
}
// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for crate::api::RustyXChaCha20Poly1305 {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.key.into_into_dart().into_dart(),
            self.compression.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::api::RustyXChaCha20Poly1305
{
}
impl flutter_rust_bridge::IntoIntoDart<crate::api::RustyXChaCha20Poly1305>
    for crate::api::RustyXChaCha20Poly1305
{
    fn into_into_dart(self) -> crate::api::RustyXChaCha20Poly1305 {
        self
    }
}

impl SseEncode for flutter_rust_bridge::for_generated::anyhow::Error {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(format!("{:?}", self), serializer);
    }
}

impl SseEncode for String {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Vec<u8>>::sse_encode(self.into_bytes(), serializer);
    }
}

impl SseEncode for crate::api::Compression {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        match self {
            crate::api::Compression::Uncompressed => {
                <i32>::sse_encode(0, serializer);
            }
            crate::api::Compression::Zstd { compression_level } => {
                <i32>::sse_encode(1, serializer);
                <Option<i32>>::sse_encode(compression_level, serializer);
            }
        }
    }
}

impl SseEncode for i32 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_i32::<NativeEndian>(self).unwrap();
    }
}

impl SseEncode for Vec<u8> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <u8>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Option<crate::api::Compression> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <crate::api::Compression>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<i32> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <i32>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<Vec<u8>> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <Vec<u8>>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for crate::api::RustyChaCha20Poly1305 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Vec<u8>>::sse_encode(self.key, serializer);
        <crate::api::Compression>::sse_encode(self.compression, serializer);
    }
}

impl SseEncode for crate::api::RustyXChaCha20Poly1305 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Vec<u8>>::sse_encode(self.key, serializer);
        <crate::api::Compression>::sse_encode(self.compression, serializer);
    }
}

impl SseEncode for u8 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u8(self).unwrap();
    }
}

impl SseEncode for () {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {}
}

impl SseEncode for bool {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u8(self as _).unwrap();
    }
}

#[cfg(not(target_family = "wasm"))]
#[path = "frb_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
