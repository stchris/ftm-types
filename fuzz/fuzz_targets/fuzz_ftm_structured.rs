//! Structured fuzz target for FtmEntity JSON parsing
//!
//! Uses the `arbitrary` crate to generate well-formed FTM JSON structures,
//! which provides better coverage than random byte fuzzing.
//!
//! Run with: cargo +nightly fuzz run fuzz_ftm_structured

#![no_main]

use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;
use ftm_types::FtmEntity;

/// Known FTM schema names for more targeted fuzzing
const SCHEMA_NAMES: &[&str] = &[
    "Person",
    "Company",
    "Organization",
    "LegalEntity",
    "Address",
    "Sanction",
    "Payment",
    "Ownership",
    "Directorship",
    "Employment",
    "Family",
    "Associate",
    "Membership",
    "Document",
    "Event",
    "Vehicle",
    "Vessel",
    "Airplane",
    "RealEstate",
    "BankAccount",
    "CryptoWallet",
];

/// Known FTM property names
const PROPERTY_NAMES: &[&str] = &[
    "name",
    "firstName",
    "lastName",
    "birthDate",
    "deathDate",
    "nationality",
    "country",
    "address",
    "email",
    "phone",
    "website",
    "description",
    "notes",
    "sourceUrl",
    "modifiedAt",
    "retrievedAt",
    "publisher",
    "amount",
    "currency",
    "date",
    "startDate",
    "endDate",
    "status",
    "summary",
    "role",
    "title",
];

#[derive(Debug)]
struct FtmJsonInput {
    id: String,
    schema: String,
    properties: Vec<(String, Vec<String>)>,
}

impl<'a> Arbitrary<'a> for FtmJsonInput {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        // Generate ID
        let id: String = if u.ratio(3, 4)? {
            // Usually use a reasonable ID
            let len = u.int_in_range(1..=64)?;
            (0..len)
                .map(|_| {
                    let chars = b"abcdefghijklmnopqrstuvwxyz0123456789-_";
                    let idx = u.choose_index(chars.len()).unwrap_or(0);
                    chars[idx] as char
                })
                .collect()
        } else {
            // Sometimes use arbitrary string
            u.arbitrary::<String>()?
        };

        // Generate schema name
        let schema: String = if u.ratio(4, 5)? {
            // Usually pick a known schema
            SCHEMA_NAMES[u.choose_index(SCHEMA_NAMES.len())?].to_string()
        } else {
            // Sometimes use arbitrary string
            u.arbitrary::<String>()?
        };

        // Generate properties
        let num_props = u.int_in_range(0..=10)?;
        let mut properties = Vec::with_capacity(num_props);

        for _ in 0..num_props {
            let key: String = if u.ratio(3, 4)? {
                PROPERTY_NAMES[u.choose_index(PROPERTY_NAMES.len())?].to_string()
            } else {
                u.arbitrary::<String>()?
            };

            let num_values = u.int_in_range(0..=5)?;
            let values: Vec<String> = (0..num_values)
                .map(|_| u.arbitrary::<String>())
                .collect::<Result<Vec<_>, _>>()?;

            properties.push((key, values));
        }

        Ok(FtmJsonInput {
            id,
            schema,
            properties,
        })
    }
}

impl FtmJsonInput {
    fn to_json(&self) -> String {
        let escape_json = |s: &str| -> String {
            s.chars()
                .map(|c| match c {
                    '"' => "\\\"".to_string(),
                    '\\' => "\\\\".to_string(),
                    '\n' => "\\n".to_string(),
                    '\r' => "\\r".to_string(),
                    '\t' => "\\t".to_string(),
                    c if c.is_control() => format!("\\u{:04x}", c as u32),
                    c => c.to_string(),
                })
                .collect()
        };

        let props: Vec<String> = self
            .properties
            .iter()
            .map(|(k, v)| {
                let values: Vec<String> = v.iter().map(|s| format!("\"{}\"", escape_json(s))).collect();
                format!("\"{}\":[{}]", escape_json(k), values.join(","))
            })
            .collect();

        format!(
            r#"{{"id":"{}","schema":"{}","properties":{{{}}}}}"#,
            escape_json(&self.id),
            escape_json(&self.schema),
            props.join(",")
        )
    }
}

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = FtmJsonInput::arbitrary(&mut Unstructured::new(data)) {
        let json = input.to_json();
        let _ = FtmEntity::from_ftm_json(&json);
    }
});
