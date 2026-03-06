#![allow(missing_docs)]
#[cfg(feature = "builder")]
use bon::Builder;
use serde::{Deserialize, Serialize};
/// Deserialize a `Vec<f64>` whose elements may arrive as JSON strings
/// (e.g. `["6000.00"]`) or as JSON numbers (e.g. `[6000.0]`).
fn deserialize_f64_vec<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Vec::<serde_json::Value>::deserialize(deserializer)?
        .into_iter()
        .map(|v| match v {
            serde_json::Value::Number(n) => n
                .as_f64()
                .ok_or_else(|| serde::de::Error::custom("number out of f64 range")),
            serde_json::Value::String(s) => s.parse::<f64>().map_err(serde::de::Error::custom),
            other => Err(serde::de::Error::custom(format!(
                "expected number or numeric string, got {other}"
            ))),
        })
        .collect()
}
/// Same as [`deserialize_f64_vec`] but wrapped in `Some`.
/// Used for optional number fields so the field can still be absent (`None`)
/// while a present value tolerates string-encoded numbers.
fn deserialize_opt_f64_vec<'de, D>(deserializer: D) -> Result<Option<Vec<f64>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_f64_vec(deserializer).map(Some)
}
///FTM Schema: Address
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Address".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: City
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub city: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Full address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub full: Option<Vec<String>>,
    ///Property: Google Places ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub google_place_id: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Latitude
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub latitude: Option<Vec<f64>>,
    ///Property: Longitude
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub longitude: Option<Vec<f64>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: OpenStreetmap Place ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub osm_id: Option<Vec<String>>,
    ///Property: PO Box
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub post_office_box: Option<Vec<String>>,
    ///Property: Postal code
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub postal_code: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Region
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub region: Option<Vec<String>>,
    ///Property: Remarks
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub remarks: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: State
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub state: Option<Vec<String>>,
    ///Property: Street address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub street: Option<Vec<String>>,
    ///Property: Street address (ctd.)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub street2: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            city: None,
            country: None,
            created_at: None,
            description: None,
            full: None,
            google_place_id: None,
            index_text: None,
            keywords: None,
            latitude: None,
            longitude: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            osm_id: None,
            post_office_box: None,
            postal_code: None,
            previous_name: None,
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            region: None,
            remarks: None,
            retrieved_at: None,
            source_url: None,
            state: None,
            street: None,
            street2: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Airplane
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Airplane {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Airplane".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Build Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub build_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub currency: Option<Vec<String>>,
    ///Property: De-registration Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub deregistration_date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: ICAO aircraft type designator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub icao_code: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Manufacturer
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub manufacturer: Option<Vec<String>>,
    ///Property: Model
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub model: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Operator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub operator: Option<Vec<String>>,
    ///Property: Owner
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub owner: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub registration_date: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Serial Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub serial_number: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub type_: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            currency: None,
            deregistration_date: None,
            description: None,
            icao_code: None,
            index_text: None,
            keywords: None,
            manufacturer: None,
            model: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            operator: None,
            owner: None,
            previous_name: None,
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            registration_date: None,
            registration_number: None,
            retrieved_at: None,
            serial_number: None,
            source_url: None,
            summary: None,
            topics: None,
            type_: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Airplane"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Article
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Article".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ancestors: Option<Vec<String>>,
    ///Property: Author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub author: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Crawler
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub crawler: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub encoding: Option<Vec<String>>,
    ///Property: File extension
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub extension: Option<Vec<String>>,
    ///Property: File name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub file_name: Vec<String>,
    ///Property: File size
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub file_size: Option<Vec<f64>>,
    ///Property: Generator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub generator: Option<Vec<String>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub message_id: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Processing error
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_error: Option<Vec<String>>,
    ///Property: Processing status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_status: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            author: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            crawler: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            encoding: None,
            extension: None,
            file_name: Vec::new(),
            file_size: None,
            generator: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            keywords: None,
            language: None,
            location_mentioned: None,
            message_id: None,
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
            processing_error: None,
            processing_status: None,
            program: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Asset
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Asset".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub currency: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            currency: None,
            description: None,
            index_text: None,
            keywords: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            previous_name: None,
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Associate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Associate {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Associate".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Associate
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub associate: Vec<String>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Person
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub person: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Relationship
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub relationship: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            publisher: None,
            publisher_url: None,
            record_id: None,
            relationship: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Audio
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Audio {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Audio".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ancestors: Option<Vec<String>>,
    ///Property: Author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub author: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Crawler
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub crawler: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Duration
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub duration: Option<Vec<f64>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub encoding: Option<Vec<String>>,
    ///Property: File extension
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub extension: Option<Vec<String>>,
    ///Property: File name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub file_name: Vec<String>,
    ///Property: File size
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub file_size: Option<Vec<f64>>,
    ///Property: Generator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub generator: Option<Vec<String>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub message_id: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Processing error
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_error: Option<Vec<String>>,
    ///Property: Processing status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_status: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Sampling Rate
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub sampling_rate: Option<Vec<f64>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            author: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            crawler: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            duration: None,
            email_mentioned: None,
            encoding: None,
            extension: None,
            file_name: Vec::new(),
            file_size: None,
            generator: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            keywords: None,
            language: None,
            location_mentioned: None,
            message_id: None,
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
            processing_error: None,
            processing_status: None,
            program: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Bank account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct BankAccount {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "BankAccount".to_string()))]
    pub schema: String,
    ///Property: Account number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub account_number: Option<Vec<String>>,
    ///Property: Account type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub account_type: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Balance
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub balance: Option<Vec<f64>>,
    ///Property: Balance date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub balance_date: Option<Vec<String>>,
    ///Property: Bank
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bank: Option<Vec<String>>,
    ///Property: Bank address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bank_address: Option<Vec<String>>,
    ///Property: Bank name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bank_name: Option<Vec<String>>,
    ///Property: Bank Identifier Code
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bic: Option<Vec<String>>,
    ///Property: Closing date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub closing_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub currency: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: IBAN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Maximum balance
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub max_balance: Option<Vec<f64>>,
    ///Property: Maximum balance date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub max_balance_date: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Opening date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub opening_date: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            account_type: None,
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
            bank_address: None,
            bank_name: None,
            bic: None,
            closing_date: None,
            country: None,
            created_at: None,
            currency: None,
            description: None,
            iban: None,
            index_text: None,
            keywords: None,
            max_balance: None,
            max_balance_date: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            opening_date: None,
            previous_name: None,
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Call
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Call {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Call".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Caller
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub caller: Option<Vec<String>>,
    ///Property: Caller's Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub caller_number: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Duration
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub duration: Option<Vec<f64>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Receiver
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub receiver: Option<Vec<String>>,
    ///Property: Receiver's Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub receiver_number: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            publisher: None,
            publisher_url: None,
            receiver: None,
            receiver_number: None,
            record_id: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Call for tenders
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct CallForTenders {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "CallForTenders".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Name of contracting authority
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authority: Vec<String>,
    ///Property: Contracting authority reference ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authority_reference_id: Option<Vec<String>>,
    ///Property: Award Notice Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub award_notice_date: Option<Vec<String>>,
    ///Property: Contract awarded in Lots
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub awarded_in_lots: Option<Vec<String>>,
    ///Property: Date of awarding
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub awarding_date: Option<Vec<String>>,
    ///Property: CfT unique id
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub call_id: Option<Vec<String>>,
    ///Property: Certification check
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub certification_check: Option<Vec<String>>,
    ///Property: End of clarification period
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub clarification_deadline: Option<Vec<String>>,
    ///Property: Contract notice date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub contract_notice_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: CPV code
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub cpv_code: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Directive
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub directive: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: EU funding
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub eu_funding: Option<Vec<String>>,
    ///Property: Evaluation mechanism
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub evaluation_mechanism: Option<Vec<String>>,
    ///Property: Does this call fall under the scope of GPP?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub falls_under_gppscope: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Call for tenders result
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub involves_outcome: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Lots names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub lots_names: Option<Vec<String>>,
    ///Property: Maximum number of lots
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub maximum_number_of_lots: Option<Vec<f64>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Multiple tenders will be accepted
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub multiple_tenders: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Number of lots
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub number_of_lots: Option<Vec<f64>>,
    ///Property: NUTS code
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub nuts_code: Option<Vec<String>>,
    ///Property: Published on behalf of
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub on_behalf_of: Option<Vec<String>>,
    ///Property: Payment options
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub payment_options: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Procedure
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub procedure: Option<Vec<String>>,
    ///Property: Procurement type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub procurement_type: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Date of publication/invitation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publication_date: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Above or below threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub relation_to_threshold: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Inclusion of e-Auctions
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub reverse_auctions_included: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Submission deadline
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub submission_deadline: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: TED link for published notices
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ted_url: Option<Vec<String>>,
    ///Property: Tenderers
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub tenderers: Option<Vec<String>>,
    ///Property: Tenders for lots
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub tenders_for_lots: Option<Vec<String>>,
    ///Property: Title
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Vec<String>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            awarded_in_lots: None,
            awarding_date: None,
            call_id: None,
            certification_check: None,
            clarification_deadline: None,
            contract_notice_date: None,
            country: None,
            cpv_code: None,
            created_at: None,
            date: None,
            description: None,
            directive: None,
            end_date: None,
            eu_funding: None,
            evaluation_mechanism: None,
            falls_under_gppscope: None,
            index_text: None,
            involves_outcome: None,
            keywords: None,
            lots_names: None,
            maximum_number_of_lots: None,
            modified_at: None,
            multiple_tenders: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            number_of_lots: None,
            nuts_code: None,
            on_behalf_of: None,
            payment_options: None,
            previous_name: None,
            procedure: None,
            procurement_type: None,
            program: None,
            program_id: None,
            proof: None,
            publication_date: None,
            publisher: None,
            publisher_url: None,
            record_id: None,
            relation_to_threshold: None,
            retrieved_at: None,
            reverse_auctions_included: None,
            source_url: None,
            start_date: None,
            submission_deadline: None,
            summary: None,
            ted_url: None,
            tenderers: None,
            tenders_for_lots: None,
            title: Vec::new(),
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Company
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Company".to_string()))]
    pub schema: String,
    ///Property: Abbreviation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub abbreviation: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: BIK
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bik_code: Option<Vec<String>>,
    ///Property: BrightQuery ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bright_query_id: Option<Vec<String>>,
    ///Property: BrightQuery Organization ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bright_query_org_id: Option<Vec<String>>,
    ///Property: Bureau van Dijk ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bvd_id: Option<Vec<String>>,
    ///Property: COD CAEM
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub caem_code: Option<Vec<String>>,
    ///Property: CAGE
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub cage_code: Option<Vec<String>>,
    ///Property: Capital
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub capital: Option<Vec<String>>,
    ///Property: SEC Central Index Key
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub cik_code: Option<Vec<String>>,
    ///Property: Classification
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub classification: Option<Vec<String>>,
    ///Property: COATO / SOATO / OKATO
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub coato_code: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub currency: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Dissolution date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub dissolution_date: Option<Vec<String>>,
    ///Property: DUNS
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub duns_code: Option<Vec<String>>,
    ///Property: E-Mail
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email: Option<Vec<String>>,
    ///Property: Federal tax service code
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub fns_code: Option<Vec<String>>,
    ///Property: FSS
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub fss_code: Option<Vec<String>>,
    ///Property: GIIN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub gii_number: Option<Vec<String>>,
    ///Property: ICIJ ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub icij_id: Option<Vec<String>>,
    ///Property: ID Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub id_number: Option<Vec<String>>,
    ///Property: IMO Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub imo_number: Option<Vec<String>>,
    ///Property: Incorporation date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub incorporation_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: INN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub inn_code: Option<Vec<String>>,
    ///Property: IPO
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ipo_code: Option<Vec<String>>,
    ///Property: IRS Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub irs_code: Option<Vec<String>>,
    ///Property: ISIN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub isin_code: Option<Vec<String>>,
    ///Property: JIB
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub jib_code: Option<Vec<String>>,
    ///Property: Jurisdiction
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub jurisdiction: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: KPP
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub kpp_code: Option<Vec<String>>,
    ///Property: Legal form
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub legal_form: Option<Vec<String>>,
    ///Property: LEI
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub lei_code: Option<Vec<String>>,
    ///Property: License Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub license_number: Option<Vec<String>>,
    ///Property: Country of origin
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub main_country: Option<Vec<String>>,
    ///Property: MBS
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub mbs_code: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: NPI
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub npi_code: Option<Vec<String>>,
    ///Property: OGRN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ogrn_code: Option<Vec<String>>,
    ///Property: OKOPF
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub okopf_code: Option<Vec<String>>,
    ///Property: OKPO
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub okpo_code: Option<Vec<String>>,
    ///Property: OKSM
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub oksm_code: Option<Vec<String>>,
    ///Property: OKVED(2) Classifier
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub okved_code: Option<Vec<String>>,
    ///Property: OpenCorporates URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub opencorporates_url: Option<Vec<String>>,
    ///Property: Parent company
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: PermID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub perm_id: Option<Vec<String>>,
    ///Property: PFR Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub pfr_number: Option<Vec<String>>,
    ///Property: Phone
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Reuters Instrument Code
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ric_code: Option<Vec<String>>,
    ///Property: Sayari Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub sayari_id: Option<Vec<String>>,
    ///Property: Sector
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub sector: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: SWIFT/BIC
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub swift_bic: Option<Vec<String>>,
    ///Property: Tax Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub tax_number: Option<Vec<String>>,
    ///Property: Tax status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub tax_status: Option<Vec<String>>,
    ///Property: Stock ticker symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ticker: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Unique Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub unique_entity_id: Option<Vec<String>>,
    ///Property: USCC
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub usc_code: Option<Vec<String>>,
    ///Property: V.A.T. Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub vat_code: Option<Vec<String>>,
    ///Property: VOEN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub voen_code: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Website
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub website: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            caem_code: None,
            cage_code: None,
            capital: None,
            cik_code: None,
            classification: None,
            coato_code: None,
            country: None,
            created_at: None,
            currency: None,
            description: None,
            dissolution_date: None,
            duns_code: None,
            email: None,
            fns_code: None,
            fss_code: None,
            gii_number: None,
            icij_id: None,
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
            keywords: None,
            kpp_code: None,
            legal_form: None,
            lei_code: None,
            license_number: None,
            main_country: None,
            mbs_code: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            npi_code: None,
            ogrn_code: None,
            okopf_code: None,
            okpo_code: None,
            oksm_code: None,
            okved_code: None,
            opencorporates_url: None,
            parent: None,
            perm_id: None,
            pfr_number: None,
            phone: None,
            previous_name: None,
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            registration_number: None,
            retrieved_at: None,
            ric_code: None,
            sayari_id: None,
            sector: None,
            source_url: None,
            status: None,
            summary: None,
            swift_bic: None,
            tax_number: None,
            tax_status: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Contract
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Contract".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Contract authority
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authority: Option<Vec<String>>,
    ///Property: Cancelled?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub cancelled: Option<Vec<String>>,
    ///Property: Classification
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub classification: Option<Vec<String>>,
    ///Property: Contract date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub contract_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Contract award criteria
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub criteria: Option<Vec<String>>,
    ///Property: Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub currency: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub language: Option<Vec<String>>,
    ///Property: Procurement method
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub method: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Contract Award Notice ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notice_id: Option<Vec<String>>,
    ///Property: Number of awards
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub number_awards: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Contract procedure
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub procedure: Option<Vec<String>>,
    ///Property: Procedure number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub procedure_number: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Project
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub project: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Vec<String>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub type_: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            cancelled: None,
            classification: None,
            contract_date: None,
            country: None,
            created_at: None,
            criteria: None,
            currency: None,
            description: None,
            index_text: None,
            keywords: None,
            language: None,
            method: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            notice_id: None,
            number_awards: None,
            previous_name: None,
            procedure: None,
            procedure_number: None,
            program: None,
            program_id: None,
            project: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            status: None,
            summary: None,
            title: Vec::new(),
            topics: None,
            type_: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Contract"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Contract award
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct ContractAward {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "ContractAward".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Amended
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub amended: Option<Vec<String>>,
    ///Property: Amount
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Call For Tenders
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub call_for_tenders: Option<Vec<String>>,
    ///Property: Contract
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub contract: Vec<String>,
    ///Property: CPV code
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub cpv_code: Option<Vec<String>>,
    ///Property: Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub currency: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Decision reason
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub decision_reason: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Document number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub document_number: Option<Vec<String>>,
    ///Property: Document type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub document_type: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Lot number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub lot_number: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: NUTS code
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub nuts_code: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Role
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub role: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Supplier
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            amended: None,
            amount: None,
            amount_eur: None,
            amount_usd: None,
            call_for_tenders: None,
            contract: Vec::new(),
            cpv_code: None,
            currency: None,
            date: None,
            decision_reason: None,
            description: None,
            document_number: None,
            document_type: None,
            end_date: None,
            index_text: None,
            lot_number: None,
            modified_at: None,
            names_mentioned: None,
            nuts_code: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            record_id: None,
            retrieved_at: None,
            role: None,
            source_url: None,
            start_date: None,
            status: None,
            summary: None,
            supplier: Vec::new(),
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "ContractAward"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Court case
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct CourtCase {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "CourtCase".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Case number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub case_number: Option<Vec<String>>,
    ///Property: Category
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub category: Option<Vec<String>>,
    ///Property: Close date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub close_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Court
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub court: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: File date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub file_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub type_: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            category: None,
            close_date: None,
            country: None,
            court: None,
            created_at: None,
            description: None,
            file_date: None,
            index_text: None,
            keywords: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            previous_name: None,
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            retrieved_at: None,
            source_url: None,
            status: None,
            summary: None,
            topics: None,
            type_: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "CourtCase"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Case party
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct CourtCaseParty {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "CourtCaseParty".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Case
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub case: Vec<String>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Party
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub party: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Role
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub role: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            publisher: None,
            publisher_url: None,
            record_id: None,
            retrieved_at: None,
            role: None,
            source_url: None,
            start_date: None,
            status: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "CourtCaseParty"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Cryptocurrency wallet
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct CryptoWallet {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "CryptoWallet".to_string()))]
    pub schema: String,
    ///Property: Account ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub account_id: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Balance
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub balance: Option<Vec<f64>>,
    ///Property: Balance date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub balance_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Creation date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub creation_date: Option<Vec<String>>,
    ///Property: Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub currency: Option<Vec<String>>,
    ///Property: Currency short code
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub currency_symbol: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Wallet holder
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub holder: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Managing exchange
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub managing_exchange: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Private key
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub private_key: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub public_key: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            currency: None,
            currency_symbol: None,
            description: None,
            holder: None,
            index_text: None,
            keywords: None,
            managing_exchange: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            previous_name: None,
            private_key: None,
            program: None,
            program_id: None,
            proof: None,
            public_key: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Debt
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Debt {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Debt".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Amount
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Creditor
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub creditor: Option<Vec<String>>,
    ///Property: Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub currency: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Debtor
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub debtor: Vec<String>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            currency: None,
            date: None,
            debtor: Vec::new(),
            description: None,
            end_date: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            record_id: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Directorship
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Directorship {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Directorship".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Director
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub director: Vec<String>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Organization
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub organization: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Role
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub role: Option<Vec<String>>,
    ///Property: Secretary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub secretary: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            publisher: None,
            publisher_url: None,
            record_id: None,
            retrieved_at: None,
            role: None,
            secretary: None,
            source_url: None,
            start_date: None,
            status: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Directorship"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: File
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Document".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ancestors: Option<Vec<String>>,
    ///Property: Author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub author: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Crawler
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub crawler: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub encoding: Option<Vec<String>>,
    ///Property: File extension
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub extension: Option<Vec<String>>,
    ///Property: File name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub file_name: Vec<String>,
    ///Property: File size
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub file_size: Option<Vec<f64>>,
    ///Property: Generator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub generator: Option<Vec<String>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub message_id: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Processing error
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_error: Option<Vec<String>>,
    ///Property: Processing status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_status: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            author: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            crawler: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            encoding: None,
            extension: None,
            file_name: Vec::new(),
            file_size: None,
            generator: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            keywords: None,
            language: None,
            location_mentioned: None,
            message_id: None,
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
            processing_error: None,
            processing_status: None,
            program: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Documentation {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Documentation".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Document
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub document: Vec<String>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Entity
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub entity: Vec<String>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Role
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub role: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
}
impl Documentation {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Documentation".to_string(),
            aleph_url: None,
            date: None,
            description: None,
            document: Vec::new(),
            end_date: None,
            entity: Vec::new(),
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            record_id: None,
            retrieved_at: None,
            role: None,
            source_url: None,
            start_date: None,
            status: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Documentation"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Customs declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct EconomicActivity {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "EconomicActivity".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Bank Account
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bank_account: Option<Vec<String>>,
    ///Property: Foreign currency bank
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bank_foreign: Option<Vec<String>>,
    ///Property: Rouble bank
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bank_rub: Option<Vec<String>>,
    ///Property: Customs Cargo Declaration Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ccd_number: Option<Vec<String>>,
    ///Property: CCD Value
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ccd_value: Option<Vec<String>>,
    ///Property: Contract
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub contract: Option<Vec<String>>,
    ///Property: Contract holder
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub contract_holder: Option<Vec<String>>,
    ///Property: Customs Value Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub customs_amount: Option<Vec<String>>,
    ///Property: Customs Procedure
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub customs_procedure: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Declarant
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub declarant: Option<Vec<String>>,
    ///Property: Country of departure
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub departure_country: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Country of destination
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub destination_country: Option<Vec<String>>,
    ///Property: Direction of transportation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub direction_of_transportation: Option<Vec<String>>,
    ///Property: USD Exchange Rate
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub dollar_exch_rate: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Description of goods
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub goods_description: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Invoice Value Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub invoice_amount: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Country of origin
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub origin_country: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Receiver
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub receiver: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Sender
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub sender: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Trading Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub trading_country: Option<Vec<String>>,
    ///Property: Transport
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub transport: Option<Vec<String>>,
    ///Property: FEAC Code
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ved_code: Option<Vec<String>>,
    ///Property: FEAC Code description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ved_code_description: Option<Vec<String>>,
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
            ccd_value: None,
            contract: None,
            contract_holder: None,
            customs_amount: None,
            customs_procedure: None,
            date: None,
            declarant: None,
            departure_country: None,
            description: None,
            destination_country: None,
            direction_of_transportation: None,
            dollar_exch_rate: None,
            end_date: None,
            goods_description: None,
            index_text: None,
            invoice_amount: None,
            modified_at: None,
            names_mentioned: None,
            origin_country: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            receiver: None,
            record_id: None,
            retrieved_at: None,
            sender: None,
            source_url: None,
            start_date: None,
            summary: None,
            trading_country: None,
            transport: None,
            ved_code: None,
            ved_code_description: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "EconomicActivity"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: E-Mail
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Email {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Email".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ancestors: Option<Vec<String>>,
    ///Property: Author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub author: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authored_at: Option<Vec<String>>,
    ///Property: BCC
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bcc: Option<Vec<String>>,
    ///Property: HTML
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_html: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_text: Option<Vec<String>>,
    ///Property: CC
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub cc: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Crawler
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub crawler: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: Emitter
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub emitters: Option<Vec<String>>,
    ///Property: File encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub encoding: Option<Vec<String>>,
    ///Property: File extension
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub extension: Option<Vec<String>>,
    ///Property: File name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub file_name: Vec<String>,
    ///Property: File size
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub file_size: Option<Vec<f64>>,
    ///Property: From
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub from: Option<Vec<String>>,
    ///Property: Generator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub generator: Option<Vec<String>>,
    ///Property: Raw headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: In Reply To
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub in_reply_to: Option<Vec<String>>,
    ///Property: Responding to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub in_reply_to_email: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub message_id: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Processing error
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_error: Option<Vec<String>>,
    ///Property: Processing status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_status: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Recipients
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub recipients: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Sender
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub sender: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Subject
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub subject: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Thread topic
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub thread_topic: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Option<Vec<String>>,
    ///Property: To
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub to: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            author: None,
            authored_at: None,
            bcc: None,
            body_html: None,
            body_text: None,
            cc: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            crawler: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            emitters: None,
            encoding: None,
            extension: None,
            file_name: Vec::new(),
            file_size: None,
            from: None,
            generator: None,
            headers: None,
            iban_mentioned: None,
            in_reply_to: None,
            in_reply_to_email: None,
            index_text: None,
            ip_mentioned: None,
            keywords: None,
            language: None,
            location_mentioned: None,
            message_id: None,
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
            processing_error: None,
            processing_status: None,
            program: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Employment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Employment {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Employment".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Employee
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub employee: Vec<String>,
    ///Property: Employer
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub employer: Vec<String>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Role
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub role: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            publisher: None,
            publisher_url: None,
            record_id: None,
            retrieved_at: None,
            role: None,
            source_url: None,
            start_date: None,
            status: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Employment"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Event".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Important
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub important: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Involved
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub involved: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Location
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Organizer
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub organizer: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            important: None,
            index_text: None,
            involved: None,
            ip_mentioned: None,
            keywords: None,
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
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            record_id: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Family
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Family {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Family".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Person
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub person: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Relationship
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub relationship: Option<Vec<String>>,
    ///Property: Relative
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub relative: Vec<String>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            publisher: None,
            publisher_url: None,
            record_id: None,
            relationship: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Folder
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Folder".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ancestors: Option<Vec<String>>,
    ///Property: Author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub author: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Crawler
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub crawler: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub encoding: Option<Vec<String>>,
    ///Property: File extension
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub extension: Option<Vec<String>>,
    ///Property: File name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub file_name: Vec<String>,
    ///Property: File size
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub file_size: Option<Vec<f64>>,
    ///Property: Generator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub generator: Option<Vec<String>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub message_id: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Processing error
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_error: Option<Vec<String>>,
    ///Property: Processing status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_status: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            author: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            crawler: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            encoding: None,
            extension: None,
            file_name: Vec::new(),
            file_size: None,
            generator: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            keywords: None,
            language: None,
            location_mentioned: None,
            message_id: None,
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
            processing_error: None,
            processing_status: None,
            program: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Web page
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct HyperText {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "HyperText".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ancestors: Option<Vec<String>>,
    ///Property: Author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub author: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authored_at: Option<Vec<String>>,
    ///Property: HTML
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_html: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Crawler
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub crawler: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub encoding: Option<Vec<String>>,
    ///Property: File extension
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub extension: Option<Vec<String>>,
    ///Property: File name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub file_name: Vec<String>,
    ///Property: File size
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub file_size: Option<Vec<f64>>,
    ///Property: Generator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub generator: Option<Vec<String>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub message_id: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Processing error
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_error: Option<Vec<String>>,
    ///Property: Processing status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_status: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            author: None,
            authored_at: None,
            body_html: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            crawler: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            encoding: None,
            extension: None,
            file_name: Vec::new(),
            file_size: None,
            generator: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            keywords: None,
            language: None,
            location_mentioned: None,
            message_id: None,
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
            processing_error: None,
            processing_status: None,
            program: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Identification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Identification {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Identification".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Authority
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authority: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Identification holder
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub holder: Vec<String>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Document number
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub number: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub type_: Option<Vec<String>>,
}
impl Identification {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Identification".to_string(),
            aleph_url: None,
            authority: None,
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
            publisher: None,
            publisher_url: None,
            record_id: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
            type_: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Identification"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Image
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Image".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ancestors: Option<Vec<String>>,
    ///Property: Author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub author: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Crawler
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub crawler: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Credit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub credit: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub encoding: Option<Vec<String>>,
    ///Property: File extension
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub extension: Option<Vec<String>>,
    ///Property: File name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub file_name: Vec<String>,
    ///Property: File size
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub file_size: Option<Vec<f64>>,
    ///Property: Generator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub generator: Option<Vec<String>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub message_id: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Pictured
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub pictured: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Processing error
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_error: Option<Vec<String>>,
    ///Property: Processing status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_status: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            author: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            crawler: None,
            created_at: None,
            credit: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            encoding: None,
            extension: None,
            file_name: Vec::new(),
            file_size: None,
            generator: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            keywords: None,
            language: None,
            location_mentioned: None,
            message_id: None,
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
            processing_error: None,
            processing_status: None,
            program: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Legal entity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct LegalEntity {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "LegalEntity".to_string()))]
    pub schema: String,
    ///Property: Abbreviation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub abbreviation: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: BrightQuery ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bright_query_id: Option<Vec<String>>,
    ///Property: BrightQuery Organization ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bright_query_org_id: Option<Vec<String>>,
    ///Property: Bureau van Dijk ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bvd_id: Option<Vec<String>>,
    ///Property: Classification
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub classification: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Dissolution date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub dissolution_date: Option<Vec<String>>,
    ///Property: DUNS
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub duns_code: Option<Vec<String>>,
    ///Property: E-Mail
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email: Option<Vec<String>>,
    ///Property: ICIJ ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub icij_id: Option<Vec<String>>,
    ///Property: ID Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub id_number: Option<Vec<String>>,
    ///Property: Incorporation date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub incorporation_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: INN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub inn_code: Option<Vec<String>>,
    ///Property: Jurisdiction
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub jurisdiction: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Legal form
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub legal_form: Option<Vec<String>>,
    ///Property: LEI
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub lei_code: Option<Vec<String>>,
    ///Property: License Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub license_number: Option<Vec<String>>,
    ///Property: Country of origin
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub main_country: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: NPI
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub npi_code: Option<Vec<String>>,
    ///Property: OGRN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ogrn_code: Option<Vec<String>>,
    ///Property: OKPO
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub okpo_code: Option<Vec<String>>,
    ///Property: OpenCorporates URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub opencorporates_url: Option<Vec<String>>,
    ///Property: Parent company
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: Phone
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Sayari Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub sayari_id: Option<Vec<String>>,
    ///Property: Sector
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub sector: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: SWIFT/BIC
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub swift_bic: Option<Vec<String>>,
    ///Property: Tax Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub tax_number: Option<Vec<String>>,
    ///Property: Tax status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub tax_status: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Unique Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub unique_entity_id: Option<Vec<String>>,
    ///Property: USCC
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub usc_code: Option<Vec<String>>,
    ///Property: V.A.T. Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub vat_code: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Website
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub website: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            classification: None,
            country: None,
            created_at: None,
            description: None,
            dissolution_date: None,
            duns_code: None,
            email: None,
            icij_id: None,
            id_number: None,
            incorporation_date: None,
            index_text: None,
            inn_code: None,
            jurisdiction: None,
            keywords: None,
            legal_form: None,
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
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            registration_number: None,
            retrieved_at: None,
            sayari_id: None,
            sector: None,
            source_url: None,
            status: None,
            summary: None,
            swift_bic: None,
            tax_number: None,
            tax_status: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: License
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "License".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Area
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub area: Option<Vec<f64>>,
    ///Property: Contract authority
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authority: Vec<String>,
    ///Property: Cancelled?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub cancelled: Option<Vec<String>>,
    ///Property: Classification
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub classification: Option<Vec<String>>,
    ///Property: Commodities
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub commodities: Option<Vec<String>>,
    ///Property: Contract date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub contract_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Contract award criteria
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub criteria: Option<Vec<String>>,
    ///Property: Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub currency: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub language: Option<Vec<String>>,
    ///Property: Procurement method
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub method: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Contract Award Notice ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notice_id: Option<Vec<String>>,
    ///Property: Number of awards
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub number_awards: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Contract procedure
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub procedure: Option<Vec<String>>,
    ///Property: Procedure number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub procedure_number: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Project
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub project: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: License review date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub review_date: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Vec<String>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub type_: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            cancelled: None,
            classification: None,
            commodities: None,
            contract_date: None,
            country: None,
            created_at: None,
            criteria: None,
            currency: None,
            description: None,
            index_text: None,
            keywords: None,
            language: None,
            method: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            notice_id: None,
            number_awards: None,
            previous_name: None,
            procedure: None,
            procedure_number: None,
            program: None,
            program_id: None,
            project: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            retrieved_at: None,
            review_date: None,
            source_url: None,
            status: None,
            summary: None,
            title: Vec::new(),
            topics: None,
            type_: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "License"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Membership
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Membership {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Membership".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Member
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub member: Vec<String>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Organization
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub organization: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Role
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub role: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            publisher: None,
            publisher_url: None,
            record_id: None,
            retrieved_at: None,
            role: None,
            source_url: None,
            start_date: None,
            status: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Membership"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Mention
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Mention".to_string()))]
    pub schema: String,
    ///Property: Co-occurring countries
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub context_country: Option<Vec<String>>,
    ///Property: Co-occurring e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub context_email: Option<Vec<String>>,
    ///Property: Co-occurring phone numbers
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub context_phone: Option<Vec<String>>,
    ///Property: Detected entity type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_schema: Option<Vec<String>>,
    ///Property: Document
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub document: Vec<String>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Entity
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            detected_schema: None,
            document: Vec::new(),
            name: Vec::new(),
            resolved: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Mention"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Message".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ancestors: Option<Vec<String>>,
    ///Property: Author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub author: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authored_at: Option<Vec<String>>,
    ///Property: HTML
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_html: Option<Vec<String>>,
    ///Property: Text
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_text: Vec<String>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Crawler
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub crawler: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub encoding: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: File extension
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub extension: Option<Vec<String>>,
    ///Property: File name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub file_name: Vec<String>,
    ///Property: File size
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub file_size: Option<Vec<f64>>,
    ///Property: Generator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub generator: Option<Vec<String>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: In Reply To
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub in_reply_to: Option<Vec<String>>,
    ///Property: Responding to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub in_reply_to_message: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub message_id: Option<Vec<String>>,
    ///Property: Metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Processing error
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_error: Option<Vec<String>>,
    ///Property: Processing status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_status: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Recipient Account
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub recipient_account: Option<Vec<String>>,
    ///Property: Recipients
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub recipients: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Sender
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub sender: Vec<String>,
    ///Property: Sender Account
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub sender_account: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Subject
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub subject: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Thread topic
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub thread_topic: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            author: None,
            authored_at: None,
            body_html: None,
            body_text: Vec::new(),
            companies_mentioned: None,
            content_hash: None,
            country: None,
            crawler: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            encoding: None,
            end_date: None,
            extension: None,
            file_name: Vec::new(),
            file_size: None,
            generator: None,
            iban_mentioned: None,
            in_reply_to: None,
            in_reply_to_message: None,
            index_text: None,
            ip_mentioned: None,
            keywords: None,
            language: None,
            location_mentioned: None,
            message_id: None,
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
            processing_error: None,
            processing_status: None,
            program: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher: None,
            publisher_url: None,
            recipient_account: None,
            recipients: None,
            record_id: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Note
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Note".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: Entity
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub entity: Option<Vec<String>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            keywords: None,
            location_mentioned: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            people_mentioned: None,
            phone_mentioned: None,
            previous_name: None,
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Occupancy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Occupancy {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Occupancy".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Declaration date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub declaration_date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Holder
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub holder: Vec<String>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Position occupied
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub post: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            publisher: None,
            publisher_url: None,
            record_id: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Organization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Organization".to_string()))]
    pub schema: String,
    ///Property: Abbreviation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub abbreviation: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: BrightQuery ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bright_query_id: Option<Vec<String>>,
    ///Property: BrightQuery Organization ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bright_query_org_id: Option<Vec<String>>,
    ///Property: Bureau van Dijk ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bvd_id: Option<Vec<String>>,
    ///Property: CAGE
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub cage_code: Option<Vec<String>>,
    ///Property: Classification
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub classification: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Dissolution date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub dissolution_date: Option<Vec<String>>,
    ///Property: DUNS
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub duns_code: Option<Vec<String>>,
    ///Property: E-Mail
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email: Option<Vec<String>>,
    ///Property: GIIN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub gii_number: Option<Vec<String>>,
    ///Property: ICIJ ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub icij_id: Option<Vec<String>>,
    ///Property: ID Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub id_number: Option<Vec<String>>,
    ///Property: IMO Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub imo_number: Option<Vec<String>>,
    ///Property: Incorporation date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub incorporation_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: INN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub inn_code: Option<Vec<String>>,
    ///Property: Jurisdiction
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub jurisdiction: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Legal form
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub legal_form: Option<Vec<String>>,
    ///Property: LEI
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub lei_code: Option<Vec<String>>,
    ///Property: License Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub license_number: Option<Vec<String>>,
    ///Property: Country of origin
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub main_country: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: NPI
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub npi_code: Option<Vec<String>>,
    ///Property: OGRN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ogrn_code: Option<Vec<String>>,
    ///Property: OKPO
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub okpo_code: Option<Vec<String>>,
    ///Property: OpenCorporates URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub opencorporates_url: Option<Vec<String>>,
    ///Property: Parent company
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: PermID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub perm_id: Option<Vec<String>>,
    ///Property: Phone
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Sayari Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub sayari_id: Option<Vec<String>>,
    ///Property: Sector
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub sector: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: SWIFT/BIC
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub swift_bic: Option<Vec<String>>,
    ///Property: Tax Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub tax_number: Option<Vec<String>>,
    ///Property: Tax status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub tax_status: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Unique Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub unique_entity_id: Option<Vec<String>>,
    ///Property: USCC
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub usc_code: Option<Vec<String>>,
    ///Property: V.A.T. Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub vat_code: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Website
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub website: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            classification: None,
            country: None,
            created_at: None,
            description: None,
            dissolution_date: None,
            duns_code: None,
            email: None,
            gii_number: None,
            icij_id: None,
            id_number: None,
            imo_number: None,
            incorporation_date: None,
            index_text: None,
            inn_code: None,
            jurisdiction: None,
            keywords: None,
            legal_form: None,
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
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            registration_number: None,
            retrieved_at: None,
            sayari_id: None,
            sector: None,
            source_url: None,
            status: None,
            summary: None,
            swift_bic: None,
            tax_number: None,
            tax_status: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Ownership
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Ownership {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Ownership".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Asset
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub asset: Vec<String>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Legal basis
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub legal_basis: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Owner
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub owner: Vec<String>,
    ///Property: Type of ownership
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ownership_type: Option<Vec<String>>,
    ///Property: Percentage held
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub percentage: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Role
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub role: Option<Vec<String>>,
    ///Property: Number of shares
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub shares_count: Option<Vec<String>>,
    ///Property: Currency of shares
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub shares_currency: Option<Vec<String>>,
    ///Property: Type of shares
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub shares_type: Option<Vec<String>>,
    ///Property: Value of shares
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub shares_value: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            legal_basis: None,
            modified_at: None,
            names_mentioned: None,
            owner: Vec::new(),
            ownership_type: None,
            percentage: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            record_id: None,
            retrieved_at: None,
            role: None,
            shares_count: None,
            shares_currency: None,
            shares_type: None,
            shares_value: None,
            source_url: None,
            start_date: None,
            status: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Ownership"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Package
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Package".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ancestors: Option<Vec<String>>,
    ///Property: Author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub author: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Crawler
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub crawler: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub encoding: Option<Vec<String>>,
    ///Property: File extension
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub extension: Option<Vec<String>>,
    ///Property: File name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub file_name: Vec<String>,
    ///Property: File size
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub file_size: Option<Vec<f64>>,
    ///Property: Generator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub generator: Option<Vec<String>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub message_id: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Processing error
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_error: Option<Vec<String>>,
    ///Property: Processing status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_status: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            author: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            crawler: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            encoding: None,
            extension: None,
            file_name: Vec::new(),
            file_size: None,
            generator: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            keywords: None,
            language: None,
            location_mentioned: None,
            message_id: None,
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
            processing_error: None,
            processing_status: None,
            program: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Page
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Page".to_string()))]
    pub schema: String,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub document: Option<Vec<String>>,
    ///Property: Index
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub index: Option<Vec<f64>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_text: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_text_language: Option<Vec<String>>,
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
            translated_text_language: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Page"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Document
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Pages {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Pages".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ancestors: Option<Vec<String>>,
    ///Property: Author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub author: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Crawler
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub crawler: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub encoding: Option<Vec<String>>,
    ///Property: File extension
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub extension: Option<Vec<String>>,
    ///Property: File name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub file_name: Vec<String>,
    ///Property: File size
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub file_size: Option<Vec<f64>>,
    ///Property: Generator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub generator: Option<Vec<String>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub message_id: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: PDF alternative version checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub pdf_hash: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Processing error
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_error: Option<Vec<String>>,
    ///Property: Processing status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_status: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            author: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            crawler: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            encoding: None,
            extension: None,
            file_name: Vec::new(),
            file_size: None,
            generator: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            keywords: None,
            language: None,
            location_mentioned: None,
            message_id: None,
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
            processing_error: None,
            processing_status: None,
            program: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Passport
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Passport {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Passport".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Authority
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authority: Option<Vec<String>>,
    ///Property: Birth date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub birth_date: Option<Vec<String>>,
    ///Property: Place of birth
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub birth_place: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Gender
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub gender: Option<Vec<String>>,
    ///Property: Given name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub given_name: Option<Vec<String>>,
    ///Property: Identification holder
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub holder: Vec<String>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Document number
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub number: Vec<String>,
    ///Property: Passport number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub passport_number: Option<Vec<String>>,
    ///Property: Personal number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub personal_number: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Surname
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub surname: Option<Vec<String>>,
    ///Property: Type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub type_: Option<Vec<String>>,
}
impl Passport {
    /// Create a new entity with the given ID
    #[deprecated(note = "Use the builder() method instead to ensure required fields are set")]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema: "Passport".to_string(),
            aleph_url: None,
            authority: None,
            birth_date: None,
            birth_place: None,
            country: None,
            date: None,
            description: None,
            end_date: None,
            gender: None,
            given_name: None,
            holder: Vec::new(),
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            number: Vec::new(),
            passport_number: None,
            personal_number: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            record_id: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
            surname: None,
            type_: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Passport"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Payment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Payment".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Amount
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Beneficiary
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub beneficiary: Vec<String>,
    ///Property: Beneficiary bank account
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub beneficiary_account: Option<Vec<String>>,
    ///Property: Contract
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub contract: Option<Vec<String>>,
    ///Property: Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub currency: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Payer
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub payer: Vec<String>,
    ///Property: Payer bank account
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub payer_account: Option<Vec<String>>,
    ///Property: Payment programme
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub programme: Option<Vec<String>>,
    ///Property: Project
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub project: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Payment purpose
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub purpose: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Sequence number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub sequence_number: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Transaction number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub transaction_number: Option<Vec<String>>,
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
            currency: None,
            date: None,
            description: None,
            end_date: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            payer: Vec::new(),
            payer_account: None,
            programme: None,
            project: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            purpose: None,
            record_id: None,
            retrieved_at: None,
            sequence_number: None,
            source_url: None,
            start_date: None,
            summary: None,
            transaction_number: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Payment"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Person
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Person".to_string()))]
    pub schema: String,
    ///Property: Abbreviation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub abbreviation: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Physical appearance
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub appearance: Option<Vec<String>>,
    ///Property: Country of birth
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub birth_country: Option<Vec<String>>,
    ///Property: Birth date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub birth_date: Option<Vec<String>>,
    ///Property: Place of birth
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub birth_place: Option<Vec<String>>,
    ///Property: BrightQuery ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bright_query_id: Option<Vec<String>>,
    ///Property: BrightQuery Organization ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bright_query_org_id: Option<Vec<String>>,
    ///Property: Bureau van Dijk ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bvd_id: Option<Vec<String>>,
    ///Property: Citizenship
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub citizenship: Option<Vec<String>>,
    ///Property: Classification
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub classification: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Death date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub death_date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Dissolution date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub dissolution_date: Option<Vec<String>>,
    ///Property: DUNS
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub duns_code: Option<Vec<String>>,
    ///Property: Education
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub education: Option<Vec<String>>,
    ///Property: E-Mail
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email: Option<Vec<String>>,
    ///Property: Ethnicity
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ethnicity: Option<Vec<String>>,
    ///Property: Eye color
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub eye_color: Option<Vec<String>>,
    ///Property: Patronymic
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub father_name: Option<Vec<String>>,
    ///Property: First name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub first_name: Option<Vec<String>>,
    ///Property: Gender
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub gender: Option<Vec<String>>,
    ///Property: Hair color
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub hair_color: Option<Vec<String>>,
    ///Property: Height
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub height: Option<Vec<f64>>,
    ///Property: ICIJ ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub icij_id: Option<Vec<String>>,
    ///Property: ID Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub id_number: Option<Vec<String>>,
    ///Property: Incorporation date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub incorporation_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: INN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub inn_code: Option<Vec<String>>,
    ///Property: Jurisdiction
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub jurisdiction: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Last name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub last_name: Option<Vec<String>>,
    ///Property: Legal form
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub legal_form: Option<Vec<String>>,
    ///Property: LEI
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub lei_code: Option<Vec<String>>,
    ///Property: License Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub license_number: Option<Vec<String>>,
    ///Property: Country of origin
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub main_country: Option<Vec<String>>,
    ///Property: Middle name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub middle_name: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Matronymic
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub mother_name: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Name suffix
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name_suffix: Option<Vec<String>>,
    ///Property: Nationality
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub nationality: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: NPI
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub npi_code: Option<Vec<String>>,
    ///Property: OGRN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ogrn_code: Option<Vec<String>>,
    ///Property: OKPO
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub okpo_code: Option<Vec<String>>,
    ///Property: OpenCorporates URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub opencorporates_url: Option<Vec<String>>,
    ///Property: Parent company
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: Passport number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub passport_number: Option<Vec<String>>,
    ///Property: Phone
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone: Option<Vec<String>>,
    ///Property: Political association
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub political: Option<Vec<String>>,
    ///Property: Position
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub position: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Profession
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub profession: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub registration_number: Option<Vec<String>>,
    ///Property: Religion
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub religion: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Sayari Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub sayari_id: Option<Vec<String>>,
    ///Property: Second name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub second_name: Option<Vec<String>>,
    ///Property: Sector
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub sector: Option<Vec<String>>,
    ///Property: Social security number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub social_security_number: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Spoken language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub spoken_language: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: SWIFT/BIC
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub swift_bic: Option<Vec<String>>,
    ///Property: Tax Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub tax_number: Option<Vec<String>>,
    ///Property: Tax status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub tax_status: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Unique Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub unique_entity_id: Option<Vec<String>>,
    ///Property: USCC
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub usc_code: Option<Vec<String>>,
    ///Property: V.A.T. Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub vat_code: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Website
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub website: Option<Vec<String>>,
    ///Property: Weight
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub weight: Option<Vec<f64>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            appearance: None,
            birth_country: None,
            birth_date: None,
            birth_place: None,
            bright_query_id: None,
            bright_query_org_id: None,
            bvd_id: None,
            citizenship: None,
            classification: None,
            country: None,
            created_at: None,
            death_date: None,
            description: None,
            dissolution_date: None,
            duns_code: None,
            education: None,
            email: None,
            ethnicity: None,
            eye_color: None,
            father_name: None,
            first_name: None,
            gender: None,
            hair_color: None,
            height: None,
            icij_id: None,
            id_number: None,
            incorporation_date: None,
            index_text: None,
            inn_code: None,
            jurisdiction: None,
            keywords: None,
            last_name: None,
            legal_form: None,
            lei_code: None,
            license_number: None,
            main_country: None,
            middle_name: None,
            modified_at: None,
            mother_name: None,
            name: Vec::new(),
            name_suffix: None,
            nationality: None,
            notes: None,
            npi_code: None,
            ogrn_code: None,
            okpo_code: None,
            opencorporates_url: None,
            parent: None,
            passport_number: None,
            phone: None,
            political: None,
            position: None,
            previous_name: None,
            profession: None,
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            registration_number: None,
            religion: None,
            retrieved_at: None,
            sayari_id: None,
            second_name: None,
            sector: None,
            social_security_number: None,
            source_url: None,
            spoken_language: None,
            status: None,
            summary: None,
            swift_bic: None,
            tax_number: None,
            tax_status: None,
            title: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Text file
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct PlainText {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "PlainText".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ancestors: Option<Vec<String>>,
    ///Property: Author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub author: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Crawler
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub crawler: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub encoding: Option<Vec<String>>,
    ///Property: File extension
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub extension: Option<Vec<String>>,
    ///Property: File name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub file_name: Vec<String>,
    ///Property: File size
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub file_size: Option<Vec<f64>>,
    ///Property: Generator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub generator: Option<Vec<String>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub message_id: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Processing error
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_error: Option<Vec<String>>,
    ///Property: Processing status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_status: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            author: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            crawler: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            encoding: None,
            extension: None,
            file_name: Vec::new(),
            file_size: None,
            generator: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            keywords: None,
            language: None,
            location_mentioned: None,
            message_id: None,
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
            processing_error: None,
            processing_status: None,
            program: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Position
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Position".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Dissolution date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub dissolution_date: Option<Vec<String>>,
    ///Property: Inception date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub inception_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Total number of seats
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub number_of_seats: Option<Vec<f64>>,
    ///Property: Organization
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub organization: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Subnational jurisdiction name or code
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub subnational_area: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            keywords: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            number_of_seats: None,
            organization: None,
            previous_name: None,
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Project
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Project".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub currency: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Project goal
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub goal: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Phase
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phase: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Project ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub project_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            currency: None,
            date: None,
            description: None,
            end_date: None,
            goal: None,
            index_text: None,
            keywords: None,
            modified_at: None,
            name: Vec::new(),
            names_mentioned: None,
            notes: None,
            phase: None,
            previous_name: None,
            program: None,
            program_id: None,
            project_id: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            record_id: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            status: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Project participant
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct ProjectParticipant {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "ProjectParticipant".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Participant
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub participant: Option<Vec<String>>,
    ///Property: Project
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub project: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Role
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub role: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            publisher: None,
            publisher_url: None,
            record_id: None,
            retrieved_at: None,
            role: None,
            source_url: None,
            start_date: None,
            status: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "ProjectParticipant"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Public body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct PublicBody {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "PublicBody".to_string()))]
    pub schema: String,
    ///Property: Abbreviation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub abbreviation: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: BrightQuery ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bright_query_id: Option<Vec<String>>,
    ///Property: BrightQuery Organization ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bright_query_org_id: Option<Vec<String>>,
    ///Property: Bureau van Dijk ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub bvd_id: Option<Vec<String>>,
    ///Property: CAGE
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub cage_code: Option<Vec<String>>,
    ///Property: Classification
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub classification: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Dissolution date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub dissolution_date: Option<Vec<String>>,
    ///Property: DUNS
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub duns_code: Option<Vec<String>>,
    ///Property: E-Mail
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email: Option<Vec<String>>,
    ///Property: GIIN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub gii_number: Option<Vec<String>>,
    ///Property: ICIJ ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub icij_id: Option<Vec<String>>,
    ///Property: ID Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub id_number: Option<Vec<String>>,
    ///Property: IMO Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub imo_number: Option<Vec<String>>,
    ///Property: Incorporation date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub incorporation_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: INN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub inn_code: Option<Vec<String>>,
    ///Property: Jurisdiction
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub jurisdiction: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Legal form
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub legal_form: Option<Vec<String>>,
    ///Property: LEI
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub lei_code: Option<Vec<String>>,
    ///Property: License Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub license_number: Option<Vec<String>>,
    ///Property: Country of origin
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub main_country: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: NPI
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub npi_code: Option<Vec<String>>,
    ///Property: OGRN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ogrn_code: Option<Vec<String>>,
    ///Property: OKPO
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub okpo_code: Option<Vec<String>>,
    ///Property: OpenCorporates URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub opencorporates_url: Option<Vec<String>>,
    ///Property: Parent company
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: PermID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub perm_id: Option<Vec<String>>,
    ///Property: Phone
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Sayari Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub sayari_id: Option<Vec<String>>,
    ///Property: Sector
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub sector: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: SWIFT/BIC
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub swift_bic: Option<Vec<String>>,
    ///Property: Tax Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub tax_number: Option<Vec<String>>,
    ///Property: Tax status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub tax_status: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Unique Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub unique_entity_id: Option<Vec<String>>,
    ///Property: USCC
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub usc_code: Option<Vec<String>>,
    ///Property: V.A.T. Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub vat_code: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Website
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub website: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            classification: None,
            country: None,
            created_at: None,
            description: None,
            dissolution_date: None,
            duns_code: None,
            email: None,
            gii_number: None,
            icij_id: None,
            id_number: None,
            imo_number: None,
            incorporation_date: None,
            index_text: None,
            inn_code: None,
            jurisdiction: None,
            keywords: None,
            legal_form: None,
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
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            registration_number: None,
            retrieved_at: None,
            sayari_id: None,
            sector: None,
            source_url: None,
            status: None,
            summary: None,
            swift_bic: None,
            tax_number: None,
            tax_status: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Real estate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct RealEstate {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "RealEstate".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Area
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub area: Option<Vec<f64>>,
    ///Property: Cadastral code
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub cadastral_code: Option<Vec<String>>,
    ///Property: Census block
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub census_block: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Record date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub create_date: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub currency: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Encumbrance
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub encumbrance: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Land type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub land_type: Option<Vec<String>>,
    ///Property: Latitude
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub latitude: Option<Vec<f64>>,
    ///Property: Longitude
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub longitude: Option<Vec<f64>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Parent unit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Property type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub property_type: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Tenure
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub tenure: Option<Vec<String>>,
    ///Property: Title number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title_number: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            census_block: None,
            country: None,
            create_date: None,
            created_at: None,
            currency: None,
            description: None,
            encumbrance: None,
            index_text: None,
            keywords: None,
            land_type: None,
            latitude: None,
            longitude: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            parent: None,
            previous_name: None,
            program: None,
            program_id: None,
            proof: None,
            property_type: None,
            publisher: None,
            publisher_url: None,
            registration_number: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            tenure: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Representation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Representation {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Representation".to_string()))]
    pub schema: String,
    ///Property: Agent
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub agent: Vec<String>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Client
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub client: Vec<String>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Role
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub role: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            publisher: None,
            publisher_url: None,
            record_id: None,
            retrieved_at: None,
            role: None,
            source_url: None,
            start_date: None,
            status: None,
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Representation"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Risk
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Risk {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Risk".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Duration
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub duration: Option<Vec<f64>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Entity
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub entity: Vec<String>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Listing date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub listing_date: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Reason
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub reason: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            publisher: None,
            publisher_url: None,
            reason: None,
            record_id: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            status: None,
            summary: None,
            topics: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Risk"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Sanction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Sanction {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Sanction".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Authority
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authority: Option<Vec<String>>,
    ///Property: Authority-issued identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authority_id: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Duration
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub duration: Option<Vec<f64>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Entity
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub entity: Vec<String>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Listing date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub listing_date: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Program URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_url: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Scope of sanctions
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub provisions: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Reason
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub reason: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: UN SC identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            authority: None,
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
            program: None,
            program_id: None,
            program_url: None,
            proof: None,
            provisions: None,
            publisher: None,
            publisher_url: None,
            reason: None,
            record_id: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            status: None,
            summary: None,
            unsc_id: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Sanction"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Security
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Security {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Security".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Classification
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub classification: Option<Vec<String>>,
    ///Property: Collateral
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub collateral: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub currency: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Financial Instrument Global Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub figi_code: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: ISIN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub isin: Option<Vec<String>>,
    ///Property: Date issued
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub issue_date: Option<Vec<String>>,
    ///Property: Issuer
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub issuer: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Maturity date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub maturity_date: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Stock ticker symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ticker: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub type_: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            classification: None,
            collateral: None,
            country: None,
            created_at: None,
            currency: None,
            description: None,
            figi_code: None,
            index_text: None,
            isin: None,
            issue_date: None,
            issuer: None,
            keywords: None,
            maturity_date: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            previous_name: None,
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            registration_number: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            ticker: None,
            topics: None,
            type_: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Security"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Similar
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Similar {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Similar".to_string()))]
    pub schema: String,
    ///Property: Candidate
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub candidate: Option<Vec<String>>,
    ///Property: Confidence score
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub confidence_score: Option<Vec<f64>>,
    ///Property: Matching criteria
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub criteria: Option<Vec<String>>,
    ///Property: Match
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub match_: Option<Vec<String>>,
    ///Property: Matcher
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub matcher: Option<Vec<String>>,
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
            criteria: None,
            match_: None,
            matcher: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Similar"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Succession
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Succession {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Succession".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Predecessor
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub predecessor: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Role
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub role: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Successor
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub successor: Vec<String>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            publisher: None,
            publisher_url: None,
            record_id: None,
            retrieved_at: None,
            role: None,
            source_url: None,
            start_date: None,
            status: None,
            successor: Vec::new(),
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Succession"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Table".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ancestors: Option<Vec<String>>,
    ///Property: Author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub author: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_text: Option<Vec<String>>,
    ///Property: Column headings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<serde_json::Value>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Crawler
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub crawler: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: CSV alternative version checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub csv_hash: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub encoding: Option<Vec<String>>,
    ///Property: File extension
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub extension: Option<Vec<String>>,
    ///Property: File name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub file_name: Vec<String>,
    ///Property: File size
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub file_size: Option<Vec<f64>>,
    ///Property: Generator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub generator: Option<Vec<String>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub message_id: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Processing error
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_error: Option<Vec<String>>,
    ///Property: Processing status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_status: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Number of rows
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub row_count: Option<Vec<f64>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            author: None,
            authored_at: None,
            body_text: None,
            columns: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            crawler: None,
            created_at: None,
            csv_hash: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            encoding: None,
            extension: None,
            file_name: Vec::new(),
            file_size: None,
            generator: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            keywords: None,
            language: None,
            location_mentioned: None,
            message_id: None,
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
            processing_error: None,
            processing_status: None,
            program: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Tax roll
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct TaxRoll {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "TaxRoll".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Birth date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub birth_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Given name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub given_name: Option<Vec<String>>,
    ///Property: Registered income
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub income: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Surname
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub surname: Option<Vec<String>>,
    ///Property: Amount of tax paid
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub tax_paid: Option<Vec<String>>,
    ///Property: Taxee
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub taxee: Vec<String>,
    ///Property: Registered wealth
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wealth: Option<Vec<String>>,
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
            given_name: None,
            income: None,
            index_text: None,
            modified_at: None,
            names_mentioned: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            record_id: None,
            retrieved_at: None,
            source_url: None,
            start_date: None,
            summary: None,
            surname: None,
            tax_paid: None,
            taxee: Vec::new(),
            wealth: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "TaxRoll"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Trip
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Trip {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Trip".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: End location
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_location: Vec<String>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Important
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub important: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Involved
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub involved: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Location
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Organizer
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub organizer: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Start location
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_location: Vec<String>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Vehicle
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub vehicle: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            important: None,
            index_text: None,
            involved: None,
            ip_mentioned: None,
            keywords: None,
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
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            record_id: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Other link
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct UnknownLink {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "UnknownLink".to_string()))]
    pub schema: String,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: End date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub end_date: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Object
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub object: Vec<String>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub record_id: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Role
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub role: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Start date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub start_date: Option<Vec<String>>,
    ///Property: Status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub status: Option<Vec<String>>,
    ///Property: Subject
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub subject: Vec<String>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            publisher: None,
            publisher_url: None,
            record_id: None,
            retrieved_at: None,
            role: None,
            source_url: None,
            start_date: None,
            status: None,
            subject: Vec::new(),
            summary: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "UnknownLink"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: User account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct UserAccount {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "UserAccount".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: E-Mail
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_address: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Owner
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub owner: Option<Vec<String>>,
    ///Property: Password
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub password: Option<Vec<String>>,
    ///Property: Phone
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Service
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub service: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Username
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub username: Vec<String>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            keywords: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            owner: None,
            password: None,
            phone: None,
            previous_name: None,
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Vehicle
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Vehicle {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Vehicle".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Build Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub build_date: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub currency: Option<Vec<String>>,
    ///Property: De-registration Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub deregistration_date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Model
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub model: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Operator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub operator: Option<Vec<String>>,
    ///Property: Owner
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub owner: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub registration_date: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub registration_number: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub type_: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            currency: None,
            deregistration_date: None,
            description: None,
            index_text: None,
            keywords: None,
            model: None,
            modified_at: None,
            name: Vec::new(),
            notes: None,
            operator: None,
            owner: None,
            previous_name: None,
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            registration_date: None,
            registration_number: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            topics: None,
            type_: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Vehicle"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Vessel
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Vessel {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Vessel".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Amount
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount: Option<Vec<f64>>,
    ///Property: Amount in EUR
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_eur: Option<Vec<f64>>,
    ///Property: Amount in USD
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub amount_usd: Option<Vec<f64>>,
    ///Property: Build Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub build_date: Option<Vec<String>>,
    ///Property: Call Sign
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub call_sign: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: CRS Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub crs_number: Option<Vec<String>>,
    ///Property: Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub currency: Option<Vec<String>>,
    ///Property: Deadweight Tonnage
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub deadweight_tonnage: Option<Vec<f64>>,
    ///Property: De-registration Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub deregistration_date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Flag
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub flag: Option<Vec<String>>,
    ///Property: Gross Registered Tonnage
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub gross_registered_tonnage: Option<Vec<f64>>,
    ///Property: IMO Number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub imo_number: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: MMSI
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub mmsi: Option<Vec<String>>,
    ///Property: Model
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub model: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Date of Name Change
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name_change_date: Option<Vec<String>>,
    ///Property: Navigation Area
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub navigation_area: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Operator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub operator: Option<Vec<String>>,
    ///Property: Owner
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub owner: Option<Vec<String>>,
    ///Property: Past Flags
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub past_flags: Option<Vec<String>>,
    ///Property: Past Types
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub past_types: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Registration Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub registration_date: Option<Vec<String>>,
    ///Property: Registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub registration_number: Option<Vec<String>>,
    ///Property: Port of Registration
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub registration_port: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Tonnage
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub tonnage: Option<Vec<f64>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: Type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub type_: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            currency: None,
            deadweight_tonnage: None,
            deregistration_date: None,
            description: None,
            flag: None,
            gross_registered_tonnage: None,
            imo_number: None,
            index_text: None,
            keywords: None,
            mmsi: None,
            model: None,
            modified_at: None,
            name: Vec::new(),
            name_change_date: None,
            navigation_area: None,
            notes: None,
            operator: None,
            owner: None,
            past_flags: None,
            past_types: None,
            previous_name: None,
            program: None,
            program_id: None,
            proof: None,
            publisher: None,
            publisher_url: None,
            registration_date: None,
            registration_number: None,
            registration_port: None,
            retrieved_at: None,
            source_url: None,
            summary: None,
            tonnage: None,
            topics: None,
            type_: None,
            weak_alias: None,
            wikidata_id: None,
            wikipedia_url: None,
        }
    }
    /// Get the schema name
    pub fn schema_name() -> &'static str {
        "Vessel"
    }
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Video
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Video".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ancestors: Option<Vec<String>>,
    ///Property: Author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub author: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Crawler
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub crawler: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Duration
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub duration: Option<Vec<f64>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub encoding: Option<Vec<String>>,
    ///Property: File extension
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub extension: Option<Vec<String>>,
    ///Property: File name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub file_name: Vec<String>,
    ///Property: File size
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub file_size: Option<Vec<f64>>,
    ///Property: Generator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub generator: Option<Vec<String>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub message_id: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Processing error
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_error: Option<Vec<String>>,
    ///Property: Processing status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_status: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            author: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            crawler: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            duration: None,
            email_mentioned: None,
            encoding: None,
            extension: None,
            file_name: Vec::new(),
            file_size: None,
            generator: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            keywords: None,
            language: None,
            location_mentioned: None,
            message_id: None,
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
            processing_error: None,
            processing_status: None,
            program: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
///FTM Schema: Workbook
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[serde(rename_all = "camelCase")]
pub struct Workbook {
    pub id: String,
    #[cfg_attr(feature = "builder", builder(default = "Workbook".to_string()))]
    pub schema: String,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address: Option<Vec<String>>,
    ///Property: Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub address_entity: Option<Vec<String>>,
    ///Property: Aleph URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub aleph_url: Option<Vec<String>>,
    ///Property: Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub alias: Option<Vec<String>>,
    ///Property: Ancestors
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ancestors: Option<Vec<String>>,
    ///Property: Author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub author: Option<Vec<String>>,
    ///Property: Authored on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub authored_at: Option<Vec<String>>,
    ///Property: Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub body_text: Option<Vec<String>>,
    ///Property: Detected companies
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub companies_mentioned: Option<Vec<String>>,
    ///Property: Checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub content_hash: Option<Vec<String>>,
    ///Property: Country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub country: Option<Vec<String>>,
    ///Property: Crawler
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub crawler: Option<Vec<String>>,
    ///Property: Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub created_at: Option<Vec<String>>,
    ///Property: Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub date: Option<Vec<String>>,
    ///Property: Description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub description: Option<Vec<String>>,
    ///Property: Detected country
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_country: Option<Vec<String>>,
    ///Property: Detected language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub detected_language: Option<Vec<String>>,
    ///Property: Detected e-mail addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub email_mentioned: Option<Vec<String>>,
    ///Property: File encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub encoding: Option<Vec<String>>,
    ///Property: File extension
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub extension: Option<Vec<String>>,
    ///Property: File name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub file_name: Vec<String>,
    ///Property: File size
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_f64_vec",
        default
    )]
    #[cfg_attr(feature = "builder", builder(with = |value:f64|vec![value]))]
    pub file_size: Option<Vec<f64>>,
    ///Property: Generator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub generator: Option<Vec<String>>,
    ///Property: Detected IBANs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub iban_mentioned: Option<Vec<String>>,
    ///Property: Index text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub index_text: Option<Vec<String>>,
    ///Property: Detected IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub ip_mentioned: Option<Vec<String>>,
    ///Property: Keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub keywords: Option<Vec<String>>,
    ///Property: Language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub language: Option<Vec<String>>,
    ///Property: Detected locations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub location_mentioned: Option<Vec<String>>,
    ///Property: Message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub message_id: Option<Vec<String>>,
    ///Property: MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub mime_type: Option<Vec<String>>,
    ///Property: Modified on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub modified_at: Option<Vec<String>>,
    ///Property: Name
    #[serde(default)]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub name: Vec<String>,
    ///Property: Detected names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub names_mentioned: Option<Vec<String>>,
    ///Property: Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub notes: Option<Vec<String>>,
    ///Property: Folder
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub parent: Option<Vec<String>>,
    ///Property: Detected people
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub people_mentioned: Option<Vec<String>>,
    ///Property: Detected phones
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub phone_mentioned: Option<Vec<String>>,
    ///Property: Previous name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub previous_name: Option<Vec<String>>,
    ///Property: Processed at
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processed_at: Option<Vec<String>>,
    ///Property: Processing agent
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_agent: Option<Vec<String>>,
    ///Property: Processing error
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_error: Option<Vec<String>>,
    ///Property: Processing status
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub processing_status: Option<Vec<String>>,
    ///Property: Program
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program: Option<Vec<String>>,
    ///Property: Program ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub program_id: Option<Vec<String>>,
    ///Property: Source document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub proof: Option<Vec<String>>,
    ///Property: Published on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub published_at: Option<Vec<String>>,
    ///Property: Publishing source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher: Option<Vec<String>>,
    ///Property: Publishing source URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub publisher_url: Option<Vec<String>>,
    ///Property: Retrieved on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub retrieved_at: Option<Vec<String>>,
    ///Property: Source link
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub source_url: Option<Vec<String>>,
    ///Property: Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub summary: Option<Vec<String>>,
    ///Property: Title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub title: Option<Vec<String>>,
    ///Property: Topics
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub topics: Option<Vec<String>>,
    ///Property: The language of the translated text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_language: Option<Vec<String>>,
    ///Property: Translated version of the body text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub translated_text: Option<Vec<String>>,
    ///Property: Weak alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub weak_alias: Option<Vec<String>>,
    ///Property: Wikidata ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
    pub wikidata_id: Option<Vec<String>>,
    ///Property: Wikipedia Article
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "builder",
        builder(with = |value:impl Into<String>|vec![value.into()])
    )]
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
            author: None,
            authored_at: None,
            body_text: None,
            companies_mentioned: None,
            content_hash: None,
            country: None,
            crawler: None,
            created_at: None,
            date: None,
            description: None,
            detected_country: None,
            detected_language: None,
            email_mentioned: None,
            encoding: None,
            extension: None,
            file_name: Vec::new(),
            file_size: None,
            generator: None,
            iban_mentioned: None,
            index_text: None,
            ip_mentioned: None,
            keywords: None,
            language: None,
            location_mentioned: None,
            message_id: None,
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
            processing_error: None,
            processing_status: None,
            program: None,
            program_id: None,
            proof: None,
            published_at: None,
            publisher: None,
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
    /// Serialize to standard FTM nested JSON format
    ///
    /// Produces `{"id": "...", "schema": "...", "properties": {...}}`
    pub fn to_ftm_json(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(obj) = value.as_object_mut() {
            let id = obj.remove("id");
            let schema = obj.remove("schema");
            let properties = serde_json::Value::Object(std::mem::take(obj));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            if let Some(schema) = schema {
                obj.insert("schema".into(), schema);
            }
            obj.insert("properties".into(), properties);
        }
        serde_json::to_string(&value)
    }
}
