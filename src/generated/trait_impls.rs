#![allow(missing_docs)]
use super::entities::*;
use super::traits::*;
impl Thing for Address {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Thing for Airplane {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Value for Airplane {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn amount(&self) -> Option<&[f64]> {
        self.amount.as_deref()
    }
    fn amount_eur(&self) -> Option<&[f64]> {
        self.amount_eur.as_deref()
    }
    fn amount_usd(&self) -> Option<&[f64]> {
        self.amount_usd.as_deref()
    }
}
impl Analyzable for Article {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn companies_mentioned(&self) -> Option<&[String]> {
        self.companies_mentioned.as_deref()
    }
    fn detected_country(&self) -> Option<&[String]> {
        self.detected_country.as_deref()
    }
    fn detected_language(&self) -> Option<&[String]> {
        self.detected_language.as_deref()
    }
    fn email_mentioned(&self) -> Option<&[String]> {
        self.email_mentioned.as_deref()
    }
    fn iban_mentioned(&self) -> Option<&[String]> {
        self.iban_mentioned.as_deref()
    }
    fn ip_mentioned(&self) -> Option<&[String]> {
        self.ip_mentioned.as_deref()
    }
    fn location_mentioned(&self) -> Option<&[String]> {
        self.location_mentioned.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn people_mentioned(&self) -> Option<&[String]> {
        self.people_mentioned.as_deref()
    }
    fn phone_mentioned(&self) -> Option<&[String]> {
        self.phone_mentioned.as_deref()
    }
}
impl Thing for Article {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Thing for Asset {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Value for Asset {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn amount(&self) -> Option<&[f64]> {
        self.amount.as_deref()
    }
    fn amount_eur(&self) -> Option<&[f64]> {
        self.amount_eur.as_deref()
    }
    fn amount_usd(&self) -> Option<&[f64]> {
        self.amount_usd.as_deref()
    }
}
impl Interval for Associate {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Analyzable for Audio {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn companies_mentioned(&self) -> Option<&[String]> {
        self.companies_mentioned.as_deref()
    }
    fn detected_country(&self) -> Option<&[String]> {
        self.detected_country.as_deref()
    }
    fn detected_language(&self) -> Option<&[String]> {
        self.detected_language.as_deref()
    }
    fn email_mentioned(&self) -> Option<&[String]> {
        self.email_mentioned.as_deref()
    }
    fn iban_mentioned(&self) -> Option<&[String]> {
        self.iban_mentioned.as_deref()
    }
    fn ip_mentioned(&self) -> Option<&[String]> {
        self.ip_mentioned.as_deref()
    }
    fn location_mentioned(&self) -> Option<&[String]> {
        self.location_mentioned.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn people_mentioned(&self) -> Option<&[String]> {
        self.people_mentioned.as_deref()
    }
    fn phone_mentioned(&self) -> Option<&[String]> {
        self.phone_mentioned.as_deref()
    }
}
impl Thing for Audio {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Thing for BankAccount {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Value for BankAccount {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn amount(&self) -> Option<&[f64]> {
        self.amount.as_deref()
    }
    fn amount_eur(&self) -> Option<&[f64]> {
        self.amount_eur.as_deref()
    }
    fn amount_usd(&self) -> Option<&[f64]> {
        self.amount_usd.as_deref()
    }
}
impl Interval for Call {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Interval for CallForTenders {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Thing for CallForTenders {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Thing for Company {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Value for Company {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn amount(&self) -> Option<&[f64]> {
        self.amount.as_deref()
    }
    fn amount_eur(&self) -> Option<&[f64]> {
        self.amount_eur.as_deref()
    }
    fn amount_usd(&self) -> Option<&[f64]> {
        self.amount_usd.as_deref()
    }
}
impl Thing for Contract {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Value for Contract {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn amount(&self) -> Option<&[f64]> {
        self.amount.as_deref()
    }
    fn amount_eur(&self) -> Option<&[f64]> {
        self.amount_eur.as_deref()
    }
    fn amount_usd(&self) -> Option<&[f64]> {
        self.amount_usd.as_deref()
    }
}
impl Interest for ContractAward {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
}
impl Interval for ContractAward {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Value for ContractAward {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn amount(&self) -> Option<&[f64]> {
        self.amount.as_deref()
    }
    fn amount_eur(&self) -> Option<&[f64]> {
        self.amount_eur.as_deref()
    }
    fn amount_usd(&self) -> Option<&[f64]> {
        self.amount_usd.as_deref()
    }
}
impl Thing for CourtCase {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Interest for CourtCaseParty {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
}
impl Interval for CourtCaseParty {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Thing for CryptoWallet {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Value for CryptoWallet {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn amount(&self) -> Option<&[f64]> {
        self.amount.as_deref()
    }
    fn amount_eur(&self) -> Option<&[f64]> {
        self.amount_eur.as_deref()
    }
    fn amount_usd(&self) -> Option<&[f64]> {
        self.amount_usd.as_deref()
    }
}
impl Interval for Debt {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Value for Debt {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn amount(&self) -> Option<&[f64]> {
        self.amount.as_deref()
    }
    fn amount_eur(&self) -> Option<&[f64]> {
        self.amount_eur.as_deref()
    }
    fn amount_usd(&self) -> Option<&[f64]> {
        self.amount_usd.as_deref()
    }
}
impl Interest for Directorship {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
}
impl Interval for Directorship {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Analyzable for Document {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn companies_mentioned(&self) -> Option<&[String]> {
        self.companies_mentioned.as_deref()
    }
    fn detected_country(&self) -> Option<&[String]> {
        self.detected_country.as_deref()
    }
    fn detected_language(&self) -> Option<&[String]> {
        self.detected_language.as_deref()
    }
    fn email_mentioned(&self) -> Option<&[String]> {
        self.email_mentioned.as_deref()
    }
    fn iban_mentioned(&self) -> Option<&[String]> {
        self.iban_mentioned.as_deref()
    }
    fn ip_mentioned(&self) -> Option<&[String]> {
        self.ip_mentioned.as_deref()
    }
    fn location_mentioned(&self) -> Option<&[String]> {
        self.location_mentioned.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn people_mentioned(&self) -> Option<&[String]> {
        self.people_mentioned.as_deref()
    }
    fn phone_mentioned(&self) -> Option<&[String]> {
        self.phone_mentioned.as_deref()
    }
}
impl Thing for Document {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Interval for EconomicActivity {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Analyzable for Email {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn companies_mentioned(&self) -> Option<&[String]> {
        self.companies_mentioned.as_deref()
    }
    fn detected_country(&self) -> Option<&[String]> {
        self.detected_country.as_deref()
    }
    fn detected_language(&self) -> Option<&[String]> {
        self.detected_language.as_deref()
    }
    fn email_mentioned(&self) -> Option<&[String]> {
        self.email_mentioned.as_deref()
    }
    fn iban_mentioned(&self) -> Option<&[String]> {
        self.iban_mentioned.as_deref()
    }
    fn ip_mentioned(&self) -> Option<&[String]> {
        self.ip_mentioned.as_deref()
    }
    fn location_mentioned(&self) -> Option<&[String]> {
        self.location_mentioned.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn people_mentioned(&self) -> Option<&[String]> {
        self.people_mentioned.as_deref()
    }
    fn phone_mentioned(&self) -> Option<&[String]> {
        self.phone_mentioned.as_deref()
    }
}
impl Thing for Email {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Interest for Employment {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
}
impl Interval for Employment {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Analyzable for Event {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn companies_mentioned(&self) -> Option<&[String]> {
        self.companies_mentioned.as_deref()
    }
    fn detected_country(&self) -> Option<&[String]> {
        self.detected_country.as_deref()
    }
    fn detected_language(&self) -> Option<&[String]> {
        self.detected_language.as_deref()
    }
    fn email_mentioned(&self) -> Option<&[String]> {
        self.email_mentioned.as_deref()
    }
    fn iban_mentioned(&self) -> Option<&[String]> {
        self.iban_mentioned.as_deref()
    }
    fn ip_mentioned(&self) -> Option<&[String]> {
        self.ip_mentioned.as_deref()
    }
    fn location_mentioned(&self) -> Option<&[String]> {
        self.location_mentioned.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn people_mentioned(&self) -> Option<&[String]> {
        self.people_mentioned.as_deref()
    }
    fn phone_mentioned(&self) -> Option<&[String]> {
        self.phone_mentioned.as_deref()
    }
}
impl Interval for Event {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Thing for Event {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Interval for Family {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Analyzable for Folder {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn companies_mentioned(&self) -> Option<&[String]> {
        self.companies_mentioned.as_deref()
    }
    fn detected_country(&self) -> Option<&[String]> {
        self.detected_country.as_deref()
    }
    fn detected_language(&self) -> Option<&[String]> {
        self.detected_language.as_deref()
    }
    fn email_mentioned(&self) -> Option<&[String]> {
        self.email_mentioned.as_deref()
    }
    fn iban_mentioned(&self) -> Option<&[String]> {
        self.iban_mentioned.as_deref()
    }
    fn ip_mentioned(&self) -> Option<&[String]> {
        self.ip_mentioned.as_deref()
    }
    fn location_mentioned(&self) -> Option<&[String]> {
        self.location_mentioned.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn people_mentioned(&self) -> Option<&[String]> {
        self.people_mentioned.as_deref()
    }
    fn phone_mentioned(&self) -> Option<&[String]> {
        self.phone_mentioned.as_deref()
    }
}
impl Thing for Folder {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Analyzable for HyperText {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn companies_mentioned(&self) -> Option<&[String]> {
        self.companies_mentioned.as_deref()
    }
    fn detected_country(&self) -> Option<&[String]> {
        self.detected_country.as_deref()
    }
    fn detected_language(&self) -> Option<&[String]> {
        self.detected_language.as_deref()
    }
    fn email_mentioned(&self) -> Option<&[String]> {
        self.email_mentioned.as_deref()
    }
    fn iban_mentioned(&self) -> Option<&[String]> {
        self.iban_mentioned.as_deref()
    }
    fn ip_mentioned(&self) -> Option<&[String]> {
        self.ip_mentioned.as_deref()
    }
    fn location_mentioned(&self) -> Option<&[String]> {
        self.location_mentioned.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn people_mentioned(&self) -> Option<&[String]> {
        self.people_mentioned.as_deref()
    }
    fn phone_mentioned(&self) -> Option<&[String]> {
        self.phone_mentioned.as_deref()
    }
}
impl Thing for HyperText {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Interval for Identification {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Analyzable for Image {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn companies_mentioned(&self) -> Option<&[String]> {
        self.companies_mentioned.as_deref()
    }
    fn detected_country(&self) -> Option<&[String]> {
        self.detected_country.as_deref()
    }
    fn detected_language(&self) -> Option<&[String]> {
        self.detected_language.as_deref()
    }
    fn email_mentioned(&self) -> Option<&[String]> {
        self.email_mentioned.as_deref()
    }
    fn iban_mentioned(&self) -> Option<&[String]> {
        self.iban_mentioned.as_deref()
    }
    fn ip_mentioned(&self) -> Option<&[String]> {
        self.ip_mentioned.as_deref()
    }
    fn location_mentioned(&self) -> Option<&[String]> {
        self.location_mentioned.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn people_mentioned(&self) -> Option<&[String]> {
        self.people_mentioned.as_deref()
    }
    fn phone_mentioned(&self) -> Option<&[String]> {
        self.phone_mentioned.as_deref()
    }
}
impl Thing for Image {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Thing for LegalEntity {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Thing for License {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Value for License {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn amount(&self) -> Option<&[f64]> {
        self.amount.as_deref()
    }
    fn amount_eur(&self) -> Option<&[f64]> {
        self.amount_eur.as_deref()
    }
    fn amount_usd(&self) -> Option<&[f64]> {
        self.amount_usd.as_deref()
    }
}
impl Interest for Membership {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
}
impl Interval for Membership {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Analyzable for Message {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn companies_mentioned(&self) -> Option<&[String]> {
        self.companies_mentioned.as_deref()
    }
    fn detected_country(&self) -> Option<&[String]> {
        self.detected_country.as_deref()
    }
    fn detected_language(&self) -> Option<&[String]> {
        self.detected_language.as_deref()
    }
    fn email_mentioned(&self) -> Option<&[String]> {
        self.email_mentioned.as_deref()
    }
    fn iban_mentioned(&self) -> Option<&[String]> {
        self.iban_mentioned.as_deref()
    }
    fn ip_mentioned(&self) -> Option<&[String]> {
        self.ip_mentioned.as_deref()
    }
    fn location_mentioned(&self) -> Option<&[String]> {
        self.location_mentioned.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn people_mentioned(&self) -> Option<&[String]> {
        self.people_mentioned.as_deref()
    }
    fn phone_mentioned(&self) -> Option<&[String]> {
        self.phone_mentioned.as_deref()
    }
}
impl Interval for Message {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Thing for Message {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Analyzable for Note {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn companies_mentioned(&self) -> Option<&[String]> {
        self.companies_mentioned.as_deref()
    }
    fn detected_country(&self) -> Option<&[String]> {
        self.detected_country.as_deref()
    }
    fn detected_language(&self) -> Option<&[String]> {
        self.detected_language.as_deref()
    }
    fn email_mentioned(&self) -> Option<&[String]> {
        self.email_mentioned.as_deref()
    }
    fn iban_mentioned(&self) -> Option<&[String]> {
        self.iban_mentioned.as_deref()
    }
    fn ip_mentioned(&self) -> Option<&[String]> {
        self.ip_mentioned.as_deref()
    }
    fn location_mentioned(&self) -> Option<&[String]> {
        self.location_mentioned.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn people_mentioned(&self) -> Option<&[String]> {
        self.people_mentioned.as_deref()
    }
    fn phone_mentioned(&self) -> Option<&[String]> {
        self.phone_mentioned.as_deref()
    }
}
impl Thing for Note {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Interval for Occupancy {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Thing for Organization {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Interest for Ownership {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
}
impl Interval for Ownership {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Analyzable for Package {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn companies_mentioned(&self) -> Option<&[String]> {
        self.companies_mentioned.as_deref()
    }
    fn detected_country(&self) -> Option<&[String]> {
        self.detected_country.as_deref()
    }
    fn detected_language(&self) -> Option<&[String]> {
        self.detected_language.as_deref()
    }
    fn email_mentioned(&self) -> Option<&[String]> {
        self.email_mentioned.as_deref()
    }
    fn iban_mentioned(&self) -> Option<&[String]> {
        self.iban_mentioned.as_deref()
    }
    fn ip_mentioned(&self) -> Option<&[String]> {
        self.ip_mentioned.as_deref()
    }
    fn location_mentioned(&self) -> Option<&[String]> {
        self.location_mentioned.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn people_mentioned(&self) -> Option<&[String]> {
        self.people_mentioned.as_deref()
    }
    fn phone_mentioned(&self) -> Option<&[String]> {
        self.phone_mentioned.as_deref()
    }
}
impl Thing for Package {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Analyzable for Pages {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn companies_mentioned(&self) -> Option<&[String]> {
        self.companies_mentioned.as_deref()
    }
    fn detected_country(&self) -> Option<&[String]> {
        self.detected_country.as_deref()
    }
    fn detected_language(&self) -> Option<&[String]> {
        self.detected_language.as_deref()
    }
    fn email_mentioned(&self) -> Option<&[String]> {
        self.email_mentioned.as_deref()
    }
    fn iban_mentioned(&self) -> Option<&[String]> {
        self.iban_mentioned.as_deref()
    }
    fn ip_mentioned(&self) -> Option<&[String]> {
        self.ip_mentioned.as_deref()
    }
    fn location_mentioned(&self) -> Option<&[String]> {
        self.location_mentioned.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn people_mentioned(&self) -> Option<&[String]> {
        self.people_mentioned.as_deref()
    }
    fn phone_mentioned(&self) -> Option<&[String]> {
        self.phone_mentioned.as_deref()
    }
}
impl Thing for Pages {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Interval for Passport {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Interval for Payment {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Value for Payment {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn amount(&self) -> Option<&[f64]> {
        self.amount.as_deref()
    }
    fn amount_eur(&self) -> Option<&[f64]> {
        self.amount_eur.as_deref()
    }
    fn amount_usd(&self) -> Option<&[f64]> {
        self.amount_usd.as_deref()
    }
}
impl Thing for Person {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Analyzable for PlainText {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn companies_mentioned(&self) -> Option<&[String]> {
        self.companies_mentioned.as_deref()
    }
    fn detected_country(&self) -> Option<&[String]> {
        self.detected_country.as_deref()
    }
    fn detected_language(&self) -> Option<&[String]> {
        self.detected_language.as_deref()
    }
    fn email_mentioned(&self) -> Option<&[String]> {
        self.email_mentioned.as_deref()
    }
    fn iban_mentioned(&self) -> Option<&[String]> {
        self.iban_mentioned.as_deref()
    }
    fn ip_mentioned(&self) -> Option<&[String]> {
        self.ip_mentioned.as_deref()
    }
    fn location_mentioned(&self) -> Option<&[String]> {
        self.location_mentioned.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn people_mentioned(&self) -> Option<&[String]> {
        self.people_mentioned.as_deref()
    }
    fn phone_mentioned(&self) -> Option<&[String]> {
        self.phone_mentioned.as_deref()
    }
}
impl Thing for PlainText {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Thing for Position {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Interval for Project {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Thing for Project {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Value for Project {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn amount(&self) -> Option<&[f64]> {
        self.amount.as_deref()
    }
    fn amount_eur(&self) -> Option<&[f64]> {
        self.amount_eur.as_deref()
    }
    fn amount_usd(&self) -> Option<&[f64]> {
        self.amount_usd.as_deref()
    }
}
impl Interest for ProjectParticipant {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
}
impl Interval for ProjectParticipant {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Thing for PublicBody {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Thing for RealEstate {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Value for RealEstate {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn amount(&self) -> Option<&[f64]> {
        self.amount.as_deref()
    }
    fn amount_eur(&self) -> Option<&[f64]> {
        self.amount_eur.as_deref()
    }
    fn amount_usd(&self) -> Option<&[f64]> {
        self.amount_usd.as_deref()
    }
}
impl Interest for Representation {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
}
impl Interval for Representation {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Interval for Risk {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Interval for Sanction {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Thing for Security {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Value for Security {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn amount(&self) -> Option<&[f64]> {
        self.amount.as_deref()
    }
    fn amount_eur(&self) -> Option<&[f64]> {
        self.amount_eur.as_deref()
    }
    fn amount_usd(&self) -> Option<&[f64]> {
        self.amount_usd.as_deref()
    }
}
impl Interest for Succession {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
}
impl Interval for Succession {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Analyzable for Table {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn companies_mentioned(&self) -> Option<&[String]> {
        self.companies_mentioned.as_deref()
    }
    fn detected_country(&self) -> Option<&[String]> {
        self.detected_country.as_deref()
    }
    fn detected_language(&self) -> Option<&[String]> {
        self.detected_language.as_deref()
    }
    fn email_mentioned(&self) -> Option<&[String]> {
        self.email_mentioned.as_deref()
    }
    fn iban_mentioned(&self) -> Option<&[String]> {
        self.iban_mentioned.as_deref()
    }
    fn ip_mentioned(&self) -> Option<&[String]> {
        self.ip_mentioned.as_deref()
    }
    fn location_mentioned(&self) -> Option<&[String]> {
        self.location_mentioned.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn people_mentioned(&self) -> Option<&[String]> {
        self.people_mentioned.as_deref()
    }
    fn phone_mentioned(&self) -> Option<&[String]> {
        self.phone_mentioned.as_deref()
    }
}
impl Thing for Table {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Interval for TaxRoll {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Analyzable for Trip {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn companies_mentioned(&self) -> Option<&[String]> {
        self.companies_mentioned.as_deref()
    }
    fn detected_country(&self) -> Option<&[String]> {
        self.detected_country.as_deref()
    }
    fn detected_language(&self) -> Option<&[String]> {
        self.detected_language.as_deref()
    }
    fn email_mentioned(&self) -> Option<&[String]> {
        self.email_mentioned.as_deref()
    }
    fn iban_mentioned(&self) -> Option<&[String]> {
        self.iban_mentioned.as_deref()
    }
    fn ip_mentioned(&self) -> Option<&[String]> {
        self.ip_mentioned.as_deref()
    }
    fn location_mentioned(&self) -> Option<&[String]> {
        self.location_mentioned.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn people_mentioned(&self) -> Option<&[String]> {
        self.people_mentioned.as_deref()
    }
    fn phone_mentioned(&self) -> Option<&[String]> {
        self.phone_mentioned.as_deref()
    }
}
impl Interval for Trip {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Thing for Trip {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Interest for UnknownLink {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
}
impl Interval for UnknownLink {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn date(&self) -> Option<&[String]> {
        self.date.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn end_date(&self) -> Option<&[String]> {
        self.end_date.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn start_date(&self) -> Option<&[String]> {
        self.start_date.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
}
impl Thing for UserAccount {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Thing for Vehicle {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Value for Vehicle {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn amount(&self) -> Option<&[f64]> {
        self.amount.as_deref()
    }
    fn amount_eur(&self) -> Option<&[f64]> {
        self.amount_eur.as_deref()
    }
    fn amount_usd(&self) -> Option<&[f64]> {
        self.amount_usd.as_deref()
    }
}
impl Thing for Vessel {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Value for Vessel {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn amount(&self) -> Option<&[f64]> {
        self.amount.as_deref()
    }
    fn amount_eur(&self) -> Option<&[f64]> {
        self.amount_eur.as_deref()
    }
    fn amount_usd(&self) -> Option<&[f64]> {
        self.amount_usd.as_deref()
    }
}
impl Analyzable for Video {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn companies_mentioned(&self) -> Option<&[String]> {
        self.companies_mentioned.as_deref()
    }
    fn detected_country(&self) -> Option<&[String]> {
        self.detected_country.as_deref()
    }
    fn detected_language(&self) -> Option<&[String]> {
        self.detected_language.as_deref()
    }
    fn email_mentioned(&self) -> Option<&[String]> {
        self.email_mentioned.as_deref()
    }
    fn iban_mentioned(&self) -> Option<&[String]> {
        self.iban_mentioned.as_deref()
    }
    fn ip_mentioned(&self) -> Option<&[String]> {
        self.ip_mentioned.as_deref()
    }
    fn location_mentioned(&self) -> Option<&[String]> {
        self.location_mentioned.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn people_mentioned(&self) -> Option<&[String]> {
        self.people_mentioned.as_deref()
    }
    fn phone_mentioned(&self) -> Option<&[String]> {
        self.phone_mentioned.as_deref()
    }
}
impl Thing for Video {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
impl Analyzable for Workbook {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn companies_mentioned(&self) -> Option<&[String]> {
        self.companies_mentioned.as_deref()
    }
    fn detected_country(&self) -> Option<&[String]> {
        self.detected_country.as_deref()
    }
    fn detected_language(&self) -> Option<&[String]> {
        self.detected_language.as_deref()
    }
    fn email_mentioned(&self) -> Option<&[String]> {
        self.email_mentioned.as_deref()
    }
    fn iban_mentioned(&self) -> Option<&[String]> {
        self.iban_mentioned.as_deref()
    }
    fn ip_mentioned(&self) -> Option<&[String]> {
        self.ip_mentioned.as_deref()
    }
    fn location_mentioned(&self) -> Option<&[String]> {
        self.location_mentioned.as_deref()
    }
    fn names_mentioned(&self) -> Option<&[String]> {
        self.names_mentioned.as_deref()
    }
    fn people_mentioned(&self) -> Option<&[String]> {
        self.people_mentioned.as_deref()
    }
    fn phone_mentioned(&self) -> Option<&[String]> {
        self.phone_mentioned.as_deref()
    }
}
impl Thing for Workbook {
    fn id(&self) -> &str {
        &self.id
    }
    fn schema(&self) -> &str {
        &self.schema
    }
    fn address(&self) -> Option<&[String]> {
        self.address.as_deref()
    }
    fn address_entity(&self) -> Option<&[String]> {
        self.address_entity.as_deref()
    }
    fn aleph_url(&self) -> Option<&[String]> {
        self.aleph_url.as_deref()
    }
    fn alias(&self) -> Option<&[String]> {
        self.alias.as_deref()
    }
    fn country(&self) -> Option<&[String]> {
        self.country.as_deref()
    }
    fn created_at(&self) -> Option<&[String]> {
        self.created_at.as_deref()
    }
    fn description(&self) -> Option<&[String]> {
        self.description.as_deref()
    }
    fn index_text(&self) -> Option<&[String]> {
        self.index_text.as_deref()
    }
    fn modified_at(&self) -> Option<&[String]> {
        self.modified_at.as_deref()
    }
    fn name(&self) -> Option<&[String]> {
        self.name.as_deref()
    }
    fn notes(&self) -> Option<&[String]> {
        self.notes.as_deref()
    }
    fn previous_name(&self) -> Option<&[String]> {
        self.previous_name.as_deref()
    }
    fn program_id(&self) -> Option<&[String]> {
        self.program_id.as_deref()
    }
    fn proof(&self) -> Option<&[String]> {
        self.proof.as_deref()
    }
    fn publisher_url(&self) -> Option<&[String]> {
        self.publisher_url.as_deref()
    }
    fn retrieved_at(&self) -> Option<&[String]> {
        self.retrieved_at.as_deref()
    }
    fn source_url(&self) -> Option<&[String]> {
        self.source_url.as_deref()
    }
    fn summary(&self) -> Option<&[String]> {
        self.summary.as_deref()
    }
    fn topics(&self) -> Option<&[String]> {
        self.topics.as_deref()
    }
    fn weak_alias(&self) -> Option<&[String]> {
        self.weak_alias.as_deref()
    }
    fn wikidata_id(&self) -> Option<&[String]> {
        self.wikidata_id.as_deref()
    }
    fn wikipedia_url(&self) -> Option<&[String]> {
        self.wikipedia_url.as_deref()
    }
}
