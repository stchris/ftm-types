#![allow(missing_docs)]
#[cfg(feature = "rand")]
use enum_derived::Rand;
use serde::{Deserialize, Serialize};
#[cfg(feature = "rand")]
fn default_json_value() -> Option<serde_json::Value> {
    Some(serde_json::Value::Object(serde_json::Map::new()))
}
#[cfg(feature = "builder")]
use bon::Builder;
///FTM Schema: Address
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Address".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Full address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full: Option<Vec<String>>,
    ///Property: Google Places ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Latitude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<Vec<f64>>,
    ///Property: Longitude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<Vec<f64>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: OpenStreetmap Place ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osm_id: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Address {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Address".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            country: None,
            created_at: None,
            description: None,
            full: None,
            google_place_id: None,
            index_text: None,
            latitude: None,
            longitude: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            osm_id: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            topics: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Address"
    }
}
///FTM Schema: Airplane
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Airplane {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Airplane".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Build Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: De-registration Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deregistration_date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: ICAO aircraft type designator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icao_code: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Operator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<Vec<String>>,
    ///Property: Owner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Serial Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Airplane {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Airplane".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            amount: None,
            amount_eur: None,
            amount_usd: None,
            build_date: None,
            country: None,
            created_at: None,
            deregistration_date: None,
            description: None,
            icao_code: None,
            index_text: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            operator: None,
            owner: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            registration_date: None,
            registration_number: None,
            retrieved_at: None,
            serial_number: None,
            source_url: None,
            summary: None,
            topics: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Airplane"
    }
}
///FTM Schema: Article
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Article".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Vec<f64>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Article {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Article".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            ancestors: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            file_size: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            language: None,
            location_mentioned: None,
            mime_type: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            parent: None,
            people_mentioned: None,
            phone_mentioned: None,
            previous_name: None,
            processed_at: None,
            processing_agent: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            title: None,
            topics: None,
            translated_language: None,
            translated_text: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Article"
    }
}
///FTM Schema: Asset
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Asset".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Asset {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Asset".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            amount: None,
            amount_eur: None,
            amount_usd: None,
            country: None,
            created_at: None,
            description: None,
            index_text: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            topics: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Asset"
    }
}
///FTM Schema: Associate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Associate {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Associate".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Associate
    pub associate: Vec<String>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Person
    pub person: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
}
impl Associate {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Associate".to_string(),
            aleph_url: None,
            associate: Vec::new(),
            date: None,
            description: None,
            end_date: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            person: Vec::new(),
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Associate"
    }
}
///FTM Schema: Audio
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Audio {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Audio".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Vec<f64>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Vec<f64>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Sampling Rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rate: Option<Vec<f64>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Audio {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Audio".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            ancestors: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            duration: None,
            email_mentioned: None,
            file_size: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            language: None,
            location_mentioned: None,
            mime_type: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            parent: None,
            people_mentioned: None,
            phone_mentioned: None,
            previous_name: None,
            processed_at: None,
            processing_agent: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher_url: None,
            retrieved_at: None,
            sampling_rate: None,
            source_url: None,
            summary: None,
            title: None,
            topics: None,
            translated_language: None,
            translated_text: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Audio"
    }
}
///FTM Schema: Bank account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct BankAccount {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "BankAccount".to_string()))]
    pub schema: String,
    ///Property: Account number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<Vec<f64>>,
    ///Property: Balance date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_date: Option<Vec<String>>,
    ///Property: Bank
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<Vec<String>>,
    ///Property: Bank Identifier Code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<Vec<String>>,
    ///Property: Closing date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closing_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: IBAN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Maximum balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_balance: Option<Vec<f64>>,
    ///Property: Maximum balance date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_balance_date: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Opening date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_date: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl BankAccount {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "BankAccount".to_string(),
            account_number: None,
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            amount: None,
            amount_eur: None,
            amount_usd: None,
            balance: None,
            balance_date: None,
            bank: None,
            bic: None,
            closing_date: None,
            country: None,
            created_at: None,
            description: None,
            iban: None,
            index_text: None,
            max_balance: None,
            max_balance_date: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            opening_date: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            topics: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "BankAccount"
    }
}
///FTM Schema: Call
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Call {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Call".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Caller
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller: Option<Vec<String>>,
    ///Property: Caller's Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_number: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Vec<f64>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Receiver
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<Vec<String>>,
    ///Property: Receiver's Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
}
impl Call {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Call".to_string(),
            aleph_url: None,
            caller: None,
            caller_number: None,
            date: None,
            description: None,
            duration: None,
            end_date: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            proof: None,
            publisher_url: None,
            receiver: None,
            receiver_number: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Call"
    }
}
///FTM Schema: Call for tenders
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct CallForTenders {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "CallForTenders".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Name of contracting authority
    pub authority: Vec<String>,
    ///Property: Contracting authority reference ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority_reference_id: Option<Vec<String>>,
    ///Property: Award Notice Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub award_notice_date: Option<Vec<String>>,
    ///Property: Date of awarding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarding_date: Option<Vec<String>>,
    ///Property: CfT unique id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_id: Option<Vec<String>>,
    ///Property: End of clarification period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clarification_deadline: Option<Vec<String>>,
    ///Property: Contract notice date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_notice_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: CPV code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpv_code: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Maximum number of lots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_number_of_lots: Option<Vec<f64>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Number of lots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_lots: Option<Vec<f64>>,
    ///Property: NUTS code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nuts_code: Option<Vec<String>>,
    ///Property: Published on behalf of
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Date of publication/invitation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Submission deadline
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_deadline: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: TED link for published notices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ted_url: Option<Vec<String>>,
    ///Property: Tenderers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenderers: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl CallForTenders {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "CallForTenders".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            authority: Vec::new(),
            authority_reference_id: None,
            award_notice_date: None,
            awarding_date: None,
            call_id: None,
            clarification_deadline: None,
            contract_notice_date: None,
            country: None,
            cpv_code: None,
            created_at: None,
            date: None,
            description: None,
            end_date: None,
            index_text: None,
            maximum_number_of_lots: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            number_of_lots: None,
            nuts_code: None,
            on_behalf_of: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publication_date: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            submission_deadline: None,
            summary: None,
            ted_url: None,
            tenderers: None,
            topics: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "CallForTenders"
    }
}
///FTM Schema: Company
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Company".to_string()))]
    pub schema: String,
    ///Property: Abbreviation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: BIK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bik_code: Option<Vec<String>>,
    ///Property: BrightQuery ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_query_id: Option<Vec<String>>,
    ///Property: BrightQuery Organization ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_query_org_id: Option<Vec<String>>,
    ///Property: Bureau van Dijk ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bvd_id: Option<Vec<String>>,
    ///Property: CAGE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cage_code: Option<Vec<String>>,
    ///Property: SEC Central Index Key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cik_code: Option<Vec<String>>,
    ///Property: COATO / SOATO / OKATO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coato_code: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Dissolution date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolution_date: Option<Vec<String>>,
    ///Property: DUNS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duns_code: Option<Vec<String>>,
    ///Property: E-Mail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    ///Property: Federal tax service code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fns_code: Option<Vec<String>>,
    ///Property: GIIN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gii_number: Option<Vec<String>>,
    ///Property: ID Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<Vec<String>>,
    ///Property: IMO Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imo_number: Option<Vec<String>>,
    ///Property: Incorporation date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incorporation_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: INN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inn_code: Option<Vec<String>>,
    ///Property: IPO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipo_code: Option<Vec<String>>,
    ///Property: IRS Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irs_code: Option<Vec<String>>,
    ///Property: ISIN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isin_code: Option<Vec<String>>,
    ///Property: JIB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jib_code: Option<Vec<String>>,
    ///Property: Jurisdiction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<Vec<String>>,
    ///Property: KPP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpp_code: Option<Vec<String>>,
    ///Property: LEI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei_code: Option<Vec<String>>,
    ///Property: License Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_number: Option<Vec<String>>,
    ///Property: Country of origin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_country: Option<Vec<String>>,
    ///Property: MBS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mbs_code: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: NPI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npi_code: Option<Vec<String>>,
    ///Property: OGRN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ogrn_code: Option<Vec<String>>,
    ///Property: OKPO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub okpo_code: Option<Vec<String>>,
    ///Property: OpenCorporates URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opencorporates_url: Option<Vec<String>>,
    ///Property: Parent company
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: PermID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perm_id: Option<Vec<String>>,
    ///Property: PFR Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pfr_number: Option<Vec<String>>,
    ///Property: Phone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Reuters Instrument Code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ric_code: Option<Vec<String>>,
    ///Property: Sayari Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sayari_id: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: SWIFT/BIC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift_bic: Option<Vec<String>>,
    ///Property: Tax Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<Vec<String>>,
    ///Property: Stock ticker symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Unique Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_entity_id: Option<Vec<String>>,
    ///Property: USCC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usc_code: Option<Vec<String>>,
    ///Property: V.A.T. Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<Vec<String>>,
    ///Property: VOEN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voen_code: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Website
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Company {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Company".to_string(),
            abbreviation: None,
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            amount: None,
            amount_eur: None,
            amount_usd: None,
            bik_code: None,
            bright_query_id: None,
            bright_query_org_id: None,
            bvd_id: None,
            cage_code: None,
            cik_code: None,
            coato_code: None,
            country: None,
            created_at: None,
            description: None,
            dissolution_date: None,
            duns_code: None,
            email: None,
            fns_code: None,
            gii_number: None,
            id_number: None,
            imo_number: None,
            incorporation_date: None,
            index_text: None,
            inn_code: None,
            ipo_code: None,
            irs_code: None,
            isin_code: None,
            jib_code: None,
            jurisdiction: None,
            kpp_code: None,
            lei_code: None,
            license_number: None,
            main_country: None,
            mbs_code: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            npi_code: None,
            ogrn_code: None,
            okpo_code: None,
            opencorporates_url: None,
            parent: None,
            perm_id: None,
            pfr_number: None,
            phone: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            registration_number: None,
            retrieved_at: None,
            ric_code: None,
            sayari_id: None,
            source_url: None,
            summary: None,
            swift_bic: None,
            tax_number: None,
            ticker: None,
            topics: None,
            unique_entity_id: None,
            usc_code: None,
            vat_code: None,
            voen_code: None,
            weak_alias: None,
            website: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Company"
    }
}
///FTM Schema: Contract
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Contract".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Contract authority
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority: Option<Vec<String>>,
    ///Property: Contract date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    pub title: Vec<String>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Contract {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Contract".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            amount: None,
            amount_eur: None,
            amount_usd: None,
            authority: None,
            contract_date: None,
            country: None,
            created_at: None,
            description: None,
            index_text: None,
            language: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            previous_name: None,
            program_id: None,
            project: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            title: Vec::new(),
            topics: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Contract"
    }
}
///FTM Schema: Contract award
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct ContractAward {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "ContractAward".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Call For Tenders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_for_tenders: Option<Vec<String>>,
    ///Property: Contract
    pub contract: Vec<String>,
    ///Property: CPV code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpv_code: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Decision reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_reason: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: NUTS code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nuts_code: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Supplier
    pub supplier: Vec<String>,
}
impl ContractAward {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "ContractAward".to_string(),
            aleph_url: None,
            amount: None,
            amount_eur: None,
            amount_usd: None,
            call_for_tenders: None,
            contract: Vec::new(),
            cpv_code: None,
            date: None,
            decision_reason: None,
            description: None,
            end_date: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            nuts_code: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
            supplier: Vec::new(),
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "ContractAward"
    }
}
///FTM Schema: Court case
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct CourtCase {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "CourtCase".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Case number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_number: Option<Vec<String>>,
    ///Property: Close date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: File date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl CourtCase {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "CourtCase".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            case_number: None,
            close_date: None,
            country: None,
            created_at: None,
            description: None,
            file_date: None,
            index_text: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            topics: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "CourtCase"
    }
}
///FTM Schema: Case party
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct CourtCaseParty {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "CourtCaseParty".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Case
    pub case: Vec<String>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Party
    pub party: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
}
impl CourtCaseParty {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "CourtCaseParty".to_string(),
            aleph_url: None,
            case: Vec::new(),
            date: None,
            description: None,
            end_date: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            party: Vec::new(),
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "CourtCaseParty"
    }
}
///FTM Schema: Cryptocurrency wallet
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct CryptoWallet {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "CryptoWallet".to_string()))]
    pub schema: String,
    ///Property: Account ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<Vec<f64>>,
    ///Property: Balance date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Creation date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Wallet holder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl CryptoWallet {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "CryptoWallet".to_string(),
            account_id: None,
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            amount: None,
            amount_eur: None,
            amount_usd: None,
            balance: None,
            balance_date: None,
            country: None,
            created_at: None,
            creation_date: None,
            description: None,
            holder: None,
            index_text: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            previous_name: None,
            program_id: None,
            proof: None,
            public_key: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            topics: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "CryptoWallet"
    }
}
///FTM Schema: Debt
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Debt {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Debt".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Creditor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creditor: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Debtor
    pub debtor: Vec<String>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
}
impl Debt {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Debt".to_string(),
            aleph_url: None,
            amount: None,
            amount_eur: None,
            amount_usd: None,
            creditor: None,
            date: None,
            debtor: Vec::new(),
            description: None,
            end_date: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Debt"
    }
}
///FTM Schema: Directorship
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Directorship {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Directorship".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Director
    pub director: Vec<String>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Organization
    pub organization: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
}
impl Directorship {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Directorship".to_string(),
            aleph_url: None,
            date: None,
            description: None,
            director: Vec::new(),
            end_date: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            organization: Vec::new(),
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Directorship"
    }
}
///FTM Schema: File
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Document".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Vec<f64>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Document {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Document".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            ancestors: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            file_size: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            language: None,
            location_mentioned: None,
            mime_type: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            parent: None,
            people_mentioned: None,
            phone_mentioned: None,
            previous_name: None,
            processed_at: None,
            processing_agent: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            title: None,
            topics: None,
            translated_language: None,
            translated_text: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Document"
    }
}
///FTM Schema: Customs declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct EconomicActivity {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "EconomicActivity".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Bank Account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<Vec<String>>,
    ///Property: Foreign currency bank
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_foreign: Option<Vec<String>>,
    ///Property: Rouble bank
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_rub: Option<Vec<String>>,
    ///Property: Customs Cargo Declaration Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccd_number: Option<Vec<String>>,
    ///Property: Contract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<Vec<String>>,
    ///Property: Contract holder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_holder: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Declarant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declarant: Option<Vec<String>>,
    ///Property: Country of departure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_country: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Country of destination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_country: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Description of goods
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_description: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Country of origin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_country: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Receiver
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Trading Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_country: Option<Vec<String>>,
    ///Property: Transport
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<Vec<String>>,
    ///Property: FEAC Code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ved_code: Option<Vec<String>>,
}
impl EconomicActivity {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "EconomicActivity".to_string(),
            aleph_url: None,
            bank_account: None,
            bank_foreign: None,
            bank_rub: None,
            ccd_number: None,
            contract: None,
            contract_holder: None,
            date: None,
            declarant: None,
            departure_country: None,
            description: None,
            destination_country: None,
            end_date: None,
            goods_description: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            origin_country: None,
            proof: None,
            publisher_url: None,
            receiver: None,
            retrieved_at: None,
            sender: None,
            source_url: None,
            start_date: None,
            summary: None,
            trading_country: None,
            transport: None,
            ved_code: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "EconomicActivity"
    }
}
///FTM Schema: E-Mail
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Email {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Email".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authored_at: Option<Vec<String>>,
    ///Property: BCC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    ///Property: HTML
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<Vec<String>>,
    ///Property: CC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: Emitter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emitters: Option<Vec<String>>,
    ///Property: File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Vec<f64>>,
    ///Property: From
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<String>>,
    ///Property: Raw headers
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "rand", custom_rand(default_json_value))]
    pub headers: Option<serde_json::Value>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Responding to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to_email: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Recipients
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Subject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Thread topic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_topic: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<String>>,
    ///Property: To
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Email {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Email".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            ancestors: None,
            authored_at: None,
            bcc: None,
            body_html: None,
            body_text: None,
            cc: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            emitters: None,
            file_size: None,
            from: None,
            headers: None,
            iban_mentioned: None,
            in_reply_to_email: None,
            index_text: None,
            ip_mentioned: None,
            language: None,
            location_mentioned: None,
            mime_type: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            parent: None,
            people_mentioned: None,
            phone_mentioned: None,
            previous_name: None,
            processed_at: None,
            processing_agent: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher_url: None,
            recipients: None,
            retrieved_at: None,
            sender: None,
            source_url: None,
            subject: None,
            summary: None,
            thread_topic: None,
            title: None,
            to: None,
            topics: None,
            translated_language: None,
            translated_text: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Email"
    }
}
///FTM Schema: Employment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Employment {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Employment".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Employee
    pub employee: Vec<String>,
    ///Property: Employer
    pub employer: Vec<String>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
}
impl Employment {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Employment".to_string(),
            aleph_url: None,
            date: None,
            description: None,
            employee: Vec::new(),
            employer: Vec::new(),
            end_date: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Employment"
    }
}
///FTM Schema: Event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Event".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Involved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub involved: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Organizer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizer: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Event {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Event".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            companies_mentioned: None,
            country: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            end_date: None,
            iban_mentioned: None,
            index_text: None,
            involved: None,
            ip_mentioned: None,
            location: None,
            location_mentioned: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            organizer: None,
            people_mentioned: None,
            phone_mentioned: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
            topics: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Event"
    }
}
///FTM Schema: Family
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Family {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Family".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Person
    pub person: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Relative
    pub relative: Vec<String>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
}
impl Family {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Family".to_string(),
            aleph_url: None,
            date: None,
            description: None,
            end_date: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            person: Vec::new(),
            proof: None,
            publisher_url: None,
            relative: Vec::new(),
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Family"
    }
}
///FTM Schema: Folder
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Folder".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Vec<f64>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Folder {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Folder".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            ancestors: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            file_size: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            language: None,
            location_mentioned: None,
            mime_type: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            parent: None,
            people_mentioned: None,
            phone_mentioned: None,
            previous_name: None,
            processed_at: None,
            processing_agent: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            title: None,
            topics: None,
            translated_language: None,
            translated_text: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Folder"
    }
}
///FTM Schema: Web page
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct HyperText {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "HyperText".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authored_at: Option<Vec<String>>,
    ///Property: HTML
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Vec<f64>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl HyperText {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "HyperText".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            ancestors: None,
            authored_at: None,
            body_html: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            file_size: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            language: None,
            location_mentioned: None,
            mime_type: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            parent: None,
            people_mentioned: None,
            phone_mentioned: None,
            previous_name: None,
            processed_at: None,
            processing_agent: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            title: None,
            topics: None,
            translated_language: None,
            translated_text: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "HyperText"
    }
}
///FTM Schema: Identification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Identification {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Identification".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Identification holder
    pub holder: Vec<String>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Document number
    pub number: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
}
impl Identification {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Identification".to_string(),
            aleph_url: None,
            country: None,
            date: None,
            description: None,
            end_date: None,
            holder: Vec::new(),
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            number: Vec::new(),
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Identification"
    }
}
///FTM Schema: Image
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Image".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Credit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Vec<f64>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Pictured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictured: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Image {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Image".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            ancestors: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            created_at: None,
            credit: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            file_size: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            language: None,
            location_mentioned: None,
            mime_type: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            parent: None,
            people_mentioned: None,
            phone_mentioned: None,
            pictured: None,
            previous_name: None,
            processed_at: None,
            processing_agent: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            title: None,
            topics: None,
            translated_language: None,
            translated_text: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Image"
    }
}
///FTM Schema: Legal entity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct LegalEntity {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "LegalEntity".to_string()))]
    pub schema: String,
    ///Property: Abbreviation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: BrightQuery ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_query_id: Option<Vec<String>>,
    ///Property: BrightQuery Organization ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_query_org_id: Option<Vec<String>>,
    ///Property: Bureau van Dijk ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bvd_id: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Dissolution date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolution_date: Option<Vec<String>>,
    ///Property: DUNS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duns_code: Option<Vec<String>>,
    ///Property: E-Mail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    ///Property: ID Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<Vec<String>>,
    ///Property: Incorporation date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incorporation_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: INN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inn_code: Option<Vec<String>>,
    ///Property: Jurisdiction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<Vec<String>>,
    ///Property: LEI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei_code: Option<Vec<String>>,
    ///Property: License Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_number: Option<Vec<String>>,
    ///Property: Country of origin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_country: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: NPI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npi_code: Option<Vec<String>>,
    ///Property: OGRN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ogrn_code: Option<Vec<String>>,
    ///Property: OKPO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub okpo_code: Option<Vec<String>>,
    ///Property: OpenCorporates URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opencorporates_url: Option<Vec<String>>,
    ///Property: Parent company
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: Phone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Sayari Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sayari_id: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: SWIFT/BIC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift_bic: Option<Vec<String>>,
    ///Property: Tax Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Unique Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_entity_id: Option<Vec<String>>,
    ///Property: USCC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usc_code: Option<Vec<String>>,
    ///Property: V.A.T. Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Website
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl LegalEntity {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "LegalEntity".to_string(),
            abbreviation: None,
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            bright_query_id: None,
            bright_query_org_id: None,
            bvd_id: None,
            country: None,
            created_at: None,
            description: None,
            dissolution_date: None,
            duns_code: None,
            email: None,
            id_number: None,
            incorporation_date: None,
            index_text: None,
            inn_code: None,
            jurisdiction: None,
            lei_code: None,
            license_number: None,
            main_country: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            npi_code: None,
            ogrn_code: None,
            okpo_code: None,
            opencorporates_url: None,
            parent: None,
            phone: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            registration_number: None,
            retrieved_at: None,
            sayari_id: None,
            source_url: None,
            summary: None,
            swift_bic: None,
            tax_number: None,
            topics: None,
            unique_entity_id: None,
            usc_code: None,
            vat_code: None,
            weak_alias: None,
            website: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "LegalEntity"
    }
}
///FTM Schema: License
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "License".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Area
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<Vec<f64>>,
    ///Property: Contract authority
    pub authority: Vec<String>,
    ///Property: Contract date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    pub title: Vec<String>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl License {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "License".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            amount: None,
            amount_eur: None,
            amount_usd: None,
            area: None,
            authority: Vec::new(),
            contract_date: None,
            country: None,
            created_at: None,
            description: None,
            index_text: None,
            language: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            previous_name: None,
            program_id: None,
            project: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            title: Vec::new(),
            topics: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "License"
    }
}
///FTM Schema: Membership
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Membership {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Membership".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Member
    pub member: Vec<String>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Organization
    pub organization: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
}
impl Membership {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Membership".to_string(),
            aleph_url: None,
            date: None,
            description: None,
            end_date: None,
            index_text: None,
            member: Vec::new(),
            modified_at: None,
            names_mentioned: None,
            organization: Vec::new(),
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Membership"
    }
}
///FTM Schema: Mention
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Mention".to_string()))]
    pub schema: String,
    ///Property: Co-occurring countries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_country: Option<Vec<String>>,
    ///Property: Co-occurring e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_email: Option<Vec<String>>,
    ///Property: Co-occurring phone numbers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_phone: Option<Vec<String>>,
    ///Property: Document
    pub document: Vec<String>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Entity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<Vec<String>>,
}
impl Mention {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Mention".to_string(),
            context_country: None,
            context_email: None,
            context_phone: None,
            document: Vec::new(),
            name: Vec::new(),
            resolved: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Mention"
    }
}
///FTM Schema: Message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Message".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authored_at: Option<Vec<String>>,
    ///Property: HTML
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<Vec<String>>,
    ///Property: Text
    pub body_text: Vec<String>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Vec<f64>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Responding to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to_message: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "rand", custom_rand(default_json_value))]
    pub metadata: Option<serde_json::Value>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Recipient Account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_account: Option<Vec<String>>,
    ///Property: Recipients
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Sender
    pub sender: Vec<String>,
    ///Property: Sender Account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_account: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Subject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Thread topic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_topic: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Message {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Message".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            ancestors: None,
            authored_at: None,
            body_html: None,
            body_text: Vec::new(),
            companies_mentioned: None,
            content_hash: None,
            country: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            end_date: None,
            file_size: None,
            iban_mentioned: None,
            in_reply_to_message: None,
            index_text: None,
            ip_mentioned: None,
            language: None,
            location_mentioned: None,
            metadata: None,
            mime_type: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            parent: None,
            people_mentioned: None,
            phone_mentioned: None,
            previous_name: None,
            processed_at: None,
            processing_agent: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher_url: None,
            recipient_account: None,
            recipients: None,
            retrieved_at: None,
            sender: Vec::new(),
            sender_account: None,
            source_url: None,
            start_date: None,
            subject: None,
            summary: None,
            thread_topic: None,
            title: None,
            topics: None,
            translated_language: None,
            translated_text: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Message"
    }
}
///FTM Schema: Note
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Note".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: Entity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Vec<String>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Note {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Note".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            companies_mentioned: None,
            country: None,
            created_at: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            entity: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            location_mentioned: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            people_mentioned: None,
            phone_mentioned: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            topics: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Note"
    }
}
///FTM Schema: Occupancy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Occupancy {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Occupancy".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Declaration date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declaration_date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Holder
    pub holder: Vec<String>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Position occupied
    pub post: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
}
impl Occupancy {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Occupancy".to_string(),
            aleph_url: None,
            date: None,
            declaration_date: None,
            description: None,
            end_date: None,
            holder: Vec::new(),
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            post: Vec::new(),
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            status: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Occupancy"
    }
}
///FTM Schema: Organization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Organization".to_string()))]
    pub schema: String,
    ///Property: Abbreviation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: BrightQuery ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_query_id: Option<Vec<String>>,
    ///Property: BrightQuery Organization ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_query_org_id: Option<Vec<String>>,
    ///Property: Bureau van Dijk ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bvd_id: Option<Vec<String>>,
    ///Property: CAGE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cage_code: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Dissolution date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolution_date: Option<Vec<String>>,
    ///Property: DUNS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duns_code: Option<Vec<String>>,
    ///Property: E-Mail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    ///Property: GIIN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gii_number: Option<Vec<String>>,
    ///Property: ID Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<Vec<String>>,
    ///Property: IMO Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imo_number: Option<Vec<String>>,
    ///Property: Incorporation date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incorporation_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: INN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inn_code: Option<Vec<String>>,
    ///Property: Jurisdiction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<Vec<String>>,
    ///Property: LEI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei_code: Option<Vec<String>>,
    ///Property: License Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_number: Option<Vec<String>>,
    ///Property: Country of origin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_country: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: NPI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npi_code: Option<Vec<String>>,
    ///Property: OGRN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ogrn_code: Option<Vec<String>>,
    ///Property: OKPO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub okpo_code: Option<Vec<String>>,
    ///Property: OpenCorporates URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opencorporates_url: Option<Vec<String>>,
    ///Property: Parent company
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: PermID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perm_id: Option<Vec<String>>,
    ///Property: Phone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Sayari Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sayari_id: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: SWIFT/BIC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift_bic: Option<Vec<String>>,
    ///Property: Tax Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Unique Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_entity_id: Option<Vec<String>>,
    ///Property: USCC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usc_code: Option<Vec<String>>,
    ///Property: V.A.T. Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Website
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Organization {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Organization".to_string(),
            abbreviation: None,
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            bright_query_id: None,
            bright_query_org_id: None,
            bvd_id: None,
            cage_code: None,
            country: None,
            created_at: None,
            description: None,
            dissolution_date: None,
            duns_code: None,
            email: None,
            gii_number: None,
            id_number: None,
            imo_number: None,
            incorporation_date: None,
            index_text: None,
            inn_code: None,
            jurisdiction: None,
            lei_code: None,
            license_number: None,
            main_country: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            npi_code: None,
            ogrn_code: None,
            okpo_code: None,
            opencorporates_url: None,
            parent: None,
            perm_id: None,
            phone: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            registration_number: None,
            retrieved_at: None,
            sayari_id: None,
            source_url: None,
            summary: None,
            swift_bic: None,
            tax_number: None,
            topics: None,
            unique_entity_id: None,
            usc_code: None,
            vat_code: None,
            weak_alias: None,
            website: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Organization"
    }
}
///FTM Schema: Ownership
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Ownership {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Ownership".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Asset
    pub asset: Vec<String>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Owner
    pub owner: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
}
impl Ownership {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Ownership".to_string(),
            aleph_url: None,
            asset: Vec::new(),
            date: None,
            description: None,
            end_date: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            owner: Vec::new(),
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Ownership"
    }
}
///FTM Schema: Package
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Package".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Vec<f64>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Package {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Package".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            ancestors: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            file_size: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            language: None,
            location_mentioned: None,
            mime_type: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            parent: None,
            people_mentioned: None,
            phone_mentioned: None,
            previous_name: None,
            processed_at: None,
            processing_agent: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            title: None,
            topics: None,
            translated_language: None,
            translated_text: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Package"
    }
}
///FTM Schema: Page
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Page".to_string()))]
    pub schema: String,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Vec<String>>,
    ///Property: Index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Vec<f64>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_text: Option<Vec<String>>,
}
impl Page {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Page".to_string(),
            body_text: None,
            detected_language: None,
            document: None,
            index: None,
            index_text: None,
            translated_text: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Page"
    }
}
///FTM Schema: Document
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Pages {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Pages".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Vec<f64>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: PDF alternative version checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_hash: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Pages {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Pages".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            ancestors: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            file_size: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            language: None,
            location_mentioned: None,
            mime_type: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            parent: None,
            pdf_hash: None,
            people_mentioned: None,
            phone_mentioned: None,
            previous_name: None,
            processed_at: None,
            processing_agent: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            title: None,
            topics: None,
            translated_language: None,
            translated_text: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Pages"
    }
}
///FTM Schema: Passport
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Passport {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Passport".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Birth date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Gender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Vec<String>>,
    ///Property: Identification holder
    pub holder: Vec<String>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Document number
    pub number: Vec<String>,
    ///Property: Passport number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_number: Option<Vec<String>>,
    ///Property: Personal number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_number: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
}
impl Passport {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Passport".to_string(),
            aleph_url: None,
            birth_date: None,
            country: None,
            date: None,
            description: None,
            end_date: None,
            gender: None,
            holder: Vec::new(),
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            number: Vec::new(),
            passport_number: None,
            personal_number: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Passport"
    }
}
///FTM Schema: Payment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Payment".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Beneficiary
    pub beneficiary: Vec<String>,
    ///Property: Beneficiary bank account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_account: Option<Vec<String>>,
    ///Property: Contract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Payer
    pub payer: Vec<String>,
    ///Property: Payer bank account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_account: Option<Vec<String>>,
    ///Property: Project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Payment purpose
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
}
impl Payment {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Payment".to_string(),
            aleph_url: None,
            amount: None,
            amount_eur: None,
            amount_usd: None,
            beneficiary: Vec::new(),
            beneficiary_account: None,
            contract: None,
            date: None,
            description: None,
            end_date: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            payer: Vec::new(),
            payer_account: None,
            project: None,
            proof: None,
            publisher_url: None,
            purpose: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Payment"
    }
}
///FTM Schema: Person
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Person".to_string()))]
    pub schema: String,
    ///Property: Abbreviation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Country of birth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_country: Option<Vec<String>>,
    ///Property: Birth date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<Vec<String>>,
    ///Property: BrightQuery ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_query_id: Option<Vec<String>>,
    ///Property: BrightQuery Organization ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_query_org_id: Option<Vec<String>>,
    ///Property: Bureau van Dijk ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bvd_id: Option<Vec<String>>,
    ///Property: Citizenship
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citizenship: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Death date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub death_date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Dissolution date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolution_date: Option<Vec<String>>,
    ///Property: DUNS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duns_code: Option<Vec<String>>,
    ///Property: E-Mail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    ///Property: Gender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Vec<String>>,
    ///Property: Height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Vec<f64>>,
    ///Property: ID Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<Vec<String>>,
    ///Property: Incorporation date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incorporation_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: INN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inn_code: Option<Vec<String>>,
    ///Property: Jurisdiction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<Vec<String>>,
    ///Property: LEI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei_code: Option<Vec<String>>,
    ///Property: License Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_number: Option<Vec<String>>,
    ///Property: Country of origin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_country: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Nationality
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: NPI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npi_code: Option<Vec<String>>,
    ///Property: OGRN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ogrn_code: Option<Vec<String>>,
    ///Property: OKPO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub okpo_code: Option<Vec<String>>,
    ///Property: OpenCorporates URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opencorporates_url: Option<Vec<String>>,
    ///Property: Parent company
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: Passport number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_number: Option<Vec<String>>,
    ///Property: Phone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Sayari Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sayari_id: Option<Vec<String>>,
    ///Property: Social security number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_security_number: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Spoken language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoken_language: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: SWIFT/BIC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift_bic: Option<Vec<String>>,
    ///Property: Tax Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Unique Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_entity_id: Option<Vec<String>>,
    ///Property: USCC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usc_code: Option<Vec<String>>,
    ///Property: V.A.T. Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Website
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<Vec<String>>,
    ///Property: Weight
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<Vec<f64>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Person {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Person".to_string(),
            abbreviation: None,
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            birth_country: None,
            birth_date: None,
            bright_query_id: None,
            bright_query_org_id: None,
            bvd_id: None,
            citizenship: None,
            country: None,
            created_at: None,
            death_date: None,
            description: None,
            dissolution_date: None,
            duns_code: None,
            email: None,
            gender: None,
            height: None,
            id_number: None,
            incorporation_date: None,
            index_text: None,
            inn_code: None,
            jurisdiction: None,
            lei_code: None,
            license_number: None,
            main_country: None,
            modified_at: None,
            name: Vec::new(),
            nationality: None,
            notes: None,
            npi_code: None,
            ogrn_code: None,
            okpo_code: None,
            opencorporates_url: None,
            parent: None,
            passport_number: None,
            phone: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            registration_number: None,
            retrieved_at: None,
            sayari_id: None,
            social_security_number: None,
            source_url: None,
            spoken_language: None,
            summary: None,
            swift_bic: None,
            tax_number: None,
            topics: None,
            unique_entity_id: None,
            usc_code: None,
            vat_code: None,
            weak_alias: None,
            website: None,
            weight: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Person"
    }
}
///FTM Schema: Text file
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct PlainText {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "PlainText".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Vec<f64>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl PlainText {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "PlainText".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            ancestors: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            file_size: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            language: None,
            location_mentioned: None,
            mime_type: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            parent: None,
            people_mentioned: None,
            phone_mentioned: None,
            previous_name: None,
            processed_at: None,
            processing_agent: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            title: None,
            topics: None,
            translated_language: None,
            translated_text: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "PlainText"
    }
}
///FTM Schema: Position
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Position".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Dissolution date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolution_date: Option<Vec<String>>,
    ///Property: Inception date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inception_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Total number of seats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_seats: Option<Vec<f64>>,
    ///Property: Organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Subnational jurisdiction name or code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnational_area: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Position {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Position".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            country: None,
            created_at: None,
            description: None,
            dissolution_date: None,
            inception_date: None,
            index_text: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            number_of_seats: None,
            organization: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            subnational_area: None,
            summary: None,
            topics: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Position"
    }
}
///FTM Schema: Project
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Project".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Project ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Project {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Project".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            amount: None,
            amount_eur: None,
            amount_usd: None,
            country: None,
            created_at: None,
            date: None,
            description: None,
            end_date: None,
            index_text: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            previous_name: None,
            program_id: None,
            project_id: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
            topics: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Project"
    }
}
///FTM Schema: Project participant
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct ProjectParticipant {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "ProjectParticipant".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Participant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant: Option<Vec<String>>,
    ///Property: Project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
}
impl ProjectParticipant {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "ProjectParticipant".to_string(),
            aleph_url: None,
            date: None,
            description: None,
            end_date: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            participant: None,
            project: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "ProjectParticipant"
    }
}
///FTM Schema: Public body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct PublicBody {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "PublicBody".to_string()))]
    pub schema: String,
    ///Property: Abbreviation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: BrightQuery ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_query_id: Option<Vec<String>>,
    ///Property: BrightQuery Organization ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_query_org_id: Option<Vec<String>>,
    ///Property: Bureau van Dijk ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bvd_id: Option<Vec<String>>,
    ///Property: CAGE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cage_code: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Dissolution date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolution_date: Option<Vec<String>>,
    ///Property: DUNS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duns_code: Option<Vec<String>>,
    ///Property: E-Mail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    ///Property: GIIN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gii_number: Option<Vec<String>>,
    ///Property: ID Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<Vec<String>>,
    ///Property: IMO Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imo_number: Option<Vec<String>>,
    ///Property: Incorporation date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incorporation_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: INN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inn_code: Option<Vec<String>>,
    ///Property: Jurisdiction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<Vec<String>>,
    ///Property: LEI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei_code: Option<Vec<String>>,
    ///Property: License Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_number: Option<Vec<String>>,
    ///Property: Country of origin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_country: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: NPI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npi_code: Option<Vec<String>>,
    ///Property: OGRN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ogrn_code: Option<Vec<String>>,
    ///Property: OKPO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub okpo_code: Option<Vec<String>>,
    ///Property: OpenCorporates URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opencorporates_url: Option<Vec<String>>,
    ///Property: Parent company
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: PermID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perm_id: Option<Vec<String>>,
    ///Property: Phone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Sayari Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sayari_id: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: SWIFT/BIC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift_bic: Option<Vec<String>>,
    ///Property: Tax Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Unique Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_entity_id: Option<Vec<String>>,
    ///Property: USCC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usc_code: Option<Vec<String>>,
    ///Property: V.A.T. Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Website
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl PublicBody {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "PublicBody".to_string(),
            abbreviation: None,
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            bright_query_id: None,
            bright_query_org_id: None,
            bvd_id: None,
            cage_code: None,
            country: None,
            created_at: None,
            description: None,
            dissolution_date: None,
            duns_code: None,
            email: None,
            gii_number: None,
            id_number: None,
            imo_number: None,
            incorporation_date: None,
            index_text: None,
            inn_code: None,
            jurisdiction: None,
            lei_code: None,
            license_number: None,
            main_country: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            npi_code: None,
            ogrn_code: None,
            okpo_code: None,
            opencorporates_url: None,
            parent: None,
            perm_id: None,
            phone: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            registration_number: None,
            retrieved_at: None,
            sayari_id: None,
            source_url: None,
            summary: None,
            swift_bic: None,
            tax_number: None,
            topics: None,
            unique_entity_id: None,
            usc_code: None,
            vat_code: None,
            weak_alias: None,
            website: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "PublicBody"
    }
}
///FTM Schema: Real estate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct RealEstate {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "RealEstate".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Area
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<Vec<f64>>,
    ///Property: Cadastral code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cadastral_code: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Record date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Latitude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<Vec<f64>>,
    ///Property: Longitude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<Vec<f64>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Parent unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Title number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_number: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl RealEstate {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "RealEstate".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            amount: None,
            amount_eur: None,
            amount_usd: None,
            area: None,
            cadastral_code: None,
            country: None,
            create_date: None,
            created_at: None,
            description: None,
            index_text: None,
            latitude: None,
            longitude: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            parent: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            registration_number: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            title_number: None,
            topics: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "RealEstate"
    }
}
///FTM Schema: Representation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Representation {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Representation".to_string()))]
    pub schema: String,
    ///Property: Agent
    pub agent: Vec<String>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Client
    pub client: Vec<String>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
}
impl Representation {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Representation".to_string(),
            agent: Vec::new(),
            aleph_url: None,
            client: Vec::new(),
            date: None,
            description: None,
            end_date: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Representation"
    }
}
///FTM Schema: Risk
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Risk {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Risk".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Vec<f64>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Entity
    pub entity: Vec<String>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Listing date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listing_date: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
}
impl Risk {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Risk".to_string(),
            aleph_url: None,
            country: None,
            date: None,
            description: None,
            duration: None,
            end_date: None,
            entity: Vec::new(),
            index_text: None,
            listing_date: None,
            modified_at: None,
            names_mentioned: None,
            proof: None,
            publisher_url: None,
            reason: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
            topics: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Risk"
    }
}
///FTM Schema: Sanction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Sanction {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Sanction".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Authority-issued identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority_id: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Vec<f64>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Entity
    pub entity: Vec<String>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Listing date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listing_date: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Program URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_url: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: UN SC identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsc_id: Option<Vec<String>>,
}
impl Sanction {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Sanction".to_string(),
            aleph_url: None,
            authority_id: None,
            country: None,
            date: None,
            description: None,
            duration: None,
            end_date: None,
            entity: Vec::new(),
            index_text: None,
            listing_date: None,
            modified_at: None,
            names_mentioned: None,
            program_id: None,
            program_url: None,
            proof: None,
            publisher_url: None,
            reason: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
            unsc_id: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Sanction"
    }
}
///FTM Schema: Security
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Security {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Security".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Financial Instrument Global Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub figi_code: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: ISIN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isin: Option<Vec<String>>,
    ///Property: Date issued
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<Vec<String>>,
    ///Property: Issuer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<Vec<String>>,
    ///Property: Maturity date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity_date: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Stock ticker symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Security {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Security".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            amount: None,
            amount_eur: None,
            amount_usd: None,
            country: None,
            created_at: None,
            description: None,
            figi_code: None,
            index_text: None,
            isin: None,
            issue_date: None,
            issuer: None,
            maturity_date: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            registration_number: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            ticker: None,
            topics: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Security"
    }
}
///FTM Schema: Similar
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Similar {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Similar".to_string()))]
    pub schema: String,
    ///Property: Candidate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate: Option<Vec<String>>,
    ///Property: Confidence score
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_score: Option<Vec<f64>>,
    ///Property: Match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_: Option<Vec<String>>,
}
impl Similar {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Similar".to_string(),
            candidate: None,
            confidence_score: None,
            match_: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Similar"
    }
}
///FTM Schema: Succession
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Succession {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Succession".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Predecessor
    pub predecessor: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Successor
    pub successor: Vec<String>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
}
impl Succession {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Succession".to_string(),
            aleph_url: None,
            date: None,
            description: None,
            end_date: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            predecessor: Vec::new(),
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            successor: Vec::new(),
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Succession"
    }
}
///FTM Schema: Table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Table".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<Vec<String>>,
    ///Property: Column headings
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "rand", custom_rand(default_json_value))]
    pub columns: Option<serde_json::Value>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: CSV alternative version checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_hash: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Vec<f64>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Number of rows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<Vec<f64>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Table {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Table".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            ancestors: None,
            authored_at: None,
            body_text: None,
            columns: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            created_at: None,
            csv_hash: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            file_size: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            language: None,
            location_mentioned: None,
            mime_type: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            parent: None,
            people_mentioned: None,
            phone_mentioned: None,
            previous_name: None,
            processed_at: None,
            processing_agent: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher_url: None,
            retrieved_at: None,
            row_count: None,
            source_url: None,
            summary: None,
            title: None,
            topics: None,
            translated_language: None,
            translated_text: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Table"
    }
}
///FTM Schema: Tax roll
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct TaxRoll {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "TaxRoll".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Birth date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Taxee
    pub taxee: Vec<String>,
}
impl TaxRoll {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "TaxRoll".to_string(),
            aleph_url: None,
            birth_date: None,
            country: None,
            date: None,
            description: None,
            end_date: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
            taxee: Vec::new(),
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "TaxRoll"
    }
}
///FTM Schema: Trip
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Trip {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Trip".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: End location
    pub end_location: Vec<String>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Involved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub involved: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Organizer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizer: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Start location
    pub start_location: Vec<String>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Vehicle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Trip {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Trip".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            companies_mentioned: None,
            country: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            end_date: None,
            end_location: Vec::new(),
            iban_mentioned: None,
            index_text: None,
            involved: None,
            ip_mentioned: None,
            location: None,
            location_mentioned: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            organizer: None,
            people_mentioned: None,
            phone_mentioned: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            start_location: Vec::new(),
            summary: None,
            topics: None,
            vehicle: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Trip"
    }
}
///FTM Schema: Other link
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct UnknownLink {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "UnknownLink".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Object
    pub object: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Vec<String>>,
    ///Property: Subject
    pub subject: Vec<String>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
}
impl UnknownLink {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "UnknownLink".to_string(),
            aleph_url: None,
            date: None,
            description: None,
            end_date: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            object: Vec::new(),
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            subject: Vec::new(),
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "UnknownLink"
    }
}
///FTM Schema: User account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct UserAccount {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "UserAccount".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: E-Mail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Owner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Vec<String>>,
    ///Property: Password
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<Vec<String>>,
    ///Property: Phone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Username
    pub username: Vec<String>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl UserAccount {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "UserAccount".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            country: None,
            created_at: None,
            description: None,
            email: None,
            index_text: None,
            ip_address: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            owner: None,
            password: None,
            phone: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            retrieved_at: None,
            service: None,
            source_url: None,
            summary: None,
            topics: None,
            username: Vec::new(),
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "UserAccount"
    }
}
///FTM Schema: Vehicle
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Vehicle {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Vehicle".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Build Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: De-registration Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deregistration_date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Operator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<Vec<String>>,
    ///Property: Owner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Vehicle {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Vehicle".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            amount: None,
            amount_eur: None,
            amount_usd: None,
            build_date: None,
            country: None,
            created_at: None,
            deregistration_date: None,
            description: None,
            index_text: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            operator: None,
            owner: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            registration_date: None,
            registration_number: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            topics: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Vehicle"
    }
}
///FTM Schema: Vessel
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Vessel {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Vessel".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Build Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_date: Option<Vec<String>>,
    ///Property: Call Sign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_sign: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: CRS Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crs_number: Option<Vec<String>>,
    ///Property: Deadweight Tonnage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadweight_tonnage: Option<Vec<f64>>,
    ///Property: De-registration Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deregistration_date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag: Option<Vec<String>>,
    ///Property: Gross Registered Tonnage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_registered_tonnage: Option<Vec<f64>>,
    ///Property: IMO Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imo_number: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: MMSI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmsi: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Date of Name Change
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_change_date: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Operator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<Vec<String>>,
    ///Property: Owner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Vec<String>>,
    ///Property: Past Flags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_flags: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Tonnage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tonnage: Option<Vec<f64>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Vessel {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Vessel".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            amount: None,
            amount_eur: None,
            amount_usd: None,
            build_date: None,
            call_sign: None,
            country: None,
            created_at: None,
            crs_number: None,
            deadweight_tonnage: None,
            deregistration_date: None,
            description: None,
            flag: None,
            gross_registered_tonnage: None,
            imo_number: None,
            index_text: None,
            mmsi: None,
            modified_at: None,
            name: Vec::new(),
            name_change_date: None,
            notes: None,
            operator: None,
            owner: None,
            past_flags: None,
            previous_name: None,
            program_id: None,
            proof: None,
            publisher_url: None,
            registration_date: None,
            registration_number: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            tonnage: None,
            topics: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Vessel"
    }
}
///FTM Schema: Video
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Video".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Vec<f64>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Vec<f64>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Video {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Video".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            ancestors: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            duration: None,
            email_mentioned: None,
            file_size: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            language: None,
            location_mentioned: None,
            mime_type: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            parent: None,
            people_mentioned: None,
            phone_mentioned: None,
            previous_name: None,
            processed_at: None,
            processing_agent: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            title: None,
            topics: None,
            translated_language: None,
            translated_text: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Video"
    }
}
///FTM Schema: Workbook
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rand", derive(Rand))]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Workbook {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Workbook".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Vec<f64>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia_url: Option<Vec<String>>,
}
impl Workbook {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Workbook".to_string(),
            address: None,
            address_entity: None,
            aleph_url: None,
            alias: None,
            ancestors: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            file_size: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            language: None,
            location_mentioned: None,
            mime_type: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            parent: None,
            people_mentioned: None,
            phone_mentioned: None,
            previous_name: None,
            processed_at: None,
            processing_agent: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            title: None,
            topics: None,
            translated_language: None,
            translated_text: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Workbook"
    }
}
