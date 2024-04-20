use divan::{counter::BytesCount, Bencher};
use embedded_rusty_chacha::api::{
    decrypt, decrypt_compressed, decrypt_from_file, encrypt, encrypt_compressed, encrypt_to_file,
    generate_cha_cha_20_key,
};
use rand::{thread_rng, Rng};
use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("=> System info");
    println!(
        "System name          : {}",
        System::name().unwrap_or_default()
    );
    println!(
        "System kernel version: {}",
        System::kernel_version().unwrap_or_default()
    );
    println!(
        "System OS version    : {}",
        System::os_version().unwrap_or_default()
    );
    println!("Total memory         : {} bytes", sys.total_memory());
    println!("Used memory          : {} bytes", sys.used_memory());
    println!("Total swap           : {} bytes", sys.total_swap());
    println!("Used swap            : {} bytes", sys.used_swap());
    println!("Number of CPUs       : {}", sys.cpus().len());
    for cpu in sys.cpus() {
        println!("                       {}", cpu.brand());
    }

    divan::main();
}

// different sizes from 1 MB to 180 MB
#[divan::bench(args = [1, 10, 100, 180], max_time = 2)]
fn bench_encrypt(bencher: Bencher, size: usize) {
    let size = size * 1024 * 1024; // MB to bytes

    bencher
        .counter(BytesCount::new(size))
        .with_inputs(|| (generate_cha_cha_20_key(), vec![0; size]))
        .bench_values(|(key, cleartext)| {
            let _ = encrypt(key, cleartext, None).unwrap();
        });
}

// different sizes from 1 MB to 180 MB
#[divan::bench(args = [1, 10, 100, 180], max_time = 2)]
fn bench_decrypt(bencher: divan::Bencher, size: usize) {
    let size = size * 1024 * 1024; // MB  to bytes
    let key = generate_cha_cha_20_key();

    bencher
        .counter(BytesCount::new(size))
        .with_inputs(|| {
            (
                key.clone(),
                encrypt(key.clone(), vec![0; size], None).unwrap(),
            )
        })
        .bench_values(|(key, ciphertext)| {
            let _ = decrypt(key, ciphertext, None).unwrap();
        });
}

// different sizes from 1 MB to 180 MB
#[divan::bench(args = [1, 10, 100, 180], max_time = 2)]
fn bench_encrypt_to_file(bencher: Bencher, size: usize) {
    let size = size * 1024 * 1024; // MB to bytes

    bencher
        .counter(BytesCount::new(size))
        .with_inputs(|| (generate_cha_cha_20_key(), vec![0; size]))
        .bench_values(|(key, cleartext)| {
            let _ = encrypt_to_file(key, cleartext, "test_file.bin".to_string(), None).unwrap();
        });
}

// different sizes from 1 MB to 180 MB
#[divan::bench(args = [1, 10, 100, 180], max_time = 2)]
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
                )
                .unwrap(),
            )
        })
        .bench_values(|(key, _)| {
            let _ = decrypt_from_file(key, "test_file.bin".to_string(), None).unwrap();
        });
}

#[divan::bench(args = [
    (0.0, -1000), (0.0, 3), (0.0, 7), (0.0, 12),
    (0.25, -1000), (0.25, 3), (0.25, 7), (0.25, 12),
    (0.5, -1000), (0.5, 3), (0.5, 7), (0.5, 12),
],
max_time = 3)]
fn bench_compress_encrypt(bencher: Bencher, args: (f64, i32)) {
    let size = 180 * 1024 * 1024; // 800 MB

    let (data_randomness, compression_level) = args;

    bencher
        .counter(BytesCount::new(size))
        .with_inputs(|| {
            (
                generate_cha_cha_20_key(),
                generate_test_data(size, data_randomness),
            )
        })
        .bench_values(|(key, cleartext)| {
            let _ = encrypt_compressed(key, cleartext, compression_level, None).unwrap();
        });
}

#[divan::bench(args = [
    (0.0, -1000), (0.0, 3), (0.0, 7), (0.0, 12),
    (0.25, -1000), (0.25, 3), (0.25, 7), (0.25, 12),
    (0.5, -1000), (0.5, 3), (0.5, 7), (0.5, 12),
],
max_time = 3)]
fn bench_decrypt_decompress(bencher: Bencher, args: (f64, i32)) {
    let size = 180 * 1024 * 1024; // 180 MB

    let key = generate_cha_cha_20_key();
    let (data_randomness, compression_level) = args;

    bencher
        .counter(BytesCount::new(size))
        .with_inputs(|| {
            (key.clone(), {
                let cleartext = generate_test_data(size, data_randomness);
                encrypt_compressed(key.clone(), cleartext, compression_level, None).unwrap()
            })
        })
        .bench_values(|(key, ciphertext)| {
            let _ = decrypt_compressed(key, ciphertext, None).unwrap();
        });
}

fn generate_test_data(size: usize, randomness: f64) -> Vec<u8> {
    let mut rng = thread_rng();
    let data: Vec<u8> = (0..size)
        .map(|_| {
            if rng.gen_bool(randomness) {
                rng.gen()
            } else {
                0
            }
        })
        .collect();
    data
}
