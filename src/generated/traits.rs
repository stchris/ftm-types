#![allow(missing_docs)]
/// Traits representing FTM schema inheritance hierarchy.
///
/// These traits enable polymorphic code that works across entity types.
/// All concrete entity structs implement the traits for their parent schemas.
///Trait for FTM schema: Analyzable
pub trait Analyzable {
    /// Get the entity ID
    fn id(&self) -> &str;
    /// Get the schema name
    fn schema(&self) -> &str;
    ///Get Detected companies property
    fn companies_mentioned(&self) -> Option<&[String]>;
    ///Get Detected country property
    fn detected_country(&self) -> Option<&[String]>;
    ///Get Detected language property
    fn detected_language(&self) -> Option<&[String]>;
    ///Get Detected e-mail addresses property
    fn email_mentioned(&self) -> Option<&[String]>;
    ///Get Detected IBANs property
    fn iban_mentioned(&self) -> Option<&[String]>;
    ///Get Detected IP addresses property
    fn ip_mentioned(&self) -> Option<&[String]>;
    ///Get Detected locations property
    fn location_mentioned(&self) -> Option<&[String]>;
    ///Get Detected names property
    fn names_mentioned(&self) -> Option<&[String]>;
    ///Get Detected people property
    fn people_mentioned(&self) -> Option<&[String]>;
    ///Get Detected phones property
    fn phone_mentioned(&self) -> Option<&[String]>;
}
///Trait for FTM schema: Interest
pub trait Interest: Interval {
    /// Get the entity ID
    fn id(&self) -> &str;
    /// Get the schema name
    fn schema(&self) -> &str;
    ///Get Role property
    fn role(&self) -> Option<&[String]>;
    ///Get Status property
    fn status(&self) -> Option<&[String]>;
}
///Trait for FTM schema: Interval
pub trait Interval {
    /// Get the entity ID
    fn id(&self) -> &str;
    /// Get the schema name
    fn schema(&self) -> &str;
    ///Get Aleph URL property
    fn aleph_url(&self) -> Option<&[String]>;
    ///Get Date property
    fn date(&self) -> Option<&[String]>;
    ///Get Description property
    fn description(&self) -> Option<&[String]>;
    ///Get End date property
    fn end_date(&self) -> Option<&[String]>;
    ///Get Index text property
    fn index_text(&self) -> Option<&[String]>;
    ///Get Modified on property
    fn modified_at(&self) -> Option<&[String]>;
    ///Get Detected names property
    fn names_mentioned(&self) -> Option<&[String]>;
    ///Get Source document property
    fn proof(&self) -> Option<&[String]>;
    ///Get Publishing source property
    fn publisher(&self) -> Option<&[String]>;
    ///Get Publishing source URL property
    fn publisher_url(&self) -> Option<&[String]>;
    ///Get Record ID property
    fn record_id(&self) -> Option<&[String]>;
    ///Get Retrieved on property
    fn retrieved_at(&self) -> Option<&[String]>;
    ///Get Source link property
    fn source_url(&self) -> Option<&[String]>;
    ///Get Start date property
    fn start_date(&self) -> Option<&[String]>;
    ///Get Summary property
    fn summary(&self) -> Option<&[String]>;
}
///Trait for FTM schema: Thing
pub trait Thing {
    /// Get the entity ID
    fn id(&self) -> &str;
    /// Get the schema name
    fn schema(&self) -> &str;
    ///Get Address property
    fn address(&self) -> Option<&[String]>;
    ///Get Address property
    fn address_entity(&self) -> Option<&[String]>;
    ///Get Aleph URL property
    fn aleph_url(&self) -> Option<&[String]>;
    ///Get Alias property
    fn alias(&self) -> Option<&[String]>;
    ///Get Country property
    fn country(&self) -> Option<&[String]>;
    ///Get Created at property
    fn created_at(&self) -> Option<&[String]>;
    ///Get Description property
    fn description(&self) -> Option<&[String]>;
    ///Get Index text property
    fn index_text(&self) -> Option<&[String]>;
    ///Get Keywords property
    fn keywords(&self) -> Option<&[String]>;
    ///Get Modified on property
    fn modified_at(&self) -> Option<&[String]>;
    ///Get Name property
    fn name(&self) -> Option<&[String]>;
    ///Get Notes property
    fn notes(&self) -> Option<&[String]>;
    ///Get Previous name property
    fn previous_name(&self) -> Option<&[String]>;
    ///Get Program property
    fn program(&self) -> Option<&[String]>;
    ///Get Program ID property
    fn program_id(&self) -> Option<&[String]>;
    ///Get Source document property
    fn proof(&self) -> Option<&[String]>;
    ///Get Publishing source property
    fn publisher(&self) -> Option<&[String]>;
    ///Get Publishing source URL property
    fn publisher_url(&self) -> Option<&[String]>;
    ///Get Retrieved on property
    fn retrieved_at(&self) -> Option<&[String]>;
    ///Get Source link property
    fn source_url(&self) -> Option<&[String]>;
    ///Get Summary property
    fn summary(&self) -> Option<&[String]>;
    ///Get Topics property
    fn topics(&self) -> Option<&[String]>;
    ///Get Weak alias property
    fn weak_alias(&self) -> Option<&[String]>;
    ///Get Wikidata ID property
    fn wikidata_id(&self) -> Option<&[String]>;
    ///Get Wikipedia Article property
    fn wikipedia_url(&self) -> Option<&[String]>;
}
///Trait for FTM schema: Value
pub trait Value {
    /// Get the entity ID
    fn id(&self) -> &str;
    /// Get the schema name
    fn schema(&self) -> &str;
    ///Get Amount property
    fn amount(&self) -> Option<&[f64]>;
    ///Get Amount in EUR property
    fn amount_eur(&self) -> Option<&[f64]>;
    ///Get Amount in USD property
    fn amount_usd(&self) -> Option<&[f64]>;
    ///Get Currency property
    fn currency(&self) -> Option<&[String]>;
}
