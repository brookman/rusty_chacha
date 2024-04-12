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
                                      uintptr_t secrets,
                                      struct wire_cst_list_prim_u_8_loose *encrypted);

void frbgen_rusty_chacha_wire_encrypt(int64_t port_,
                                      uintptr_t secrets,
                                      struct wire_cst_list_prim_u_8_loose *clear);

void frbgen_rusty_chacha_wire_encrypt_and_write(int64_t port_,
                                                struct wire_cst_list_prim_u_8_loose *clear,
                                                struct wire_cst_list_prim_u_8_strict *file_path);

void frbgen_rusty_chacha_wire_generate_random_cha_cha20_key(int64_t port_);

void frbgen_rusty_chacha_wire_generate_random_cha_cha20_nonce(int64_t port_);

void frbgen_rusty_chacha_wire_generate_secrets(int64_t port_);

void frbgen_rusty_chacha_wire_read_and_decrypt(int64_t port_,
                                               uintptr_t secrets,
                                               struct wire_cst_list_prim_u_8_strict *file_path);

void frbgen_rusty_chacha_wire_read_clear_text(int64_t port_,
                                              struct wire_cst_list_prim_u_8_strict *file_path);

void frbgen_rusty_chacha_wire_read_encrypted(int64_t port_,
                                             struct wire_cst_list_prim_u_8_strict *file_path);

void frbgen_rusty_chacha_wire_write(int64_t port_,
                                    struct wire_cst_list_prim_u_8_loose *encrypted,
                                    struct wire_cst_list_prim_u_8_strict *file_path);

void frbgen_rusty_chacha_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockSecrets(const void *ptr);

void frbgen_rusty_chacha_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockSecrets(const void *ptr);

struct wire_cst_list_prim_u_8_loose *frbgen_rusty_chacha_cst_new_list_prim_u_8_loose(int32_t len);

struct wire_cst_list_prim_u_8_strict *frbgen_rusty_chacha_cst_new_list_prim_u_8_strict(int32_t len);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_cst_new_list_prim_u_8_loose);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_cst_new_list_prim_u_8_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockSecrets);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockSecrets);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_decrypt);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_encrypt);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_encrypt_and_write);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_generate_random_cha_cha20_key);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_generate_random_cha_cha20_nonce);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_generate_secrets);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_read_and_decrypt);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_read_clear_text);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_read_encrypted);
    dummy_var ^= ((int64_t) (void*) frbgen_rusty_chacha_wire_write);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
