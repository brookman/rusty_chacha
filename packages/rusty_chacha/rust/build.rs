use std::env;

use lib_flutter_rust_bridge_codegen::codegen;
use lib_flutter_rust_bridge_codegen::codegen::Config;

fn main() {
    env::set_var("CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_DEBUG", "true");
    codegen::generate(
        Config::from_config_file("../flutter_rust_bridge.yaml")
            .unwrap()
            .unwrap(),
        Default::default(),
    )
    .unwrap();

    // Format the generated Dart code
    _ = std::process::Command::new("dart")
        .arg("format")
        .arg("..")
        .spawn();
}
