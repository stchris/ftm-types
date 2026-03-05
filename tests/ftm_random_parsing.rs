/// Integration tests that use `uvx ftm-random` to generate live FTM entities
/// and verify that `FtmEntity::from_ftm_json` can parse them correctly.
use ftm_types::FtmEntity;
use std::process::Command;
use std::sync::LazyLock;

// ── shared entity pools (generated once for the whole test run) ───────────────

/// 200 random-schema entities shared across tests that don't need specific schemas.
static RANDOM_ENTITIES: LazyLock<Vec<String>> =
    LazyLock::new(|| run_ftm_random(&["--random-schema", "--count", "200"]));

/// 3 entities per every concrete schema, shared by `test_all_schemas_parse`.
static ALL_SCHEMA_ENTITIES: LazyLock<Vec<String>> = LazyLock::new(|| {
    let mut args: Vec<&str> = Vec::new();
    for schema in CONCRETE_SCHEMAS {
        args.push("--schema");
        args.push(schema);
    }
    args.extend_from_slice(&["--count-per-schema", "3"]);
    run_ftm_random(&args)
});

// ── helpers ───────────────────────────────────────────────────────────────────

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

fn run_ftm_random(extra_args: &[&str]) -> Vec<String> {
    let mut cmd = Command::new("uvx");
    cmd.args(["ftm-random"]);
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

const CONCRETE_SCHEMAS: &[&str] = &[
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
    "Documentation",
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

// ── tests ─────────────────────────────────────────────────────────────────────

/// Parse a batch of random-schema entities and verify basic invariants:
/// every entity must parse without error, `id()` must be non-empty, and
/// `entity.schema()` must match the `"schema"` field in the original JSON.
#[test]
fn test_parse_random_entities() {
    let lines = &*RANDOM_ENTITIES;

    for (i, line) in lines.iter().enumerate() {
        let entity = FtmEntity::from_ftm_json(line)
            .unwrap_or_else(|err| panic!("parse failed on line {i}: {err}\n  json: {line}"));

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
    let lines = &*RANDOM_ENTITIES;

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
    let lines = &*ALL_SCHEMA_ENTITIES;
    let expected_total = CONCRETE_SCHEMAS.len() * 3;
    assert_eq!(
        lines.len(),
        expected_total,
        "expected {expected_total} lines, got {}",
        lines.len()
    );

    let mut failures: Vec<String> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let json_schema = schema_from_json(line);
        match FtmEntity::from_ftm_json(line) {
            Err(err) => failures.push(format!(
                "parse failed for schema {json_schema} entity {i}: {err}\n{line}"
            )),
            Ok(entity) => {
                if entity.schema() != json_schema {
                    failures.push(format!(
                        "wrong variant for schema {json_schema} on entity {i}: got {:?}\n{line}",
                        entity.schema()
                    ));
                }
                if entity.id().is_empty() {
                    failures.push(format!(
                        "empty id for schema {json_schema} on entity {i}\n{line}"
                    ));
                }
            }
        }
    }

    assert!(
        failures.is_empty(),
        "{} failure(s):\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// Parse a larger batch of random entities and collect all failures at once
/// rather than stopping at the first, giving a full picture of any breakage.
#[test]
fn test_bulk_random_entities_no_failures() {
    let lines = &*RANDOM_ENTITIES;

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
        lines.len(),
        failures.join("\n")
    );
}
