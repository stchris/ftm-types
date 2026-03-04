/// Integration tests that use `uvx ftm-random` to generate live FTM entities
/// and verify that `FtmEntity::from_ftm_json` can parse them correctly.
///
/// `ftm-random --random-schema` may also emit abstract schemas (e.g. `Thing`,
/// `Interval`, `Analyzable`) that have no concrete `FtmEntity` variant.
/// Those produce an "unknown FTM schema" error from `from_ftm_json`, which is
/// the expected and correct behaviour — the tests below distinguish between
/// these expected misses and genuine parse failures.
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

/// Returns `true` when `schema` is one of the concrete FTM schemas that
/// `FtmEntity` can represent.  Abstract schemas (`Thing`, `Interval`, …) that
/// `ftm-random --random-schema` may emit are **not** in this list.
fn is_concrete_schema(schema: &str) -> bool {
    matches!(
        schema,
        "Address"
            | "Airplane"
            | "Article"
            | "Asset"
            | "Associate"
            | "Audio"
            | "BankAccount"
            | "Call"
            | "CallForTenders"
            | "Company"
            | "Contract"
            | "ContractAward"
            | "CourtCase"
            | "CourtCaseParty"
            | "CryptoWallet"
            | "Debt"
            | "Directorship"
            | "Document"
            | "EconomicActivity"
            | "Email"
            | "Employment"
            | "Event"
            | "Family"
            | "Folder"
            | "HyperText"
            | "Identification"
            | "Image"
            | "LegalEntity"
            | "License"
            | "Membership"
            | "Mention"
            | "Message"
            | "Note"
            | "Occupancy"
            | "Organization"
            | "Ownership"
            | "Package"
            | "Page"
            | "Pages"
            | "Passport"
            | "Payment"
            | "Person"
            | "PlainText"
            | "Position"
            | "Project"
            | "ProjectParticipant"
            | "PublicBody"
            | "RealEstate"
            | "Representation"
            | "Risk"
            | "Sanction"
            | "Security"
            | "Similar"
            | "Succession"
            | "Table"
            | "TaxRoll"
            | "Trip"
            | "UnknownLink"
            | "UserAccount"
            | "Vehicle"
            | "Vessel"
            | "Video"
            | "Workbook"
    )
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

/// Run `uvx ftm-random` with the given extra arguments and return the stdout
/// lines (one JSON entity per line).  Panics with a descriptive message if
/// the tool is unavailable or exits non-zero.
fn run_ftm_random(extra_args: &[&str]) -> Vec<String> {
    let mut cmd = Command::new("uvx");
    cmd.arg("ftm-random");
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

/// Parse a random number of random-schema entities.
///
/// * Every entity whose JSON `"schema"` field is a *concrete* FTM type must
///   parse without error and the resulting `FtmEntity::schema()` must match.
/// * Entities with abstract schemas (e.g. `Thing`, `Interval`) correctly
///   return an "unknown FTM schema" error; the test records those but does
///   not fail because of them.
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

    let mut concrete_parsed = 0usize;
    let mut abstract_skipped = 0usize;

    for (i, line) in lines.iter().enumerate() {
        let json_schema = schema_from_json(line);

        if !is_concrete_schema(json_schema) {
            // Abstract schemas are not representable by FtmEntity.
            // from_ftm_json returns an "unknown FTM schema" error here,
            // which is correct behaviour.
            abstract_skipped += 1;
            continue;
        }

        let entity = FtmEntity::from_ftm_json(line).unwrap_or_else(|err| {
            panic!(
                "parse failed for concrete schema {json_schema:?} on line {i}:\n  \
                 error: {err}\n  json: {line}"
            )
        });

        assert!(
            !entity.id().is_empty(),
            "entity.id() is empty on line {i}: {line}"
        );
        assert_eq!(
            entity.schema(),
            json_schema,
            "schema mismatch on line {i}: entity.schema()={:?} but JSON had {:?}",
            entity.schema(),
            json_schema,
        );

        concrete_parsed += 1;
    }

    println!(
        "parsed {concrete_parsed} concrete entities, skipped {abstract_skipped} abstract schemas \
         (out of {count} total)"
    );
}

/// Verify that the `TryFrom<&str>` impl (which delegates to `from_ftm_json`)
/// produces results consistent with calling `from_ftm_json` directly.
#[test]
fn test_try_from_str_matches_from_ftm_json() {
    let count = random_count(10, 40);
    let count_str = count.to_string();

    let lines = run_ftm_random(&["--random-schema", "--count", &count_str]);

    for (i, line) in lines.iter().enumerate() {
        let json_schema = schema_from_json(line);

        if !is_concrete_schema(json_schema) {
            // Both paths will return the same "unknown FTM schema" error;
            // no mismatch is possible, so skip.
            continue;
        }

        let via_method = FtmEntity::from_ftm_json(line).unwrap_or_else(|e| {
            panic!("from_ftm_json failed on line {i} (schema={json_schema:?}): {e}\n{line}")
        });
        let via_try_from = FtmEntity::try_from(line.as_str()).unwrap_or_else(|e| {
            panic!("TryFrom<&str> failed on line {i} (schema={json_schema:?}): {e}\n{line}")
        });

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
/// entity parses without error and is matched to the right variant.
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

/// Parse a larger batch of random entities and collect *unexpected* failures
/// (i.e. errors for concrete schemas), reporting them all at once rather than
/// stopping at the first one.  Abstract-schema entities are tolerated.
#[test]
fn test_bulk_random_entities_no_failures() {
    let count = random_count(50, 150);
    let count_str = count.to_string();

    let lines = run_ftm_random(&["--random-schema", "--count", &count_str]);

    let mut unexpected_failures: Vec<String> = Vec::new();
    let mut abstract_count = 0usize;
    let mut ok_count = 0usize;

    for (i, line) in lines.iter().enumerate() {
        let json_schema = schema_from_json(line);

        if !is_concrete_schema(json_schema) {
            abstract_count += 1;
            continue;
        }

        match FtmEntity::from_ftm_json(line) {
            Ok(_) => ok_count += 1,
            Err(err) => unexpected_failures.push(format!(
                "line {i} (schema={json_schema:?}): {err}\n  json: {line}"
            )),
        }
    }

    println!(
        "bulk test: {ok_count} parsed ok, {abstract_count} abstract schemas skipped, \
         {} unexpected failures (out of {count} total)",
        unexpected_failures.len()
    );

    assert!(
        unexpected_failures.is_empty(),
        "{} concrete-schema entities failed to parse:\n{}",
        unexpected_failures.len(),
        unexpected_failures.join("\n")
    );
}
