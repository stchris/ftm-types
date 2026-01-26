//! Fuzz target for FtmEntity::from_ftm_json()
//!
//! This is the highest-priority fuzzing target as it parses untrusted JSON input.
//! Run with: cargo +nightly fuzz run fuzz_ftm_json

#![no_main]

use libfuzzer_sys::fuzz_target;
use ftm_types::FtmEntity;

fuzz_target!(|data: &[u8]| {
    // Only fuzz valid UTF-8 strings
    if let Ok(s) = std::str::from_utf8(data) {
        // We don't care about parse errors, only panics/crashes
        let _ = FtmEntity::from_ftm_json(s);
    }
});
