//! FTM schema parser
//!
//! Parses YAML schemas from FollowTheMoney and resolves inheritance chains.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

/// FTM schema definition from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FtmSchema {
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub plural: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub extends: Option<Vec<String>>,
    #[serde(rename = "abstract", default)]
    pub abstract_: Option<bool>,
    #[serde(default)]
    pub matchable: Option<bool>,
    #[serde(default)]
    pub featured: Option<Vec<String>>,
    #[serde(default)]
    pub required: Option<Vec<String>>,
    #[serde(default)]
    pub caption: Option<Vec<String>>,
    #[serde(default)]
    pub properties: HashMap<String, FtmProperty>,
    // Ignore other fields we don't need
    #[serde(flatten)]
    pub _extra: HashMap<String, serde_json::Value>,
}

/// Reverse property definition for bidirectional relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseProperty {
    pub name: String,
    pub label: String,
}

/// FTM property definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FtmProperty {
    pub label: Option<String>,
    #[serde(rename = "type", default)]
    pub type_: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub reverse: Option<ReverseProperty>,
    #[serde(default)]
    pub range: Option<String>,
    #[serde(default)]
    pub stub: Option<bool>,
    #[serde(default)]
    pub deprecated: Option<bool>,
    // Ignore other fields we don't need
    #[serde(flatten)]
    pub _extra: HashMap<String, serde_json::Value>,
}

/// Schema registry that manages all loaded schemas
pub struct SchemaRegistry {
    schemas: HashMap<String, FtmSchema>,
}

impl SchemaRegistry {
    /// Load all schemas from a cache directory
    pub fn load_from_cache<P: AsRef<Path>>(cache_dir: P) -> Result<Self> {
        let cache_path = cache_dir.as_ref();

        if !cache_path.exists() {
            anyhow::bail!("Cache directory does not exist: {:?}", cache_path);
        }

        let mut schemas = HashMap::new();

        // Read all YAML files (both .yml and .yaml extensions)
        for entry in fs::read_dir(cache_path)? {
            let entry = entry?;
            let path = entry.path();

            let ext = path.extension().and_then(|s| s.to_str());
            if ext == Some("yml") || ext == Some("yaml") {
                let schema_name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .context("Invalid schema filename")?
                    .to_string();

                let content = fs::read_to_string(&path).context(format!(
                    "Failed to read schema file: {} at {:?}",
                    schema_name, path
                ))?;

                // Try parsing as wrapped format first (FTM official format)
                // where schema name is root key: "Person: { label: Person, ... }"
                let schema = if let Ok(root) =
                    serde_yaml::from_str::<HashMap<String, FtmSchema>>(&content)
                {
                    // Extract the schema (should only have one key)
                    root.into_iter()
                        .next()
                        .map(|(_, schema)| schema)
                        .context(format!("Empty schema file: {} at {:?}", schema_name, path))?
                } else {
                    // Fall back to direct format (used in tests)
                    // where the file directly contains: "{ label: Thing, ... }"
                    serde_yaml::from_str::<FtmSchema>(&content).context(format!(
                        "Failed to parse schema: {} at {:?}\nFirst 500 chars of content:\n{}",
                        schema_name,
                        path,
                        &content.chars().take(500).collect::<String>()
                    ))?
                };

                schemas.insert(schema_name.clone(), schema);
            }
        }

        if schemas.is_empty() {
            anyhow::bail!("No schemas found in cache directory");
        }

        println!("Loaded {} schemas", schemas.len());
        Ok(Self { schemas })
    }

    /// Get a schema by name
    pub fn get(&self, name: &str) -> Option<&FtmSchema> {
        self.schemas.get(name)
    }

    /// Get all schema names
    pub fn schema_names(&self) -> Vec<String> {
        let mut names: Vec<_> = self.schemas.keys().cloned().collect();
        names.sort();
        names
    }

    /// Count of schemas
    pub fn count(&self) -> usize {
        self.schemas.len()
    }

    /// Resolve a schema with all inherited properties
    pub fn resolve_inheritance(&self, schema_name: &str) -> Result<ResolvedSchema> {
        let mut visited = HashSet::new();
        let all_properties = self.resolve_properties_recursive(schema_name, &mut visited)?;

        let mut visited_required = HashSet::new();
        let all_required = self.resolve_required_recursive(schema_name, &mut visited_required)?;

        let metadata = self
            .get(schema_name)
            .context(format!("Schema not found: {}", schema_name))?
            .clone();

        Ok(ResolvedSchema {
            name: schema_name.to_string(),
            all_properties,
            all_required,
            metadata,
        })
    }

    /// Recursively resolve properties from parent schemas
    fn resolve_properties_recursive(
        &self,
        schema_name: &str,
        visited: &mut HashSet<String>,
    ) -> Result<HashMap<String, FtmProperty>> {
        // Check for circular inheritance (schema directly referencing itself in the current path)
        if visited.contains(schema_name) {
            anyhow::bail!("Circular inheritance detected: {}", schema_name);
        }
        visited.insert(schema_name.to_string());

        let schema = self
            .get(schema_name)
            .context(format!("Schema not found: {}", schema_name))?;

        let mut properties = HashMap::new();

        // First, collect properties from parent schemas
        if let Some(extends) = &schema.extends {
            for parent_name in extends {
                let parent_props = self.resolve_properties_recursive(parent_name, visited)?;
                properties.extend(parent_props);
            }
        }

        // Then, add/override with this schema's properties
        properties.extend(schema.properties.clone());

        // Remove from visited set to allow diamond inheritance patterns
        visited.remove(schema_name);

        Ok(properties)
    }

    /// Recursively resolve required fields from parent schemas
    fn resolve_required_recursive(
        &self,
        schema_name: &str,
        visited: &mut HashSet<String>,
    ) -> Result<HashSet<String>> {
        // Check for circular inheritance
        if visited.contains(schema_name) {
            anyhow::bail!("Circular inheritance detected: {}", schema_name);
        }
        visited.insert(schema_name.to_string());

        let schema = self
            .get(schema_name)
            .context(format!("Schema not found: {}", schema_name))?;

        let mut required = HashSet::new();

        // First, collect required fields from parent schemas
        if let Some(extends) = &schema.extends {
            for parent_name in extends {
                let parent_required = self.resolve_required_recursive(parent_name, visited)?;
                required.extend(parent_required);
            }
        }

        // Then, add this schema's required fields
        if let Some(schema_required) = &schema.required {
            required.extend(schema_required.iter().cloned());
        }

        // Remove from visited set to allow diamond inheritance patterns
        visited.remove(schema_name);

        Ok(required)
    }

    /// Get all concrete (non-abstract) schemas
    pub fn concrete_schemas(&self) -> Vec<String> {
        self.schemas
            .iter()
            .filter(|(_, schema)| !schema.abstract_.unwrap_or(false))
            .map(|(name, _)| name.clone())
            .collect()
    }
}

/// A resolved schema with all properties flattened
#[derive(Debug, Clone)]
pub struct ResolvedSchema {
    pub name: String,
    pub all_properties: HashMap<String, FtmProperty>,
    pub all_required: HashSet<String>,
    pub metadata: FtmSchema,
}

impl ResolvedSchema {
    /// Check if this schema is abstract
    pub fn is_abstract(&self) -> bool {
        self.metadata.abstract_.unwrap_or(false)
    }

    /// Get the label for this schema
    pub fn label(&self) -> Option<&str> {
        self.metadata.label.as_deref()
    }

    /// Get the description if available
    pub fn description(&self) -> Option<&str> {
        self.metadata.description.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use crate::FtmEntity;

    use super::*;
    use std::{
        fs::File,
        io::{BufRead, BufReader, Write},
    };
    use tempfile::TempDir;

    fn create_test_schema(dir: &Path, name: &str, yaml: &str) {
        let path = dir.join(format!("{}.yml", name));
        let mut file = fs::File::create(path).unwrap();
        file.write_all(yaml.as_bytes()).unwrap();
    }

    #[test]
    fn test_schema_loading() {
        let temp_dir = TempDir::new().unwrap();

        create_test_schema(
            temp_dir.path(),
            "Thing",
            r#"
label: Thing
abstract: true
properties:
  name:
    label: Name
    type: name
"#,
        );

        let registry = SchemaRegistry::load_from_cache(temp_dir.path()).unwrap();
        assert_eq!(registry.count(), 1);
        assert!(registry.get("Thing").is_some());
    }

    #[test]
    fn test_inheritance_resolution() {
        let temp_dir = TempDir::new().unwrap();

        create_test_schema(
            temp_dir.path(),
            "Thing",
            r#"
label: Thing
abstract: true
properties:
  name:
    label: Name
    type: name
"#,
        );

        create_test_schema(
            temp_dir.path(),
            "Person",
            r#"
label: Person
extends:
  - Thing
properties:
  firstName:
    label: First Name
    type: name
"#,
        );

        let registry = SchemaRegistry::load_from_cache(temp_dir.path()).unwrap();
        let person = registry.resolve_inheritance("Person").unwrap();

        assert_eq!(person.all_properties.len(), 2);
        assert!(person.all_properties.contains_key("name"));
        assert!(person.all_properties.contains_key("firstName"));
    }

    #[test]
    fn test_circular_inheritance() {
        let temp_dir = TempDir::new().unwrap();

        create_test_schema(
            temp_dir.path(),
            "A",
            r#"
label: A
extends:
  - B
properties: {}
"#,
        );

        create_test_schema(
            temp_dir.path(),
            "B",
            r#"
label: B
extends:
  - A
properties: {}
"#,
        );

        let registry = SchemaRegistry::load_from_cache(temp_dir.path()).unwrap();
        let result = registry.resolve_inheritance("A");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Circular"));
    }

    /// Test with real FTM schemas from OpenSanctions
    /// These schemas are based on the actual FollowTheMoney schema structure
    #[test]
    fn test_opensanctions_person_schema() {
        let temp_dir = TempDir::new().unwrap();

        // Thing - the base schema
        create_test_schema(
            temp_dir.path(),
            "Thing",
            r#"
label: Thing
abstract: true
description: A basic object in a domain
properties:
  id:
    label: ID
    type: identifier
    description: Unique identifier for the entity
  schema:
    label: Schema
    type: string
    description: The schema type for this entity
  properties:
    label: Properties
    type: json
    description: All properties as a JSON object
"#,
        );

        // LegalEntity - intermediate abstract schema
        create_test_schema(
            temp_dir.path(),
            "LegalEntity",
            r#"
label: Legal entity
abstract: true
extends:
  - Thing
description: A legal entity that can enter into legal relationships
properties:
  name:
    label: Name
    type: name
    description: Primary name of the entity
  alias:
    label: Alias
    type: name
    description: Alternative names
  country:
    label: Country
    type: country
    description: Associated countries
  jurisdiction:
    label: Jurisdiction
    type: country
    description: Legal jurisdiction
  notes:
    label: Notes
    type: text
    description: Additional notes or comments
  sourceUrl:
    label: Source URL
    type: url
    description: URL of the source data
  publisher:
    label: Publisher
    type: string
    description: Data publisher name
  publisherUrl:
    label: Publisher URL
    type: url
    description: URL of the data publisher
"#,
        );

        // Person - concrete schema
        create_test_schema(
            temp_dir.path(),
            "Person",
            r#"
label: Person
extends:
  - LegalEntity
description: An individual human being
properties:
  firstName:
    label: First name
    type: name
    description: Given name
  middleName:
    label: Middle name
    type: name
    description: Middle name
  lastName:
    label: Last name
    type: name
    description: Family name
  fatherName:
    label: Father name
    type: name
    description: Patronymic name
  motherName:
    label: Mother name
    type: name
    description: Matronymic name
  birthDate:
    label: Date of birth
    type: date
    description: Date of birth
  birthPlace:
    label: Place of birth
    type: string
    description: Place of birth
  deathDate:
    label: Date of death
    type: date
    description: Date of death
  nationality:
    label: Nationality
    type: country
    description: Citizenship
  idNumber:
    label: ID number
    type: identifier
    description: National identity number
  passportNumber:
    label: Passport number
    type: identifier
    description: Passport number
  email:
    label: Email
    type: email
    description: Email address
  phone:
    label: Phone number
    type: phone
    description: Phone number
  address:
    label: Address
    type: address
    description: Physical address
  position:
    label: Position
    type: string
    description: Professional position
  political:
    label: Politically exposed
    type: string
    description: PEP designation
"#,
        );

        let registry = SchemaRegistry::load_from_cache(temp_dir.path()).unwrap();

        // Verify schemas loaded
        assert_eq!(registry.count(), 3);
        assert!(registry.get("Thing").is_some());
        assert!(registry.get("LegalEntity").is_some());
        assert!(registry.get("Person").is_some());

        // Test Person inherits from LegalEntity and Thing
        let person = registry.resolve_inheritance("Person").unwrap();

        // Should have properties from all three schemas
        assert!(person.all_properties.contains_key("id")); // from Thing
        assert!(person.all_properties.contains_key("name")); // from LegalEntity
        assert!(person.all_properties.contains_key("firstName")); // from Person
        assert!(person.all_properties.contains_key("birthDate")); // from Person
        assert!(person.all_properties.contains_key("nationality")); // from Person

        // Verify abstract schemas are marked correctly
        assert!(registry.get("Thing").unwrap().abstract_.unwrap_or(false));
        assert!(
            registry
                .get("LegalEntity")
                .unwrap()
                .abstract_
                .unwrap_or(false)
        );
        assert!(!registry.get("Person").unwrap().abstract_.unwrap_or(false));
    }

    #[test]
    fn test_opensanctions_organization_schema() {
        let temp_dir = TempDir::new().unwrap();

        // Reuse Thing and LegalEntity from above
        create_test_schema(
            temp_dir.path(),
            "Thing",
            r#"
label: Thing
abstract: true
properties:
  id:
    label: ID
    type: identifier
"#,
        );

        create_test_schema(
            temp_dir.path(),
            "LegalEntity",
            r#"
label: Legal entity
abstract: true
extends:
  - Thing
properties:
  name:
    label: Name
    type: name
  country:
    label: Country
    type: country
"#,
        );

        // Organization schema
        create_test_schema(
            temp_dir.path(),
            "Organization",
            r#"
label: Organization
extends:
  - LegalEntity
description: A legal entity representing a group or institution
properties:
  incorporationDate:
    label: Incorporation date
    type: date
    description: Date of incorporation
  dissolutionDate:
    label: Dissolution date
    type: date
    description: Date of dissolution
  taxNumber:
    label: Tax number
    type: identifier
    description: Tax identification number
  registrationNumber:
    label: Registration number
    type: identifier
    description: Company registration number
  legalForm:
    label: Legal form
    type: string
    description: Type of legal entity (LLC, Corp, etc)
  sector:
    label: Sector
    type: string
    description: Industry sector
  classification:
    label: Classification
    type: string
    description: Business classification
  address:
    label: Address
    type: address
    description: Registered address
  email:
    label: Email
    type: email
    description: Contact email
  phone:
    label: Phone
    type: phone
    description: Contact phone
  website:
    label: Website
    type: url
    description: Company website
"#,
        );

        let registry = SchemaRegistry::load_from_cache(temp_dir.path()).unwrap();
        let org = registry.resolve_inheritance("Organization").unwrap();

        // Check inherited and direct properties
        assert!(org.all_properties.contains_key("id"));
        assert!(org.all_properties.contains_key("name"));
        assert!(org.all_properties.contains_key("country"));
        assert!(org.all_properties.contains_key("incorporationDate"));
        assert!(org.all_properties.contains_key("taxNumber"));
        assert!(org.all_properties.contains_key("website"));
    }

    #[test]
    fn test_opensanctions_sanction_schema() {
        let temp_dir = TempDir::new().unwrap();

        // Base schemas
        create_test_schema(
            temp_dir.path(),
            "Thing",
            r#"
label: Thing
abstract: true
properties:
  id:
    label: ID
    type: identifier
"#,
        );

        create_test_schema(
            temp_dir.path(),
            "Interval",
            r#"
label: Interval
abstract: true
extends:
  - Thing
description: An entity with a start and end date
properties:
  startDate:
    label: Start date
    type: date
    description: Start date
  endDate:
    label: End date
    type: date
    description: End date
"#,
        );

        // Sanction schema - common in OpenSanctions data
        create_test_schema(
            temp_dir.path(),
            "Sanction",
            r#"
label: Sanction
extends:
  - Interval
description: A legal sanction against a person or entity
properties:
  entity:
    label: Entity
    type: entity
    description: Sanctioned entity
  authority:
    label: Authority
    type: string
    description: Sanctioning authority
  sourceUrl:
    label: Source URL
    type: url
    description: URL of sanction listing
  program:
    label: Program
    type: string
    description: Sanctions program name
  reason:
    label: Reason
    type: text
    description: Reason for sanction
  provisions:
    label: Provisions
    type: string
    description: Sanction provisions
  country:
    label: Country
    type: country
    description: Country imposing sanction
  unscId:
    label: UNSC ID
    type: identifier
    description: UN Security Council identifier
  listingDate:
    label: Listing date
    type: date
    description: Date added to sanctions list
  duration:
    label: Duration
    type: string
    description: Sanction duration
"#,
        );

        let registry = SchemaRegistry::load_from_cache(temp_dir.path()).unwrap();
        let sanction = registry.resolve_inheritance("Sanction").unwrap();

        // Verify complete property inheritance chain
        assert!(sanction.all_properties.contains_key("id")); // from Thing
        assert!(sanction.all_properties.contains_key("startDate")); // from Interval
        assert!(sanction.all_properties.contains_key("endDate")); // from Interval
        assert!(sanction.all_properties.contains_key("entity")); // from Sanction
        assert!(sanction.all_properties.contains_key("authority")); // from Sanction
        assert!(sanction.all_properties.contains_key("program")); // from Sanction
        assert!(sanction.all_properties.contains_key("unscId")); // from Sanction

        // Total properties should include all from chain
        assert!(sanction.all_properties.len() >= 12);
    }

    #[test]
    fn test_multiple_inheritance_paths() {
        let temp_dir = TempDir::new().unwrap();

        // Diamond inheritance pattern (common in FTM)
        create_test_schema(
            temp_dir.path(),
            "Thing",
            r#"
label: Thing
abstract: true
properties:
  id:
    label: ID
    type: identifier
  name:
    label: Name
    type: name
"#,
        );

        create_test_schema(
            temp_dir.path(),
            "LegalEntity",
            r#"
label: Legal entity
abstract: true
extends:
  - Thing
properties:
  country:
    label: Country
    type: country
"#,
        );

        create_test_schema(
            temp_dir.path(),
            "DirectorshipEntity",
            r#"
label: Directorship entity
abstract: true
extends:
  - Thing
properties:
  role:
    label: Role
    type: string
"#,
        );

        // Company extends both LegalEntity and DirectorshipEntity (diamond pattern)
        create_test_schema(
            temp_dir.path(),
            "Company",
            r#"
label: Company
extends:
  - LegalEntity
properties:
  registrationNumber:
    label: Registration number
    type: identifier
"#,
        );

        let registry = SchemaRegistry::load_from_cache(temp_dir.path()).unwrap();
        let company = registry.resolve_inheritance("Company").unwrap();

        // Should have properties from all paths (no duplicates from Thing)
        assert!(company.all_properties.contains_key("id"));
        assert!(company.all_properties.contains_key("name"));
        assert!(company.all_properties.contains_key("country"));
        assert!(company.all_properties.contains_key("registrationNumber"));
    }

    #[test]
    fn test_property_overriding() {
        let temp_dir = TempDir::new().unwrap();

        create_test_schema(
            temp_dir.path(),
            "Base",
            r#"
label: Base
abstract: true
properties:
  value:
    label: Original Value
    type: string
    description: Original description
"#,
        );

        create_test_schema(
            temp_dir.path(),
            "Derived",
            r#"
label: Derived
extends:
  - Base
properties:
  value:
    label: Overridden Value
    type: number
    description: New description
  extra:
    label: Extra
    type: string
"#,
        );

        let registry = SchemaRegistry::load_from_cache(temp_dir.path()).unwrap();
        let derived = registry.resolve_inheritance("Derived").unwrap();

        // Child schema properties should override parent
        assert_eq!(derived.all_properties.len(), 2);
        let value_prop = &derived.all_properties["value"];
        assert_eq!(value_prop.label, Some("Overridden Value".to_string()));
        assert_eq!(value_prop.type_, Some("number".to_string()));
        assert_eq!(value_prop.description, Some("New description".to_string()));
    }

    #[test]
    fn test_sample_sponsoring() {
        // from https://dataresearchcenter.org/library/de_abgeordnetenwatch_sponsoring/
        let test_file = "sample/de_abgeordnetenwatch_sponsoring.ftm.json.zst";
        let test_file = BufReader::new(File::open(test_file).unwrap());
        let test_data = zstd::decode_all(test_file).unwrap();
        for line in test_data.lines() {
            let line = line.unwrap();
            let entity = FtmEntity::from_ftm_json(&line).unwrap();
            // Verify we can access basic properties
            assert!(!entity.id().is_empty());
            assert!(!entity.schema().is_empty());
        }
    }

    #[test]
    fn test_from_ftm_json_person_schema() {
        // Regression test: with #[serde(untagged)], Address (alphabetically first) was
        // matching Person entities because both structs share the same required fields
        // (id, schema) and all property fields are Option<Vec<String>>.
        // height omitted: the generated struct uses Vec<f64> but FTM encodes it as a string
        let json = r#"{"id": "e571b2b8ccfae7329036251acc47d0e833b280f5", "schema": "Person", "properties": {"motherName": ["nor"], "lastName": ["mention"], "nameSuffix": ["herself"], "birthDate": ["1961-05-20"], "birthPlace": ["data"], "nationality": ["et"], "appearance": ["to"], "religion": ["boy"], "profession": ["example"], "spokenLanguage": ["deu"], "abbreviation": ["Jamie Patterson"], "email": ["jamespalmer@example.com"], "incorporationDate": ["1959-11-10"], "taxStatus": ["eye"], "sector": ["democratic"], "registrationNumber": ["KKe-99272009"], "licenseNumber": ["rCG-26589103"], "opencorporatesUrl": ["http://www.avila-williams.biz/"], "bvdId": ["YFB-72857745"], "sayariId": ["kLt-25015135"], "brightQueryOrgId": ["DVr-24775349"], "icijId": ["real"], "name": ["Joseph Brown"], "description": ["Grow she future debate analysis much determine."], "alias": ["Jennifer Black"], "previousName": ["Anthony Davies"], "weakAlias": ["Nicole Smith"], "sourceUrl": ["http://www.wilson.com/"], "alephUrl": ["http://gilbert.com/"], "keywords": ["worker"], "createdAt": ["1998-05-22"], "retrievedAt": ["1998-11-28"]}}"#;

        let entity = FtmEntity::from_ftm_json(json).unwrap();

        assert_eq!(entity.schema(), "Person", "expected Person schema, got {} — likely matched Address due to untagged enum ordering", entity.schema());
        assert_eq!(entity.id(), "e571b2b8ccfae7329036251acc47d0e833b280f5");

        assert!(
            matches!(entity, FtmEntity::Person(_)),
            "entity should be FtmEntity::Person, got FtmEntity::{}", entity.schema()
        );
    }

    #[test]
    fn test_sample_sidejobs() {
        // from https://dataresearchcenter.org/library/de_abgeordnetenwatch_sidejobs/
        let test_file = "sample/de_abgeordnetenwatch_sidejobs.ftm.json.zst";
        let test_file = BufReader::new(File::open(test_file).unwrap());
        let test_data = zstd::decode_all(test_file).unwrap();
        for line in test_data.lines() {
            let line = line.unwrap();
            let entity = FtmEntity::from_ftm_json(&line).unwrap();
            // Verify we can access basic properties
            assert!(!entity.id().is_empty());
            assert!(!entity.schema().is_empty());
        }
    }
}
