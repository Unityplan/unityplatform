fn main() {
    // Expose package version to the binary
    println!("cargo:rustc-env=SERVICE_VERSION={}", env!("CARGO_PKG_VERSION"));
    println!("cargo:rustc-env=SERVICE_NAME={}", env!("CARGO_PKG_NAME"));
    
    // Expose build timestamp (using std to avoid build dependencies)
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);
    
    // Expose git information if available
    if let Ok(output) = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
    {
        if output.status.success() {
            let git_hash = String::from_utf8_lossy(&output.stdout);
            println!("cargo:rustc-env=GIT_HASH={}", git_hash.trim());
        }
    }
    
    // Rerun if git HEAD changes
    println!("cargo:rerun-if-changed=../.git/HEAD");
}
