use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=raylib-6.0/src");

    let status = Command::new("make")
        .arg("PLATFORM=PLATFORM_DESKTOP")
        .current_dir("raylib-6.0/src")
        .status()
        .expect("Failed to execute make");

    if !status.success() {
        panic!("Failed to build raylib");
    }

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}/raylib-6.0/src", manifest_dir);
    println!("cargo:rustc-link-lib=static=raylib");

    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("Target OS not found.");
    if target_os == "macos" {
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
    }
}
