/// Integration tests that use `uvx ftm-random` to generate live FTM entities
/// and verify that `FtmEntity::from_ftm_json` can parse them correctly.
use ftm_types::FtmEntity;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

// ── helpers ───────────────────────────────────────────────────────────────────

/// Derive a pseudo-random count in [min, max] from the current time.
/// This avoids adding the `rand` crate as a dev-dependency.
fn random_count(min: u64, max: u64) -> usize {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as u64;
    (min + nanos % (max - min + 1)) as usize
}

/// Extract the `"schema"` field value directly from a raw JSON line without a
/// full parse — the field is always near the start of the object.
fn schema_from_json(json: &str) -> &str {
    let key = "\"schema\":";
    let start = json.find(key).expect("no \"schema\" key in JSON") + key.len();
    let rest = json[start..].trim_start();
    assert!(rest.starts_with('"'), "schema value is not a JSON string");
    let inner = &rest[1..];
    let end = inner.find('"').expect("unterminated schema string");
    &inner[..end]
}

/// Run `uvx --refresh-package ftm-random ftm-random` (v0.4.0+, which only
/// emits concrete schemas with `--random-schema`) with the given extra
/// arguments and return the stdout lines (one JSON entity per line).
/// Panics with a descriptive message if the tool is unavailable or exits
/// non-zero.
fn run_ftm_random(extra_args: &[&str]) -> Vec<String> {
    let mut cmd = Command::new("uvx");
    cmd.args(["--refresh-package", "ftm-random", "--from", "ftm-random>=0.4.0", "ftm-random"]);
    for arg in extra_args {
        cmd.arg(arg);
    }

    let output = cmd.output().expect(
        "failed to spawn `uvx ftm-random` — make sure `uv` is installed and \
         `ftm-random` is available via uvx",
    );

    assert!(
        output.status.success(),
        "`uvx ftm-random {:?}` exited with {}\nstderr: {}",
        extra_args,
        output.status,
        String::from_utf8_lossy(&output.stderr),
    );

    String::from_utf8(output.stdout)
        .expect("ftm-random output is not valid UTF-8")
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.to_owned())
        .collect()
}

// ── tests ─────────────────────────────────────────────────────────────────────

/// Parse a random number of random-schema entities and verify basic invariants:
/// every entity must parse without error, `id()` must be non-empty, and
/// `entity.schema()` must match the `"schema"` field in the original JSON.
#[test]
fn test_parse_random_entities() {
    let count = random_count(20, 100);
    let count_str = count.to_string();

    let lines = run_ftm_random(&["--random-schema", "--count", &count_str]);
    assert_eq!(
        lines.len(),
        count,
        "expected {count} lines from ftm-random, got {}",
        lines.len()
    );

    for (i, line) in lines.iter().enumerate() {
        let entity = FtmEntity::from_ftm_json(line).unwrap_or_else(|err| {
            panic!("parse failed on line {i}: {err}\n  json: {line}")
        });

        assert!(
            !entity.id().is_empty(),
            "entity.id() is empty on line {i}: {line}"
        );

        let json_schema = schema_from_json(line);
        assert_eq!(
            entity.schema(),
            json_schema,
            "schema mismatch on line {i}: entity.schema()={:?} but JSON had {:?}",
            entity.schema(),
            json_schema,
        );
    }
}

/// Verify that the `TryFrom<&str>` impl (which delegates to `from_ftm_json`)
/// produces the same `id` and `schema` as calling `from_ftm_json` directly.
#[test]
fn test_try_from_str_matches_from_ftm_json() {
    let count = random_count(10, 40);
    let count_str = count.to_string();

    let lines = run_ftm_random(&["--random-schema", "--count", &count_str]);

    for (i, line) in lines.iter().enumerate() {
        let via_method = FtmEntity::from_ftm_json(line)
            .unwrap_or_else(|e| panic!("from_ftm_json failed on line {i}: {e}\n{line}"));
        let via_try_from = FtmEntity::try_from(line.as_str())
            .unwrap_or_else(|e| panic!("TryFrom<&str> failed on line {i}: {e}\n{line}"));

        assert_eq!(
            via_method.schema(),
            via_try_from.schema(),
            "schema mismatch between from_ftm_json and TryFrom on line {i}"
        );
        assert_eq!(
            via_method.id(),
            via_try_from.id(),
            "id mismatch between from_ftm_json and TryFrom on line {i}"
        );
    }
}

/// Generate a small batch for every concrete FTM schema and verify that each
/// entity parses without error and is matched to the correct variant.
#[test]
fn test_all_schemas_parse() {
    let concrete_schemas = [
        "Address",
        "Airplane",
        "Article",
        "Asset",
        "Associate",
        "Audio",
        "BankAccount",
        "Call",
        "CallForTenders",
        "Company",
        "Contract",
        "ContractAward",
        "CourtCase",
        "CourtCaseParty",
        "CryptoWallet",
        "Debt",
        "Directorship",
        "Document",
        "EconomicActivity",
        "Email",
        "Employment",
        "Event",
        "Family",
        "Folder",
        "HyperText",
        "Identification",
        "Image",
        "LegalEntity",
        "License",
        "Membership",
        "Mention",
        "Message",
        "Note",
        "Occupancy",
        "Organization",
        "Ownership",
        "Package",
        "Page",
        "Pages",
        "Passport",
        "Payment",
        "Person",
        "PlainText",
        "Position",
        "Project",
        "ProjectParticipant",
        "PublicBody",
        "RealEstate",
        "Representation",
        "Risk",
        "Sanction",
        "Security",
        "Similar",
        "Succession",
        "Table",
        "TaxRoll",
        "Trip",
        "UnknownLink",
        "UserAccount",
        "Vehicle",
        "Vessel",
        "Video",
        "Workbook",
    ];

    for schema in concrete_schemas {
        let lines = run_ftm_random(&["--schema", schema, "--count", "3"]);
        assert_eq!(
            lines.len(),
            3,
            "expected 3 lines for schema {schema}, got {}",
            lines.len()
        );

        for (i, line) in lines.iter().enumerate() {
            let entity = FtmEntity::from_ftm_json(line).unwrap_or_else(|err| {
                panic!("parse failed for schema {schema} entity {i}: {err}\n{line}")
            });

            assert_eq!(
                entity.schema(),
                schema,
                "wrong variant for schema {schema} on entity {i}: got {:?}\n{line}",
                entity.schema()
            );
            assert!(
                !entity.id().is_empty(),
                "empty id for schema {schema} on entity {i}\n{line}"
            );
        }
    }
}

/// Parse a larger batch of random entities and collect all failures at once
/// rather than stopping at the first, giving a full picture of any breakage.
#[test]
fn test_bulk_random_entities_no_failures() {
    let count = random_count(50, 150);
    let count_str = count.to_string();

    let lines = run_ftm_random(&["--random-schema", "--count", &count_str]);

    let failures: Vec<String> = lines
        .iter()
        .enumerate()
        .filter_map(|(i, line)| match FtmEntity::from_ftm_json(line) {
            Ok(_) => None,
            Err(err) => Some(format!("line {i}: {err}\n  json: {line}")),
        })
        .collect();

    assert!(
        failures.is_empty(),
        "{} out of {} entities failed to parse:\n{}",
        failures.len(),
        count,
        failures.join("\n")
    );
}
