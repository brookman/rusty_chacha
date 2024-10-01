use divan::{counter::BytesCount, Bencher};
use embedded_rusty_chacha::api::{Compression, RustyChaCha20Poly1305};
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

    if cfg!(chacha20_force_neon) {
        println!("chacha20_force_neon : yes");
    } else {
        println!("chacha20_force_neon : no");
    }
    if cfg!(target_arch = "aarch64") {
        println!("aarch64             : yes");
    } else {
        println!("aarch64             : no");
    }
    if cfg!(target_feature = "neon") {
        println!("target_feature neon : yes");
    } else {
        println!("target_feature neon : no");
    }

    divan::main();
}

// different sizes from 1 MB to 180 MB
#[divan::bench(args = [1, 10, 100, 180], max_time = 2)]
fn bench_encrypt(bencher: Bencher, size: usize) {
    let size = size * 1024 * 1024; // MB to bytes

    bencher
        .counter(BytesCount::new(size))
        .with_inputs(|| {
            let cipher = RustyChaCha20Poly1305::create_internal(None, None).unwrap();
            let cleartext = vec![0; size];
            (cipher, cleartext)
        })
        .bench_values(|(cipher, cleartext)| {
            let _ = cipher.encrypt(cleartext, None, None).unwrap();
        });
}

// different sizes from 1 MB to 180 MB
#[divan::bench(args = [1, 10, 100, 180], max_time = 2)]
fn bench_decrypt(bencher: divan::Bencher, size: usize) {
    let size = size * 1024 * 1024; // MB  to bytes
    bencher
        .counter(BytesCount::new(size))
        .with_inputs(|| {
            let cipher = RustyChaCha20Poly1305::create_internal(None, None).unwrap();
            let ciphertext = cipher.encrypt(vec![0; size], None, None).unwrap();
            (cipher, ciphertext)
        })
        .bench_values(|(cipher, ciphertext)| {
            let _ = cipher.decrypt(ciphertext, None).unwrap();
        });
}

// different sizes from 1 MB to 180 MB
#[divan::bench(args = [1, 10, 100, 180], max_time = 2)]
fn bench_encrypt_to_file(bencher: Bencher, size: usize) {
    let size = size * 1024 * 1024; // MB to bytes

    bencher
        .counter(BytesCount::new(size))
        .with_inputs(|| {
            let cipher = RustyChaCha20Poly1305::create_internal(None, None).unwrap();
            let cleartext = vec![0; size];
            (cipher, cleartext)
        })
        .bench_values(|(cipher, cleartext)| {
            let _ = cipher
                .encrypt_to_file(cleartext, "test_file.bin".to_string(), None, None, None)
                .unwrap();
        });
}

// different sizes from 1 MB to 180 MB
#[divan::bench(args = [1, 10, 100, 180], max_time = 2)]
fn bench_decrypt_from_file(bencher: Bencher, size: usize) {
    let size = size * 1024 * 1024; // MB to bytes

    bencher
        .counter(BytesCount::new(size))
        .with_inputs(|| {
            let cipher = RustyChaCha20Poly1305::create_internal(None, None).unwrap();
            cipher
                .encrypt_to_file(vec![0; size], "test_file.bin".to_string(), None, None, None)
                .unwrap();
            cipher
        })
        .bench_values(|cipher| {
            let _ = cipher
                .decrypt_from_file("test_file.bin".to_string(), None, None)
                .unwrap();
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
    let compression_level = Some(compression_level);

    bencher
        .counter(BytesCount::new(size))
        .with_inputs(|| {
            let cipher = RustyChaCha20Poly1305::create_internal(
                None,
                Some(Compression::Zstd { compression_level }),
            )
            .unwrap();
            (cipher, generate_test_data(size, data_randomness))
        })
        .bench_values(|(cipher, cleartext)| {
            let _ = cipher.encrypt(cleartext, None, None).unwrap();
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
    let (data_randomness, compression_level) = args;
    let compression_level = Some(compression_level);

    bencher
        .counter(BytesCount::new(size))
        .with_inputs(|| {
            let cipher = RustyChaCha20Poly1305::create_internal(
                None,
                Some(Compression::Zstd { compression_level }),
            )
            .unwrap();
            let cleartext = generate_test_data(size, data_randomness);
            let ciphertext = cipher.encrypt(cleartext, None, None).unwrap();
            (cipher, ciphertext)
        })
        .bench_values(|(cipher, ciphertext)| {
            let _ = cipher.decrypt(ciphertext, None).unwrap();
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
