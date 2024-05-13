// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.33.

// Section: imports

use super::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate_io!();

// Section: dart2rust

impl CstDecode<flutter_rust_bridge::for_generated::anyhow::Error>
    for *mut wire_cst_list_prim_u_8_strict
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> flutter_rust_bridge::for_generated::anyhow::Error {
        unimplemented!()
    }
}
impl CstDecode<String> for *mut wire_cst_list_prim_u_8_strict {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> String {
        let vec: Vec<u8> = self.cst_decode();
        String::from_utf8(vec).unwrap()
    }
}
impl CstDecode<bool> for *mut bool {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> bool {
        unsafe { *flutter_rust_bridge::for_generated::box_from_leak_ptr(self) }
    }
}
impl CstDecode<crate::api::Compression> for *mut wire_cst_compression {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::Compression {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::api::Compression>::cst_decode(*wrap).into()
    }
}
impl CstDecode<i32> for *mut i32 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> i32 {
        unsafe { *flutter_rust_bridge::for_generated::box_from_leak_ptr(self) }
    }
}
impl CstDecode<crate::api::RustyChaCha20Poly1305> for *mut wire_cst_rusty_cha_cha_20_poly_1305 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::RustyChaCha20Poly1305 {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::api::RustyChaCha20Poly1305>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::api::RustyXChaCha20Poly1305> for *mut wire_cst_rusty_x_cha_cha_20_poly_1305 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::RustyXChaCha20Poly1305 {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::api::RustyXChaCha20Poly1305>::cst_decode(*wrap).into()
    }
}
impl CstDecode<u64> for *mut u64 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> u64 {
        unsafe { *flutter_rust_bridge::for_generated::box_from_leak_ptr(self) }
    }
}
impl CstDecode<crate::api::Compression> for wire_cst_compression {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::Compression {
        match self.tag {
            0 => crate::api::Compression::Uncompressed,
            1 => {
                let ans = unsafe { self.kind.Zstd };
                crate::api::Compression::Zstd {
                    compression_level: ans.compression_level.cst_decode(),
                }
            }
            _ => unreachable!(),
        }
    }
}
impl CstDecode<Vec<u8>> for *mut wire_cst_list_prim_u_8_loose {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<u8> {
        unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl CstDecode<Vec<u8>> for *mut wire_cst_list_prim_u_8_strict {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<u8> {
        unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl CstDecode<crate::api::RustyChaCha20Poly1305> for wire_cst_rusty_cha_cha_20_poly_1305 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::RustyChaCha20Poly1305 {
        crate::api::RustyChaCha20Poly1305 {
            key: self.key.cst_decode(),
            compression: self.compression.cst_decode(),
        }
    }
}
impl CstDecode<crate::api::RustyXChaCha20Poly1305> for wire_cst_rusty_x_cha_cha_20_poly_1305 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::RustyXChaCha20Poly1305 {
        crate::api::RustyXChaCha20Poly1305 {
            key: self.key.cst_decode(),
            compression: self.compression.cst_decode(),
        }
    }
}
impl NewWithNullPtr for wire_cst_compression {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: CompressionKind { nil__: () },
        }
    }
}
impl Default for wire_cst_compression {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_rusty_cha_cha_20_poly_1305 {
    fn new_with_null_ptr() -> Self {
        Self {
            key: core::ptr::null_mut(),
            compression: Default::default(),
        }
    }
}
impl Default for wire_cst_rusty_cha_cha_20_poly_1305 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_rusty_x_cha_cha_20_poly_1305 {
    fn new_with_null_ptr() -> Self {
        Self {
            key: core::ptr::null_mut(),
            compression: Default::default(),
        }
    }
}
impl Default for wire_cst_rusty_x_cha_cha_20_poly_1305 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_compress(
    port_: i64,
    data: *mut wire_cst_list_prim_u_8_loose,
    zstd_compression_level: i32,
) {
    wire_compress_impl(port_, data, zstd_compression_level)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_decompress(
    port_: i64,
    data: *mut wire_cst_list_prim_u_8_loose,
) {
    wire_decompress_impl(port_, data)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_create_internal(
    port_: i64,
    key: *mut wire_cst_list_prim_u_8_strict,
    compression: *mut wire_cst_compression,
) {
    wire_rusty_cha_cha_20_poly_1305_create_internal_impl(port_, key, compression)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_decrypt(
    port_: i64,
    that: *mut wire_cst_rusty_cha_cha_20_poly_1305,
    ciphertext: *mut wire_cst_list_prim_u_8_loose,
    aad: *mut wire_cst_list_prim_u_8_strict,
) {
    wire_rusty_cha_cha_20_poly_1305_decrypt_impl(port_, that, ciphertext, aad)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_decrypt_from_file(
    port_: i64,
    that: *mut wire_cst_rusty_cha_cha_20_poly_1305,
    file_path: *mut wire_cst_list_prim_u_8_strict,
    aad: *mut wire_cst_list_prim_u_8_strict,
    offset: *mut u64,
) {
    wire_rusty_cha_cha_20_poly_1305_decrypt_from_file_impl(port_, that, file_path, aad, offset)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_encrypt(
    port_: i64,
    that: *mut wire_cst_rusty_cha_cha_20_poly_1305,
    cleartext: *mut wire_cst_list_prim_u_8_loose,
    nonce: *mut wire_cst_list_prim_u_8_strict,
    aad: *mut wire_cst_list_prim_u_8_strict,
) {
    wire_rusty_cha_cha_20_poly_1305_encrypt_impl(port_, that, cleartext, nonce, aad)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_encrypt_to_file(
    port_: i64,
    that: *mut wire_cst_rusty_cha_cha_20_poly_1305,
    cleartext: *mut wire_cst_list_prim_u_8_loose,
    file_path: *mut wire_cst_list_prim_u_8_strict,
    nonce: *mut wire_cst_list_prim_u_8_strict,
    aad: *mut wire_cst_list_prim_u_8_strict,
    append: *mut bool,
) {
    wire_rusty_cha_cha_20_poly_1305_encrypt_to_file_impl(
        port_, that, cleartext, file_path, nonce, aad, append,
    )
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_generate_key(port_: i64) {
    wire_rusty_cha_cha_20_poly_1305_generate_key_impl(port_)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_generate_nonce(port_: i64) {
    wire_rusty_cha_cha_20_poly_1305_generate_nonce_impl(port_)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_create_internal(
    port_: i64,
    key: *mut wire_cst_list_prim_u_8_strict,
    compression: *mut wire_cst_compression,
) {
    wire_rusty_x_cha_cha_20_poly_1305_create_internal_impl(port_, key, compression)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_decrypt(
    port_: i64,
    that: *mut wire_cst_rusty_x_cha_cha_20_poly_1305,
    ciphertext: *mut wire_cst_list_prim_u_8_loose,
    aad: *mut wire_cst_list_prim_u_8_strict,
) {
    wire_rusty_x_cha_cha_20_poly_1305_decrypt_impl(port_, that, ciphertext, aad)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_decrypt_from_file(
    port_: i64,
    that: *mut wire_cst_rusty_x_cha_cha_20_poly_1305,
    file_path: *mut wire_cst_list_prim_u_8_strict,
    aad: *mut wire_cst_list_prim_u_8_strict,
    offset: *mut u64,
) {
    wire_rusty_x_cha_cha_20_poly_1305_decrypt_from_file_impl(port_, that, file_path, aad, offset)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_encrypt(
    port_: i64,
    that: *mut wire_cst_rusty_x_cha_cha_20_poly_1305,
    cleartext: *mut wire_cst_list_prim_u_8_loose,
    nonce: *mut wire_cst_list_prim_u_8_strict,
    aad: *mut wire_cst_list_prim_u_8_strict,
) {
    wire_rusty_x_cha_cha_20_poly_1305_encrypt_impl(port_, that, cleartext, nonce, aad)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_encrypt_to_file(
    port_: i64,
    that: *mut wire_cst_rusty_x_cha_cha_20_poly_1305,
    cleartext: *mut wire_cst_list_prim_u_8_loose,
    file_path: *mut wire_cst_list_prim_u_8_strict,
    nonce: *mut wire_cst_list_prim_u_8_strict,
    aad: *mut wire_cst_list_prim_u_8_strict,
    append: *mut bool,
) {
    wire_rusty_x_cha_cha_20_poly_1305_encrypt_to_file_impl(
        port_, that, cleartext, file_path, nonce, aad, append,
    )
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_generate_key(port_: i64) {
    wire_rusty_x_cha_cha_20_poly_1305_generate_key_impl(port_)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_generate_nonce(port_: i64) {
    wire_rusty_x_cha_cha_20_poly_1305_generate_nonce_impl(port_)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_cst_new_box_autoadd_bool(value: bool) -> *mut bool {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_cst_new_box_autoadd_compression() -> *mut wire_cst_compression
{
    flutter_rust_bridge::for_generated::new_leak_box_ptr(wire_cst_compression::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_cst_new_box_autoadd_i_32(value: i32) -> *mut i32 {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_cst_new_box_autoadd_rusty_cha_cha_20_poly_1305(
) -> *mut wire_cst_rusty_cha_cha_20_poly_1305 {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(
        wire_cst_rusty_cha_cha_20_poly_1305::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_cst_new_box_autoadd_rusty_x_cha_cha_20_poly_1305(
) -> *mut wire_cst_rusty_x_cha_cha_20_poly_1305 {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(
        wire_cst_rusty_x_cha_cha_20_poly_1305::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_cst_new_box_autoadd_u_64(value: u64) -> *mut u64 {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_cst_new_list_prim_u_8_loose(
    len: i32,
) -> *mut wire_cst_list_prim_u_8_loose {
    let ans = wire_cst_list_prim_u_8_loose {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_cst_new_list_prim_u_8_strict(
    len: i32,
) -> *mut wire_cst_list_prim_u_8_strict {
    let ans = wire_cst_list_prim_u_8_strict {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(ans)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_compression {
    tag: i32,
    kind: CompressionKind,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CompressionKind {
    Zstd: wire_cst_Compression_Zstd,
    nil__: (),
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_Compression_Zstd {
    compression_level: *mut i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_prim_u_8_loose {
    ptr: *mut u8,
    len: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_prim_u_8_strict {
    ptr: *mut u8,
    len: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_rusty_cha_cha_20_poly_1305 {
    key: *mut wire_cst_list_prim_u_8_strict,
    compression: wire_cst_compression,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_rusty_x_cha_cha_20_poly_1305 {
    key: *mut wire_cst_list_prim_u_8_strict,
    compression: wire_cst_compression,
}
