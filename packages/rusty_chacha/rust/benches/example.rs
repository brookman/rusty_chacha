use divan::{counter::BytesCount, Bencher};
use embedded_rusty_chacha::api::{
    decrypt, decrypt_from_file, encrypt, encrypt_to_file, generate_cha_cha_20_key,
};

fn main() {
    divan::main();
}

// different sizes from 1 MB to 200 MB
#[divan::bench(args = [1, 10, 100, 200], max_time = 2)]
fn bench_encrypt(bencher: Bencher, size: usize) {
    let size = size * 1024 * 1024; // MB to bytes

    bencher
        .counter(BytesCount::new(size))
        .with_inputs(|| (generate_cha_cha_20_key(), vec![0; size]))
        .bench_values(|(key, cleartext)| {
            let _ = encrypt(key, cleartext, None, None).unwrap();
        });
}

// different sizes from 1 MB to 200 MB
#[divan::bench(args = [1, 10, 100, 200], max_time = 2)]
fn bench_decrypt(bencher: divan::Bencher, size: usize) {
    let size = size * 1024 * 1024; // MB  to bytes
    let key = generate_cha_cha_20_key();

    bencher
        .counter(BytesCount::new(size))
        .with_inputs(|| {
            (
                key.clone(),
                encrypt(key.clone(), vec![0; size], None, None).unwrap(),
            )
        })
        .bench_values(|(key, ciphertext)| {
            let _ = decrypt(key, ciphertext, None).unwrap();
        });
}

// different sizes from 1 MB to 200 MB
#[divan::bench(args = [1, 10, 100, 200], max_time = 2)]
fn bench_encrypt_to_file(bencher: Bencher, size: usize) {
    let size = size * 1024 * 1024; // MB to bytes

    bencher
        .counter(BytesCount::new(size))
        .with_inputs(|| (generate_cha_cha_20_key(), vec![0; size]))
        .bench_values(|(key, cleartext)| {
            let _ =
                encrypt_to_file(key, cleartext, "test_file.bin".to_string(), None, None).unwrap();
        });
}

// different sizes from 1 MB to 200 MB
#[divan::bench(args = [1, 10, 100, 200], max_time = 2)]
fn bench_decrypt_from_file(bencher: Bencher, size: usize) {
    let size = size * 1024 * 1024; // MB to bytes
    let key = generate_cha_cha_20_key();

    bencher
        .counter(BytesCount::new(size))
        .with_inputs(|| {
            (
                key.clone(),
                encrypt_to_file(
                    key.clone(),
                    vec![0; size],
                    "test_file.bin".to_string(),
                    None,
                    None,
                )
                .unwrap(),
            )
        })
        .bench_values(|(key, _)| {
            let _ = decrypt_from_file(key, "test_file.bin".to_string(), None).unwrap();
        });
}
