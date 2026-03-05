# ftm-types

[followthemoney](https://github.com/opensanctions/followthemoney) for Rust

:warning: unstable, use at your own risk, many changes ahead :warning:

## Overview

This library downloads FTM YAML schemas from the [opensanctions/followthemoney](https://github.com/opensanctions/followthemoney) repository and generates structs, one large Enum and a bunch of helpers.

At this moment this is an experiment, an attempt to bring the Python followthemoney API to Rust. Hopefully it enables upcoming Rust applications to be compatible with OpenAleph, OpenSanctions, and other followthemoney-based tools.

## Usage

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
ftm-types = "0.1"
```

And start exploring:

```rust
use ftm_types::generated::FtmEntity;
```

### Download and Generate Schemas

```bash
$ just download-ftm-schema 4.5.0
$ cargo run -- 4.5.0
```

### Using Generated Types

After code generation, the library produces one struct per schema:

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

but also a Trait for polymorphism:

// Traits for polymorphic code
pub trait Thing {
    fn id(&self) -> &str;
    fn name(&self) -> Option<&[String]>;
    fn country(&self) -> Option<&[String]>;
    // ... methods for all Thing properties
}

and implementations of it:

// Concrete types implement parent traits
impl Thing for Person {
    fn id(&self) -> &str { &self.id }
    fn name(&self) -> Option<&[String]> { self.name.as_deref() }
    // ...
}

and then one Enum with all the entity types:

// Enum for runtime polymorphism
pub enum FtmEntity {
    Person(Person),
    Organization(Organization),
    Company(Company),
    // ... all entity types
}
```

### Trait-Based Polymorphism

The library uses a **hybrid approach**: entity structs have flat structures for direct access, but also implement traits for polymorphic code. See also [examples/first_steps.rs](./examples/first_steps.rs) by running:

```bash
$ cargo run --example trait_polymorphism.rs
```
### Type Mapping

FTM types are mapped to Rust types as follows:

| FTM Type | Rust Type |
|----------|-----------|
| `name`, `text`, `string` | `Option<Vec<String>>` |
| `number` | `Option<Vec<f64>>` |
| `date` | `Option<Vec<String>>` |
| `json` | `Option<serde_json::Value>` |
| `country`, `email`, etc. | `Option<Vec<String>>` |

All properties are multi-valued by default (following FTM semantics) and optional (unless `required`).

## Features

* `builder` uses [Bon](https://bon-rs.com/) to generate a builder API. The builder will respect the `required` fields in the schema. 

## TODOs

- [ ] ID generation, hash-based
- [ ] Property normalization, transliteration
- [ ] unicode normalization (ICU)
- [ ] Property validation
- [ ] Fuzzy matching
- [ ] Merge operations / entity deduplication
- [ ] Schema metadata (`schema.is_a(other_schema)`)?
- [ ] Consider adding `required` fields to `new()` fn
- [ ] Consider helpers for single-value access (like `first_name()`)
- [ ] Consider impl `From<&str>` or accept `Into<String>` to reduce the String boilerplate

## License

MIT OR Apache-2.0

## Resources

- [FollowTheMoney Documentation](https://followthemoney.tech/)
- [FollowTheMoney Repository](https://github.com/opensanctions/followthemoney)
- [OpenSanctions](https://www.opensanctions.org/)
- [OpenAleph](https://www.openaleph.org/)
