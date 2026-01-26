use ftm_types::{CodeGenerator, SchemaRegistry};
use std::env;
use std::path::{Path, PathBuf};

use anyhow::Context;

fn validate_schema_version(version: &str) -> anyhow::Result<PathBuf> {
    let schema_path = PathBuf::from("schemas").join(version);

    if !schema_path.exists() {
        anyhow::bail!(
            "Schema version '{}' not found. Available versions: {}",
            version,
            get_available_versions().join(", ")
        );
    }

    if !schema_path.is_dir() {
        anyhow::bail!("Schema path '{}' is not a directory", schema_path.display());
    }

    Ok(schema_path)
}

fn get_available_versions() -> Vec<String> {
    let schemas_dir = Path::new("schemas");

    if !schemas_dir.exists() {
        return vec![];
    }

    std::fs::read_dir(schemas_dir)
        .ok()
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .filter_map(|e| e.file_name().into_string().ok())
                .collect()
        })
        .unwrap_or_default()
}

fn print_usage() {
    eprintln!("Usage: ftm-types [SCHEMA_VERSION]");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!(
        "  SCHEMA_VERSION  Schema version to use (required, one of {})",
        get_available_versions().join(", ")
    );
    eprintln!("Hint: run ./download.sh <version> to download schema files.")
}

pub fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Check for help flag
    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        print_usage();
        return Ok(());
    }

    // Parse arguments
    let schema_version = args
        .get(1)
        .map(|s| s.as_str())
        .context("schema version is required")?;

    // Validate and get schema path
    let schema_path = validate_schema_version(schema_version)?;

    println!("Using schema version: {}", schema_version);
    println!("Loading schemas from: {}", schema_path.display());

    // Load schema registry
    let registry = SchemaRegistry::load_from_cache(schema_path.to_str().unwrap())?;

    // Generate Rust code
    let output_dir = "src/generated";
    println!("Generating code to: {}", output_dir);
    let codegen = CodeGenerator::new(registry, output_dir);
    codegen.generate_all()?;

    println!("Code generation completed successfully!");

    Ok(())
}
