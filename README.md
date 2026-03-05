# ftm-types

Rust types for [followthemoney](https://github.com/opensanctions/followthemoney)

:warning: in development, use at your own risk, can suddenly swerve, entities in mirror are larger than they appear 😳 :warning:

## Overview

This is a library crate attempting to generate Rust structs and helpers based on [opensanctions/followthemoney](https://github.com/opensanctions/followthemoney) schema definitions.

The binary application in this crate downloads FTM YAML schemas from the [opensanctions/followthemoney](https://github.com/opensanctions/followthemoney) repository and generates structs for entities, one large Enum and a bunch of helpers.

At this moment this is an experiment, an attempt to bring the Python followthemoney API to Rust. Hopefully it enables upcoming Rust applications to be compatible with OpenAleph, OpenSanctions, and other [opensanctions/followthemoney](https://github.com/opensanctions/followthemoney)-based tools.

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

### As a binary, to download and generate code

```bash
$ just download-ftm-schema 4.5.0
$ cargo run -- 4.5.0
```

### Using Generated Types

The library offers one struct per ftm schema:

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

* `builder` uses [Bon](https://bon-rs.com/) to generate a builder API. The builder will respect the `required` fields in the schema. Check out [the example code](./examples/builder.rs).

## License

MIT OR Apache-2.0

## Resources

- [FollowTheMoney Documentation](https://followthemoney.tech/)
- [FollowTheMoney Repository](https://github.com/opensanctions/followthemoney)
- [OpenAleph](https://www.openaleph.org/)

## I should also mention
- [OpenSanctions](https://www.opensanctions.org/)
- [DARC (Data Research Center)](https://dataresearchcenter.org/)
