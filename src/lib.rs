//! FollowTheMoney schema parser and code generator
//!
//! This library downloads FTM YAML schemas from the opensanctions/followthemoney
//! repository and generates type-safe Rust structs.

pub mod codegen;
pub mod schema;

pub use codegen::CodeGenerator;
pub use schema::{FtmProperty, FtmSchema, ResolvedSchema, SchemaRegistry};

pub mod generated;

pub use generated::FtmEntity;

#[cfg(feature = "rand")]
pub use enum_derived;
