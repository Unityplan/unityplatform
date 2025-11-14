use std::env;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Service version from Cargo.toml
    println!(
        "cargo:rustc-env=SERVICE_VERSION={}",
        env::var("CARGO_PKG_VERSION").unwrap()
    );

    // Service name from Cargo.toml
    println!(
        "cargo:rustc-env=SERVICE_NAME={}",
        env::var("CARGO_PKG_NAME").unwrap()
    );

    // Build timestamp (Unix epoch seconds) - using std to avoid build dependencies
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);

    // Git commit hash (short)
    if let Ok(output) = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
    {
        if output.status.success() {
            let git_hash = String::from_utf8_lossy(&output.stdout);
            println!("cargo:rustc-env=GIT_HASH={}", git_hash.trim());
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
}
