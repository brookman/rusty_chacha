use divan::{counter::BytesCount, Bencher};
use embedded_rusty_chacha::api::{
    decrypt, decrypt_from_file, encrypt, encrypt_to_file, generate_cha_cha_20_key,
};
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
