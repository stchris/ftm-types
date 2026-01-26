# Fuzzing ftm-types

This directory contains fuzz targets for testing ftm-types with [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) (libFuzzer).

## Prerequisites

1. Install Rust nightly:
   ```bash
   rustup install nightly
   ```

2. Install cargo-fuzz:
   ```bash
   cargo install cargo-fuzz
   ```

## Fuzz Targets

| Target | Description | Priority |
|--------|-------------|----------|
| `fuzz_ftm_json` | Raw JSON parsing via `FtmEntity::from_ftm_json()` | High |
| `fuzz_ftm_structured` | Structured FTM JSON with valid schema names | High |
| `fuzz_yaml_schema` | YAML schema file parsing | Medium |
| `fuzz_inheritance` | Schema inheritance resolution | Medium |

## Running Fuzzers

### Using just (recommended)

```bash
# Run JSON parser fuzzer
just fuzz-json

# Run all fuzzers for 60 seconds each
just fuzz-all

# Run all fuzzers for 5 minutes each
just fuzz-all 300

# List available targets
just fuzz-list
```

### Using cargo-fuzz directly

```bash
# Run a specific target
cargo +nightly fuzz run fuzz_ftm_json

# Run with options
cargo +nightly fuzz run fuzz_ftm_json -- \
    -max_len=65536 \      # Max input size
    -timeout=5 \          # Timeout per test case
    -max_total_time=300   # Run for 5 minutes

# Run with multiple jobs
cargo +nightly fuzz run fuzz_ftm_json -- -jobs=4 -workers=4
```

## Seed Corpus

The `corpus/fuzz_ftm_json/` directory contains seed inputs to help the fuzzer:

- `valid_person.json` - Valid Person entity
- `valid_company.json` - Valid Company entity
- `valid_sanction.json` - Valid Sanction entity
- `empty_properties.json` - Entity with no properties
- `multi_values.json` - Properties with multiple values
- `unicode_content.json` - Unicode characters in values
- `special_chars.json` - Escaped special characters
- `minimal.json` - Minimal valid input

## Investigating Crashes

When the fuzzer finds a crash, it saves the input to `fuzz/artifacts/<target>/`:

```bash
# Reproduce a crash
cargo +nightly fuzz run fuzz_ftm_json fuzz/artifacts/fuzz_ftm_json/crash-xxxxx

# Minimize the crash input
cargo +nightly fuzz tmin fuzz_ftm_json fuzz/artifacts/fuzz_ftm_json/crash-xxxxx
```

## Coverage

Generate coverage reports to see what code paths are being tested:

```bash
# Generate coverage data
cargo +nightly fuzz coverage fuzz_ftm_json

# View with cargo-cov (install: cargo install cargo-cov)
cargo cov -- show fuzz/target/*/coverage/fuzz_ftm_json/ \
    --format=html \
    --output-dir=coverage-report
```

## Continuous Fuzzing

For CI/CD integration, run fuzzers with a time limit:

```bash
# Run for 5 minutes
cargo +nightly fuzz run fuzz_ftm_json -- -max_total_time=300

# Exit code 0 = no crashes found
# Exit code 77 = crash found (artifacts saved)
```
