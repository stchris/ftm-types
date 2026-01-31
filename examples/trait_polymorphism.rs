// Example demonstrating trait-based polymorphism with FTM entities
//
// Run with: cargo run --example trait_polymorphism

use ftm_types::generated::*;

// Generic function that works with any Thing
fn print_entity_info<T: Thing>(entity: &T) {
    println!("Entity ID: {}", entity.id());
    println!("Schema: {}", entity.schema());

    if let Some(names) = entity.name() {
        println!("Names: {:?}", names);
    }

    if let Some(countries) = entity.country() {
        println!("Countries: {:?}", countries);
    }

    println!();
}

// Find entities with a specific name
fn find_by_name<'a, T: Thing>(entities: &'a [T], search_name: &str) -> Vec<&'a T> {
    entities
        .iter()
        .filter(|e| {
            e.name()
                .map(|names| names.iter().any(|n| n.contains(search_name)))
                .unwrap_or(false)
        })
        .collect()
}

// Count entities by country
fn count_by_country<T: Thing>(entities: &[T], country_code: &str) -> usize {
    entities
        .iter()
        .filter(|e| {
            e.country()
                .map(|countries| countries.contains(&country_code.to_string()))
                .unwrap_or(false)
        })
        .count()
}

fn main() {
    println!("=== FTM Trait-Based Polymorphism Example ===\n");

    // Create some entities using the builder
    let mut person = Person::builder()
        .id("person-1".to_string())
        .name(vec!["Alice Smith".to_string()])
        .build();
    person.country = Some(vec!["us".to_string(), "gb".to_string()]);
    person.birth_date = Some(vec!["1990-01-15".to_string()]);

    let mut company = Company::builder()
        .id("company-1".to_string())
        .name(vec!["Acme Corporation".to_string()])
        .build();
    company.country = Some(vec!["us".to_string()]);
    company.incorporation_date = Some(vec!["2010-05-20".to_string()]);

    let mut org = Organization::builder()
        .id("org-1".to_string())
        .name(vec!["United Nations".to_string()])
        .build();
    org.country = Some(vec!["un".to_string()]);

    println!("--- Using generic function with different entity types ---\n");

    // All concrete entities implement Thing, so we can use the same function
    print_entity_info(&person);
    print_entity_info(&company);
    print_entity_info(&org);

    println!("--- Polymorphic collection with trait objects ---\n");

    // Create a collection of trait objects
    let entities: Vec<&dyn Thing> = vec![&person, &company, &org];

    for entity in &entities {
        println!("{}: {}", entity.schema(), entity.id());
    }

    println!("\n--- Generic search across entity types ---\n");

    // Search for entities - works with any type that implements Thing
    let people = vec![person.clone()];
    let companies = vec![company.clone()];

    let found_people = find_by_name(&people, "Alice");
    let found_companies = find_by_name(&companies, "Acme");

    println!("Found {} people named 'Alice'", found_people.len());
    println!("Found {} companies named 'Acme'", found_companies.len());

    println!("\n--- Count by country ---\n");

    let us_people = count_by_country(&people, "us");
    let us_companies = count_by_country(&companies, "us");

    println!("People in US: {}", us_people);
    println!("Companies in US: {}", us_companies);

    println!("\n--- Direct field access (hybrid approach) ---\n");

    // We still have direct field access for convenience
    println!(
        "Person's birth date: {:?}",
        person.birth_date.as_ref().unwrap()[0]
    );
    println!(
        "Company's incorporation date: {:?}",
        company.incorporation_date.as_ref().unwrap()[0]
    );

    println!("\n✅ Hybrid approach gives us both polymorphism AND direct access!");
}
