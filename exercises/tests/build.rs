//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Step 1: Set the environment variable `TEST_FOO` to the current timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // Current Unix timestamp

    // Set the TEST_FOO environment variable
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // Step 2: Enable the "pass" feature for the second exercise
    println!("cargo:rustc-cfg=feature=\"pass\"");


    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    
}
