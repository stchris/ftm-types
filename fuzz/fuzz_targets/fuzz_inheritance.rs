//! Fuzz target for schema inheritance resolution
//!
//! Tests the inheritance resolution logic with various schema graph configurations,
//! including potential edge cases like deep hierarchies and complex inheritance patterns.
//!
//! Run with: cargo +nightly fuzz run fuzz_inheritance

#![no_main]

use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;
use std::collections::HashSet;
use std::io::Write;
use tempfile::TempDir;
use ftm_types::SchemaRegistry;

/// A schema definition for fuzzing
#[derive(Debug)]
struct FuzzSchema {
    name: String,
    extends: Option<usize>, // Index into schemas array (must be < current index)
    properties: Vec<FuzzProperty>,
    is_abstract: bool,
}

#[derive(Debug)]
struct FuzzProperty {
    name: String,
    prop_type: String,
    label: Option<String>,
}

/// A graph of schemas for fuzzing inheritance
#[derive(Debug)]
struct SchemaGraph {
    schemas: Vec<FuzzSchema>,
}

const VALID_TYPES: &[&str] = &["string", "name", "identifier", "date", "url", "country", "language", "number"];

const VALID_NAMES: &[&str] = &[
    "Thing", "LegalEntity", "Person", "Organization", "Company", "PublicBody",
    "Asset", "Vehicle", "Vessel", "Airplane", "RealEstate", "Address",
    "Interval", "Event", "Sanction", "Document", "Payment", "Ownership",
];

impl<'a> Arbitrary<'a> for FuzzProperty {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let name = if u.ratio(3, 4)? {
            // Use common property names
            let names = &["name", "date", "country", "description", "notes", "amount", "status"];
            names[u.choose_index(names.len())?].to_string()
        } else {
            // Generate arbitrary name (alphanumeric only for valid YAML)
            let len = u.int_in_range(1..=20)?;
            (0..len)
                .map(|_| {
                    let chars = b"abcdefghijklmnopqrstuvwxyz";
                    let idx = u.choose_index(chars.len()).unwrap_or(0);
                    chars[idx] as char
                })
                .collect()
        };

        let prop_type = VALID_TYPES[u.choose_index(VALID_TYPES.len())?].to_string();

        let label = if u.ratio(1, 3)? {
            Some(format!("{} Label", name))
        } else {
            None
        };

        Ok(FuzzProperty { name, prop_type, label })
    }
}

impl<'a> Arbitrary<'a> for SchemaGraph {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        // Limit schema count to avoid timeout
        let num_schemas = u.int_in_range(1..=15)?;
        let mut schemas = Vec::with_capacity(num_schemas);
        let mut used_names = HashSet::new();

        for i in 0..num_schemas {
            // Generate unique name
            let base_name = if u.ratio(2, 3)? && i < VALID_NAMES.len() {
                VALID_NAMES[i].to_string()
            } else {
                let len = u.int_in_range(3..=15)?;
                (0..len)
                    .map(|_| {
                        let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
                        let idx = u.choose_index(chars.len()).unwrap_or(0);
                        chars[idx] as char
                    })
                    .collect()
            };

            // Ensure unique name
            let name = if used_names.contains(&base_name) {
                format!("{}{}", base_name, i)
            } else {
                base_name
            };
            used_names.insert(name.clone());

            // Maybe extend a previous schema (creating inheritance)
            let extends = if i > 0 && u.ratio(2, 3)? {
                Some(u.int_in_range(0..=i - 1)?)
            } else {
                None
            };

            // Generate properties
            let num_props = u.int_in_range(0..=5)?;
            let properties: Vec<FuzzProperty> = (0..num_props)
                .map(|_| FuzzProperty::arbitrary(u))
                .collect::<Result<Vec<_>, _>>()?;

            // Abstract schemas are more likely for base schemas
            let is_abstract = extends.is_none() && u.ratio(1, 2)?;

            schemas.push(FuzzSchema {
                name,
                extends,
                properties,
                is_abstract,
            });
        }

        Ok(SchemaGraph { schemas })
    }
}

impl SchemaGraph {
    fn to_yaml_files(&self, dir: &std::path::Path) -> std::io::Result<()> {
        for (i, schema) in self.schemas.iter().enumerate() {
            let yaml = self.schema_to_yaml(schema, i);
            let path = dir.join(format!("{}.yaml", schema.name));
            let mut file = std::fs::File::create(&path)?;
            file.write_all(yaml.as_bytes())?;
        }
        Ok(())
    }

    fn schema_to_yaml(&self, schema: &FuzzSchema, _index: usize) -> String {
        let mut yaml = format!("{}:\n", schema.name);
        yaml.push_str(&format!("  label: \"{}\"\n", schema.name));

        if schema.is_abstract {
            yaml.push_str("  abstract: true\n");
        }

        if let Some(parent_idx) = schema.extends {
            if parent_idx < self.schemas.len() {
                yaml.push_str("  extends:\n");
                yaml.push_str(&format!("    - {}\n", self.schemas[parent_idx].name));
            }
        }

        if !schema.properties.is_empty() {
            yaml.push_str("  properties:\n");
            for prop in &schema.properties {
                yaml.push_str(&format!("    {}:\n", prop.name));
                yaml.push_str(&format!("      type: {}\n", prop.prop_type));
                if let Some(ref label) = prop.label {
                    yaml.push_str(&format!("      label: \"{}\"\n", label));
                }
            }
        }

        yaml
    }
}

fuzz_target!(|data: &[u8]| {
    // Parse the fuzz input into a schema graph
    let graph = match SchemaGraph::arbitrary(&mut Unstructured::new(data)) {
        Ok(g) => g,
        Err(_) => return,
    };

    // Skip empty graphs
    if graph.schemas.is_empty() {
        return;
    }

    // Create temporary directory
    let temp_dir = match TempDir::new() {
        Ok(dir) => dir,
        Err(_) => return,
    };

    // Write schema YAML files
    if graph.to_yaml_files(temp_dir.path()).is_err() {
        return;
    }

    // Load and resolve schemas
    if let Ok(registry) = SchemaRegistry::load_from_cache(temp_dir.path()) {
        // Try to resolve inheritance for each schema
        for name in registry.schema_names() {
            let _ = registry.resolve_inheritance(&name);
        }
    }
});
