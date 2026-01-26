//! Fuzz target for YAML schema parsing
//!
//! Tests the SchemaRegistry::load_from_cache() function with arbitrary YAML input.
//!
//! Run with: cargo +nightly fuzz run fuzz_yaml_schema

#![no_main]

use libfuzzer_sys::fuzz_target;
use std::io::Write;
use tempfile::TempDir;
use ftm_types::SchemaRegistry;

fuzz_target!(|data: &[u8]| {
    // Only fuzz valid UTF-8 strings (YAML requires valid UTF-8)
    if let Ok(yaml_str) = std::str::from_utf8(data) {
        // Skip empty input
        if yaml_str.trim().is_empty() {
            return;
        }

        // Create a temporary directory for the schema file
        let temp_dir = match TempDir::new() {
            Ok(dir) => dir,
            Err(_) => return,
        };

        let schema_path = temp_dir.path().join("fuzz_schema.yaml");

        // Write the fuzzed YAML content
        if let Ok(mut file) = std::fs::File::create(&schema_path) {
            if file.write_all(data).is_err() {
                return;
            }
        } else {
            return;
        }

        // Try to load the schema - we don't care about errors, only panics
        let _ = SchemaRegistry::load_from_cache(temp_dir.path());
    }
});
