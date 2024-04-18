// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.32.

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
impl CstDecode<i32> for *mut i32 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> i32 {
        unsafe { *flutter_rust_bridge::for_generated::box_from_leak_ptr(self) }
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
pub extern "C" fn frbgen_rusty_chacha_wire_decrypt(
    port_: i64,
    key: *mut wire_cst_list_prim_u_8_loose,
    ciphertext: *mut wire_cst_list_prim_u_8_loose,
    aad: *mut wire_cst_list_prim_u_8_strict,
) {
    wire_decrypt_impl(port_, key, ciphertext, aad)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_decrypt_from_file(
    port_: i64,
    key: *mut wire_cst_list_prim_u_8_loose,
    file_path: *mut wire_cst_list_prim_u_8_strict,
    aad: *mut wire_cst_list_prim_u_8_strict,
) {
    wire_decrypt_from_file_impl(port_, key, file_path, aad)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_encrypt(
    port_: i64,
    key: *mut wire_cst_list_prim_u_8_loose,
    cleartext: *mut wire_cst_list_prim_u_8_loose,
    aad: *mut wire_cst_list_prim_u_8_strict,
    zstd_compression_level: *mut i32,
) {
    wire_encrypt_impl(port_, key, cleartext, aad, zstd_compression_level)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_encrypt_to_file(
    port_: i64,
    key: *mut wire_cst_list_prim_u_8_loose,
    cleartext: *mut wire_cst_list_prim_u_8_loose,
    file_path: *mut wire_cst_list_prim_u_8_strict,
    aad: *mut wire_cst_list_prim_u_8_strict,
    zstd_compression_level: *mut i32,
) {
    wire_encrypt_to_file_impl(
        port_,
        key,
        cleartext,
        file_path,
        aad,
        zstd_compression_level,
    )
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_generate_cha_cha_20_key(port_: i64) {
    wire_generate_cha_cha_20_key_impl(port_)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_generate_cha_cha_20_nonce(port_: i64) {
    wire_generate_cha_cha_20_nonce_impl(port_)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_read_file(
    port_: i64,
    file_path: *mut wire_cst_list_prim_u_8_strict,
) {
    wire_read_file_impl(port_, file_path)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_wire_write_file(
    port_: i64,
    data: *mut wire_cst_list_prim_u_8_loose,
    file_path: *mut wire_cst_list_prim_u_8_strict,
) {
    wire_write_file_impl(port_, data, file_path)
}

#[no_mangle]
pub extern "C" fn frbgen_rusty_chacha_cst_new_box_autoadd_i_32(value: i32) -> *mut i32 {
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
