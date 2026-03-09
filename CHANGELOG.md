# Changelog

All notable changes to this project will be documented in this file.
## [0.3.0] - 2026-03-09

### New

- Fix schema download hint
- Add followthemoney 4.6.0 schema compatibility

## [0.2.0] - 2026-03-06

### New

- Add Changelog to release process
- More convenient builder API
- Merge pull request #12 from stchris/api-ergonomics
- Add git cliff config
- Chore: Release ftm-types version 0.2.0

## [0.1.1] - 2026-03-05

### New

- Added a to_ftm_json() equivalent for JSON serialization
- Chore: Release ftm-types version 0.1.1

## [0.1.0] - 2026-03-05

### New

- Initial commit
- Rand
- Sample jsonl files
- Move download schema script to justfile
- Builder API
- Make builder feature default
- Clippy
- Explore basic usage papercuts
- Make builder respect required fields
- Updates to docs, todos
- Unignore builder test
- Actions
- Setup toolchain
- Fix syntax maybe
- Surely ...
- Oops undo test mess
- Fix tests
- Skip test for which data download needs to be added
- Job concurrency, cancel in progress
- Fix: runner name
- Clippy
- Add zstd compressed sample files and unignore tests
- Bug: fix f64 and missing name parsing as well as proper peeking into schema for FtmEntity::from_ftm_json
- Cargo fmt
- Add integration tests for FtmEntity::from_ftm_json using uvx ftm-random
- Switch to uvx --refresh-package ftm-random >=0.4.0 in test helper
- Add Documentation schema and regenerate; fix justfile for .yml files
- Add uv to the Test Suite CI job
- Install pkg-config and libicu-dev in Test Suite CI job
- Run apt-get update before installing ICU dependencies
- Merge pull request #2 from stchris/claude/test-ftm-entity-parsing-NOmzw
- Assume string type on properties without a type declaration
- Significantly faster test runs.
- Add prek
- Remove the "rand" feature.
- Rm schemas/4.4.1
- Add a builder example
- Simplify ftm-random invocation
- README polishing
- Skip doctests (which we don't have yet)
- More README tweaks
- Better wording
- Add cargo dist


