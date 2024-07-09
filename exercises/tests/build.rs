//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Set the TEST_FOO environment variable with the current timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // Set the "pass" feature if TESTS8 environment variable is set
    if env::var("TESTS8").is_ok() {
        println!("cargo:rustc-cfg=feature=\"pass\"");
    }
}
