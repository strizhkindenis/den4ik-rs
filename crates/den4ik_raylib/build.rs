use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("Target OS not found.");

    println!("cargo:rerun-if-changed=build.rs");

    match target_os.as_str() {
        "linux" => {
            println!("cargo:rustc-link-lib=raylib");
        }
        "macos" => {
            println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
            println!("cargo:rustc-link-search=native=/usr/local/lib");
            println!("cargo:rustc-link-lib=raylib");
            println!("cargo:rustc-link-lib=framework=OpenGL");
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
        }
        _ => panic!("Unsupported OS. I did not authorize other platforms."),
    }
}
