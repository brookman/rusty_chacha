#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
// EXTRA BEGIN
typedef struct DartCObject *WireSyncRust2DartDco;
typedef struct WireSyncRust2DartSse {
  uint8_t *ptr;
  int32_t len;
} WireSyncRust2DartSse;

typedef int64_t DartPort;
typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);
void store_dart_post_cobject(DartPostCObjectFnType ptr);
// EXTRA END
typedef struct _Dart_Handle* Dart_Handle;

typedef struct wire_cst_list_prim_u_8_loose {
  uint8_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_8_loose;

typedef struct wire_cst_list_prim_u_8_strict {
  uint8_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_8_strict;

void frbgen_rusty_chacha_wire_decrypt(int64_t port_,
                                      struct wire_cst_list_prim_u_8_loose *key,
                                      struct wire_cst_list_prim_u_8_loose *ciphertext,
                                      struct wire_cst_list_prim_u_8_strict *aad);

void frbgen_rusty_chacha_wire_decrypt_from_file(int64_t port_,
                                                struct wire_cst_list_prim_u_8_loose *key,
                                                struct wire_cst_list_prim_u_8_strict *file_path,
                                                struct wire_cst_list_prim_u_8_strict *aad);

void frbgen_rusty_chacha_wire_encrypt(int64_t port_,
                                      struct wire_cst_list_prim_u_8_loose *key,
                                      struct wire_cst_list_prim_u_8_loose *cleartext,
                                      struct wire_cst_list_prim_u_8_strict *aad,
                                      int32_t *zstd_compression_level);

void frbgen_rusty_chacha_wire_encrypt_to_file(int64_t port_,
                                              struct wire_cst_list_prim_u_8_loose *key,
                                              struct wire_cst_list_prim_u_8_loose *cleartext,
                                              struct wire_cst_list_prim_u_8_strict *file_path,
                                              struct wire_cst_list_prim_u_8_strict *aad,
                                              int32_t *zstd_compression_level);

void frbgen_rusty_chacha_wire_generate_cha_cha_20_key(int64_t port_);

void frbgen_rusty_chacha_wire_generate_cha_cha_20_nonce(int64_t port_);

void frbgen_rusty_chacha_wire_read_file(int64_t port_,
                                        struct wire_cst_list_prim_u_8_strict *file_path);

void frbgen_rusty_chacha_wire_write_file(int64_t port_,
                                         struct wire_cst_list_prim_u_8_loose *data,
                                         struct wire_cst_list_prim_u_8_strict *file_path);

int32_t *frbgen_rusty_chacha_cst_new_box_autoadd_i_32(int32_t value);

struct wire_cst_list_prim_u_8_loose *frbgen_rusty_chacha_cst_new_list_prim_u_8_loose(int32_t len);

struct wire_cst_list_prim_u_8_strict *frbgen_rusty_chacha_cst_new_list_prim_u_8_strict(int32_t len);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_cst_new_box_autoadd_i_32);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_cst_new_list_prim_u_8_loose);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_cst_new_list_prim_u_8_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_decrypt);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_decrypt_from_file);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_encrypt);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_encrypt_to_file);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_generate_cha_cha_20_key);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_generate_cha_cha_20_nonce);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_read_file);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_write_file);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
