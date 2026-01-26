default: 
    just --list

download-sample-data:
    mkdir -p sample/
    wget --continue --show-progress -O sample/de_abgeordnetenwatch_sidejobs.ftm.json https://data.ftm.store/de_abgeordnetenwatch_sidejobs/entities.ftm.json
    wget --continue --show-progress --verbose -O sample/de_abgeordnetenwatch_sponsoring.ftm.json https://data.ftm.store/de_abgeordnetenwatch_sponsoring/entities.ftm.json

download-fmt-schema VERSION:
    mkdir -p schemas/{{VERSION}}/
    wget https://github.com/opensanctions/followthemoney/archive/refs/tags/v{{VERSION}}.zip
    unzip v{{VERSION}}.zip >/dev/null
    mv followthemoney-{{VERSION}}/followthemoney/schema/*.yaml schemas/{{VERSION}}/ >/dev/null
    rm -rf followthemoney-{{VERSION}}/ v{{VERSION}}.zip

test:
  cargo test --features builder

# Fuzzing targets
# Install cargo-fuzz first: cargo install cargo-fuzz
# Requires nightly Rust: rustup install nightly

# Run the JSON parser fuzzer (highest priority target)
fuzz-json:
    cargo +nightly fuzz run fuzz_ftm_json -- -max_len=65536

# Run the structured JSON fuzzer (better coverage)
fuzz-structured:
    cargo +nightly fuzz run fuzz_ftm_structured -- -max_len=4096

# Run the YAML schema parser fuzzer
fuzz-yaml:
    cargo +nightly fuzz run fuzz_yaml_schema -- -max_len=16384

# Run the inheritance resolution fuzzer
fuzz-inheritance:
    cargo +nightly fuzz run fuzz_inheritance -- -max_len=4096

# Run all fuzzers for a specified duration (in seconds, default 60)
fuzz-all duration="60":
    cargo +nightly fuzz run fuzz_ftm_json -- -max_total_time={{duration}} -max_len=65536
    cargo +nightly fuzz run fuzz_ftm_structured -- -max_total_time={{duration}} -max_len=4096
    cargo +nightly fuzz run fuzz_yaml_schema -- -max_total_time={{duration}} -max_len=16384
    cargo +nightly fuzz run fuzz_inheritance -- -max_total_time={{duration}} -max_len=4096

# List available fuzz targets
fuzz-list:
    cargo +nightly fuzz list

# Generate coverage report for a fuzz target
fuzz-coverage target:
    cargo +nightly fuzz coverage {{target}}
    @echo "Coverage data generated. Use 'cargo cov' to view the report."
