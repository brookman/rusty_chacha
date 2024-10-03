use anyhow::{Context, Ok, Result};
use lib_flutter_rust_bridge_codegen::codegen;
use lib_flutter_rust_bridge_codegen::codegen::Config;
use std::env;
use std::path::PathBuf;
extern crate cbindgen;

fn main() -> Result<()> {
    if env::var("RUN_BUILD_RS").map(|v| v != "1").unwrap_or(true) {
        println!("cargo:warning=build.rs is being skipped. Set env var RUN_BUILD_RS=1 to run it. On Windows run: $env:RUN_BUILD_RS = '1'; cargo build -r");
        return Ok(());
    }

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config = cbindgen::Config {
        language: cbindgen::Language::C,
        include_guard: Some("EMBEDDED_RUSTY_CHACHA".into()),
        ..Default::default()
    };
    cbindgen::generate_with_config(&crate_dir, config)
      .unwrap()
      .write_to_file("../../../target/EmbeddedRustyChacha.h");

    codegen::generate(
        Config::from_config_file("../flutter_rust_bridge.yaml")?
            .context("could not upen config")?,
        Default::default(),
    )?;

    // Format the generated Dart code
    _ = std::process::Command::new("dart")
        .arg("format")
        .arg("..")
        .spawn();

    Ok(())
}
