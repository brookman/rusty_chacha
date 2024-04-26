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

typedef struct wire_cst_Compression_Zstd {
  int32_t *compression_level;
} wire_cst_Compression_Zstd;

typedef union CompressionKind {
  struct wire_cst_Compression_Zstd Zstd;
} CompressionKind;

typedef struct wire_cst_compression {
  int32_t tag;
  union CompressionKind kind;
} wire_cst_compression;

typedef struct wire_cst_rusty_cha_cha_20_poly_1305 {
  struct wire_cst_list_prim_u_8_strict *key;
  struct wire_cst_compression compression;
} wire_cst_rusty_cha_cha_20_poly_1305;

typedef struct wire_cst_rusty_x_cha_cha_20_poly_1305 {
  struct wire_cst_list_prim_u_8_strict *key;
  struct wire_cst_compression compression;
} wire_cst_rusty_x_cha_cha_20_poly_1305;

void frbgen_rusty_chacha_wire_compress(int64_t port_,
                                       struct wire_cst_list_prim_u_8_loose *data,
                                       int32_t zstd_compression_level);

void frbgen_rusty_chacha_wire_decompress(int64_t port_, struct wire_cst_list_prim_u_8_loose *data);

void frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_create_internal(int64_t port_,
                                                                         struct wire_cst_list_prim_u_8_strict *key,
                                                                         struct wire_cst_compression *compression);

void frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_decrypt(int64_t port_,
                                                                 struct wire_cst_rusty_cha_cha_20_poly_1305 *that,
                                                                 struct wire_cst_list_prim_u_8_loose *ciphertext,
                                                                 struct wire_cst_list_prim_u_8_strict *aad);

void frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_decrypt_from_file(int64_t port_,
                                                                           struct wire_cst_rusty_cha_cha_20_poly_1305 *that,
                                                                           struct wire_cst_list_prim_u_8_strict *file_path,
                                                                           struct wire_cst_list_prim_u_8_strict *aad);

void frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_encrypt(int64_t port_,
                                                                 struct wire_cst_rusty_cha_cha_20_poly_1305 *that,
                                                                 struct wire_cst_list_prim_u_8_loose *cleartext,
                                                                 struct wire_cst_list_prim_u_8_strict *nonce,
                                                                 struct wire_cst_list_prim_u_8_strict *aad);

void frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_encrypt_to_file(int64_t port_,
                                                                         struct wire_cst_rusty_cha_cha_20_poly_1305 *that,
                                                                         struct wire_cst_list_prim_u_8_loose *cleartext,
                                                                         struct wire_cst_list_prim_u_8_strict *file_path,
                                                                         struct wire_cst_list_prim_u_8_strict *nonce,
                                                                         struct wire_cst_list_prim_u_8_strict *aad);

void frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_generate_key(int64_t port_);

void frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_generate_nonce(int64_t port_);

void frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_create_internal(int64_t port_,
                                                                           struct wire_cst_list_prim_u_8_strict *key,
                                                                           struct wire_cst_compression *compression);

void frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_decrypt(int64_t port_,
                                                                   struct wire_cst_rusty_x_cha_cha_20_poly_1305 *that,
                                                                   struct wire_cst_list_prim_u_8_loose *ciphertext,
                                                                   struct wire_cst_list_prim_u_8_strict *aad);

void frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_decrypt_from_file(int64_t port_,
                                                                             struct wire_cst_rusty_x_cha_cha_20_poly_1305 *that,
                                                                             struct wire_cst_list_prim_u_8_strict *file_path,
                                                                             struct wire_cst_list_prim_u_8_strict *aad);

void frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_encrypt(int64_t port_,
                                                                   struct wire_cst_rusty_x_cha_cha_20_poly_1305 *that,
                                                                   struct wire_cst_list_prim_u_8_loose *cleartext,
                                                                   struct wire_cst_list_prim_u_8_strict *nonce,
                                                                   struct wire_cst_list_prim_u_8_strict *aad);

void frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_encrypt_to_file(int64_t port_,
                                                                           struct wire_cst_rusty_x_cha_cha_20_poly_1305 *that,
                                                                           struct wire_cst_list_prim_u_8_loose *cleartext,
                                                                           struct wire_cst_list_prim_u_8_strict *file_path,
                                                                           struct wire_cst_list_prim_u_8_strict *nonce,
                                                                           struct wire_cst_list_prim_u_8_strict *aad);

void frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_generate_key(int64_t port_);

void frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_generate_nonce(int64_t port_);

struct wire_cst_compression *frbgen_rusty_chacha_cst_new_box_autoadd_compression(void);

int32_t *frbgen_rusty_chacha_cst_new_box_autoadd_i_32(int32_t value);

struct wire_cst_rusty_cha_cha_20_poly_1305 *frbgen_rusty_chacha_cst_new_box_autoadd_rusty_cha_cha_20_poly_1305(void);

struct wire_cst_rusty_x_cha_cha_20_poly_1305 *frbgen_rusty_chacha_cst_new_box_autoadd_rusty_x_cha_cha_20_poly_1305(void);

struct wire_cst_list_prim_u_8_loose *frbgen_rusty_chacha_cst_new_list_prim_u_8_loose(int32_t len);

struct wire_cst_list_prim_u_8_strict *frbgen_rusty_chacha_cst_new_list_prim_u_8_strict(int32_t len);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_cst_new_box_autoadd_compression);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_cst_new_box_autoadd_i_32);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_cst_new_box_autoadd_rusty_cha_cha_20_poly_1305);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_cst_new_box_autoadd_rusty_x_cha_cha_20_poly_1305);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_cst_new_list_prim_u_8_loose);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_cst_new_list_prim_u_8_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_compress);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_decompress);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_create_internal);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_decrypt);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_decrypt_from_file);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_encrypt);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_encrypt_to_file);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_generate_key);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_rusty_cha_cha_20_poly_1305_generate_nonce);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_create_internal);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_decrypt);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_decrypt_from_file);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_encrypt);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_encrypt_to_file);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_generate_key);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_rusty_x_cha_cha_20_poly_1305_generate_nonce);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
