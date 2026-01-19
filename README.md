# ftm-types

FollowTheMoney (FTM) schema parser and code generator for Rust.

## Overview

This library uses FTM YAML schemas from the [opensanctions/followthemoney](https://github.com/opensanctions/followthemoney) repository and generates type-safe Rust structs. It enables Rust applications to work with standardized entity types compatible with OpenAleph, OpenSanctions, and other FTM-based tools.

## Features

- **Code Generation**: Produces type-safe Rust structs from YAML definitions
- **Type Safety**: Maps FTM types to appropriate Rust types

## Usage

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
ftm-schema = { path = "../ftm-schema" }
```

### Download and Generate Schemas

```rust
// Generate Rust code
let codegen = CodeGenerator::new(registry, "src/generated");
codegen.generate_all()?;
```

### Using Generated Types

After code generation, the library produces:

```rust
// Generated entity structs with all properties flattened
pub struct Person {
    pub id: String,
    pub schema: String,
    pub name: Option<Vec<String>>,
    pub birth_date: Option<Vec<String>>,
    pub nationality: Option<Vec<String>>,
    // ... all properties from parent schemas flattened
}

// Traits for polymorphic code
pub trait Thing {
    fn id(&self) -> &str;
    fn name(&self) -> Option<&[String]>;
    fn country(&self) -> Option<&[String]>;
    // ... methods for all Thing properties
}

// Concrete types implement parent traits
impl Thing for Person {
    fn id(&self) -> &str { &self.id }
    fn name(&self) -> Option<&[String]> { self.name.as_deref() }
    // ...
}

// Enum for runtime polymorphism
pub enum FtmEntity {
    Person(Person),
    Organization(Organization),
    Company(Company),
    // ... all entity types
}
```

### Trait-Based Polymorphism

The library uses a **hybrid approach**: entity structs have flat structures for direct access, but also implement traits for polymorphic code:

```rust
use ftm_types::generated::*;

// Generic function working with any Thing
fn print_entity_name<T: Thing>(entity: &T) {
    if let Some(names) = entity.name() {
        println!("Names: {:?}", names);
    }
}

let person = Person::new("person-1");
let company = Company::new("company-1");

// Works with both types through the trait
print_entity_name(&person);
print_entity_name(&company);

// Trait objects for runtime polymorphism
let entities: Vec<&dyn Thing> = vec![&person, &company];
for entity in entities {
    println!("{}: {}", entity.id(), entity.schema());
}

// Direct field access still available
println!("Birth date: {:?}", person.birth_date);
```

See [examples/trait_polymorphism.rs](examples/trait_polymorphism.rs) for a complete example.

### Type Mapping

FTM types are mapped to Rust types as follows:

| FTM Type | Rust Type |
|----------|-----------|
| `name`, `text`, `string` | `Option<Vec<String>>` |
| `number` | `Option<Vec<f64>>` |
| `date` | `Option<Vec<String>>` |
| `json` | `Option<serde_json::Value>` |
| `country`, `email`, etc. | `Option<Vec<String>>` |

All properties are multi-valued by default (following FTM semantics) and optional.

## Development

### Running Tests

```bash
cargo test
```

### Building Documentation

```bash
cargo doc --open
```

## License

MIT OR Apache-2.0

## Resources

- [FollowTheMoney Documentation](https://followthemoney.tech/)
- [FollowTheMoney Repository](https://github.com/opensanctions/followthemoney)
- [OpenSanctions](https://www.opensanctions.org/)
- [OpenAleph](https://www.openaleph.org/)
