//! Code generation for FTM schemas
//!
//! Generates type-safe Rust structs from FTM schema definitions.

use anyhow::{Context, Result};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::fs;
use std::path::{Path, PathBuf};

use crate::schema::{ResolvedSchema, SchemaRegistry};

/// Code generator for FTM schemas
pub struct CodeGenerator {
    registry: SchemaRegistry,
    output_dir: PathBuf,
}

impl CodeGenerator {
    /// Create a new code generator
    pub fn new(registry: SchemaRegistry, output_dir: impl AsRef<Path>) -> Self {
        Self {
            registry,
            output_dir: output_dir.as_ref().to_path_buf(),
        }
    }

    /// Generate all code files
    pub fn generate_all(&self) -> Result<()> {
        // Create output directory
        fs::create_dir_all(&self.output_dir).context(format!(
            "Failed to create output directory: {:?}",
            self.output_dir
        ))?;

        println!("\nGenerating code...");

        // Generate entity structs
        let entities_code = self.generate_entity_structs()?;
        self.write_module("entities.rs", entities_code)?;
        println!("  ✓ entities.rs");

        // Generate FtmEntity enum
        let enum_code = self.generate_ftm_entity_enum()?;
        self.write_module("ftm_entity.rs", enum_code)?;
        println!("  ✓ ftm_entity.rs");

        // Generate traits for abstract schemas
        let traits_code = self.generate_traits()?;
        self.write_module("traits.rs", traits_code)?;
        println!("  ✓ traits.rs");

        // Generate trait implementations
        let trait_impls_code = self.generate_trait_implementations()?;
        self.write_module("trait_impls.rs", trait_impls_code)?;
        println!("  ✓ trait_impls.rs");

        // Generate mod.rs
        let mod_code = self.generate_mod_file();
        self.write_module("mod.rs", mod_code)?;
        println!("  ✓ mod.rs");

        Ok(())
    }

    /// Generate entity structs for all concrete schemas
    fn generate_entity_structs(&self) -> Result<TokenStream> {
        let mut structs = Vec::new();

        for schema_name in self.registry.schema_names() {
            let resolved = self.registry.resolve_inheritance(&schema_name)?;

            // Skip abstract schemas
            if resolved.is_abstract() {
                continue;
            }

            let struct_code = self.generate_entity_struct(&resolved)?;
            structs.push(struct_code);
        }

        Ok(quote! {
            // Auto-generated - DO NOT EDIT
            #![allow(missing_docs)]

            use serde::{Deserialize, Serialize};
            #[cfg(feature = "rand")] use enum_derived::Rand;

            #[cfg(feature = "rand")]
            fn default_json_value() -> Option<serde_json::Value> {
                Some(serde_json::Value::Object(serde_json::Map::new()))
            }

            #(#structs)*
        })
    }

    /// Generate a single entity struct
    fn generate_entity_struct(&self, schema: &ResolvedSchema) -> Result<TokenStream> {
        let struct_name = Ident::new(&schema.name, Span::call_site());
        let label = schema.label().unwrap_or(&schema.name);
        let doc_comment = format!("FTM Schema: {}", label);
        let schema_name_str = &schema.name;

        // Generate fields
        let mut fields = Vec::new();

        // Add id field (required for all entities)
        fields.push(quote! {
            pub id: String
        });

        // Add schema field (always the schema name)
        fields.push(quote! {
            pub schema: String
        });

        // Add properties
        let mut property_names: Vec<_> = schema.all_properties.keys().collect();
        property_names.sort();

        for prop_name in property_names {
            let property = &schema.all_properties[prop_name];
            let field_name = self.property_to_field_name(prop_name);

            // Skip properties without a type (shouldn't happen but be defensive)
            let prop_type = match &property.type_ {
                Some(t) => t.as_str(),
                None => continue,
            };

            let field_type = self.map_property_type(prop_type);

            // Add custom_rand attribute for serde_json::Value fields
            let custom_rand_attr = if prop_type == "json" {
                quote! { #[cfg_attr(feature = "rand", custom_rand(default_json_value))] }
            } else {
                quote! {}
            };

            let field_doc = if let Some(label) = &property.label {
                format!("Property: {}", label)
            } else {
                format!("Property: {}", prop_name)
            };

            fields.push(quote! {
                #[doc = #field_doc]
                #[serde(skip_serializing_if = "Option::is_none")]
                #custom_rand_attr
                pub #field_name: #field_type
            });
        }

        Ok(quote! {
            #[doc = #doc_comment]
            #[derive(Debug, Clone, Serialize, Deserialize, Default)]
            #[cfg_attr(feature = "rand", derive(Rand))]
            #[serde(rename_all = "camelCase")]
            pub struct #struct_name {
                #(#fields),*
            }

            impl #struct_name {
                /// Create a new entity with the given ID
                pub fn new(id: impl Into<String>) -> Self {
                    Self {
                        id: id.into(),
                        schema: #schema_name_str.to_string(),
                        ..Default::default()
                    }
                }

                /// Get the schema name
                pub fn schema_name() -> &'static str {
                    #schema_name_str
                }
            }
        })
    }

    /// Generate the FtmEntity enum
    fn generate_ftm_entity_enum(&self) -> Result<TokenStream> {
        let mut variants = Vec::new();
        let mut match_schema_arms = Vec::new();
        let mut match_id_arms = Vec::new();
        let mut from_impls = Vec::new();

        for schema_name in self.registry.schema_names() {
            let resolved = self.registry.resolve_inheritance(&schema_name)?;

            // Skip abstract schemas
            if resolved.is_abstract() {
                continue;
            }

            let variant_name = Ident::new(&schema_name, Span::call_site());
            let type_name = Ident::new(&schema_name, Span::call_site());

            variants.push(quote! {
                #variant_name(#type_name)
            });

            match_schema_arms.push(quote! {
                FtmEntity::#variant_name(_) => #schema_name
            });

            match_id_arms.push(quote! {
                FtmEntity::#variant_name(entity) => &entity.id
            });

            from_impls.push(quote! {
                impl From<#type_name> for FtmEntity {
                    fn from(entity: #type_name) -> Self {
                        FtmEntity::#variant_name(entity)
                    }
                }
            });
        }

        Ok(quote! {
            // Auto-generated - DO NOT EDIT
            #![allow(missing_docs)]

            use super::entities::*;
            use serde::{Deserialize, Serialize};

            /// FTM Entity enum for runtime polymorphism
            #[derive(Debug, Clone, Serialize, Deserialize)]
            #[serde(tag = "schema")]
            pub enum FtmEntity {
                #(#variants),*
            }

            impl FtmEntity {
                /// Get the schema name for this entity
                pub fn schema(&self) -> &str {
                    match self {
                        #(#match_schema_arms),*
                    }
                }

                /// Get the entity ID
                pub fn id(&self) -> &str {
                    match self {
                        #(#match_id_arms),*
                    }
                }
            }

            #(#from_impls)*
        })
    }

    /// Generate mod.rs file
    fn generate_mod_file(&self) -> TokenStream {
        quote! {
            // Auto-generated - DO NOT EDIT
            #![allow(missing_docs)]

            pub mod entities;
            pub mod ftm_entity;
            pub mod traits;
            pub mod trait_impls;

            pub use entities::*;
            pub use ftm_entity::FtmEntity;
            pub use traits::*;
        }
    }

    /// Generate trait definitions for abstract schemas
    fn generate_traits(&self) -> Result<TokenStream> {
        let mut traits = Vec::new();

        for schema_name in self.registry.schema_names() {
            let schema = self
                .registry
                .get(&schema_name)
                .context(format!("Schema not found: {}", schema_name))?;

            // Only generate traits for abstract schemas
            if !schema.abstract_.unwrap_or(false) {
                continue;
            }

            let trait_code = self.generate_trait(&schema_name, schema)?;
            traits.push(trait_code);
        }

        Ok(quote! {
            // Auto-generated - DO NOT EDIT
            #![allow(missing_docs)]

            /// Traits representing FTM schema inheritance hierarchy.
            ///
            /// These traits enable polymorphic code that works across entity types.
            /// All concrete entity structs implement the traits for their parent schemas.

            #(#traits)*
        })
    }

    /// Generate a single trait definition for an abstract schema
    fn generate_trait(
        &self,
        schema_name: &str,
        schema: &crate::schema::FtmSchema,
    ) -> Result<TokenStream> {
        let trait_name = Ident::new(schema_name, Span::call_site());
        let doc_comment = format!(
            "Trait for FTM schema: {}",
            schema.label.as_deref().unwrap_or(schema_name)
        );

        // Determine parent traits
        let parent_traits: Vec<TokenStream> = if let Some(extends) = &schema.extends {
            extends
                .iter()
                .map(|parent| {
                    let parent_ident = Ident::new(parent, Span::call_site());
                    quote! { #parent_ident }
                })
                .collect()
        } else {
            vec![]
        };

        let trait_bounds = if parent_traits.is_empty() {
            quote! {}
        } else {
            quote! { : #(#parent_traits)+* }
        };

        // Generate trait methods for properties
        let mut methods = Vec::new();

        // Add id and schema methods (all entities have these)
        methods.push(quote! {
            /// Get the entity ID
            fn id(&self) -> &str;
        });

        methods.push(quote! {
            /// Get the schema name
            fn schema(&self) -> &str;
        });

        // Add property accessor methods
        let mut property_names: Vec<_> = schema.properties.keys().collect();
        property_names.sort();

        for prop_name in property_names {
            let property = &schema.properties[prop_name];
            let method_name = self.property_to_field_name(prop_name);

            // Skip properties without a type
            let prop_type = match &property.type_ {
                Some(t) => t.as_str(),
                None => continue,
            };

            let return_type = match prop_type {
                "number" => quote! { Option<&[f64]> },
                "json" => quote! { Option<&serde_json::Value> },
                _ => quote! { Option<&[String]> },
            };

            let method_doc = if let Some(label) = &property.label {
                format!("Get {} property", label)
            } else {
                format!("Get {} property", prop_name)
            };

            methods.push(quote! {
                #[doc = #method_doc]
                fn #method_name(&self) -> #return_type;
            });
        }

        Ok(quote! {
            #[doc = #doc_comment]
            pub trait #trait_name #trait_bounds {
                #(#methods)*
            }
        })
    }

    /// Generate trait implementations for concrete schemas
    fn generate_trait_implementations(&self) -> Result<TokenStream> {
        let mut impls = Vec::new();

        for schema_name in self.registry.schema_names() {
            let resolved = self.registry.resolve_inheritance(&schema_name)?;

            // Only generate impls for concrete schemas
            if resolved.is_abstract() {
                continue;
            }

            let impl_code = self.generate_trait_impls_for_entity(&resolved)?;
            impls.extend(impl_code);
        }

        Ok(quote! {
            // Auto-generated - DO NOT EDIT
            #![allow(missing_docs)]

            use super::entities::*;
            use super::traits::*;

            #(#impls)*
        })
    }

    /// Generate all trait implementations for a single entity
    fn generate_trait_impls_for_entity(&self, schema: &ResolvedSchema) -> Result<Vec<TokenStream>> {
        let mut impls = Vec::new();
        let struct_name = Ident::new(&schema.name, Span::call_site());

        // Get all parent schemas (including transitive parents)
        let parent_schemas = self.get_all_parent_schemas(&schema.name)?;

        // Generate impl for each parent trait
        for parent_name in parent_schemas {
            let parent_schema = self
                .registry
                .get(&parent_name)
                .context(format!("Parent schema not found: {}", parent_name))?;

            // Only implement traits for abstract schemas
            if !parent_schema.abstract_.unwrap_or(false) {
                continue;
            }

            let trait_name = Ident::new(&parent_name, Span::call_site());
            let mut methods = Vec::new();

            // Implement id and schema methods
            methods.push(quote! {
                fn id(&self) -> &str {
                    &self.id
                }
            });

            methods.push(quote! {
                fn schema(&self) -> &str {
                    &self.schema
                }
            });

            // Implement property accessor methods
            let mut property_names: Vec<_> = parent_schema.properties.keys().collect();
            property_names.sort();

            for prop_name in property_names {
                let property = &parent_schema.properties[prop_name];
                let method_name = self.property_to_field_name(prop_name);
                let field_name = self.property_to_field_name(prop_name);

                // Skip properties without a type
                let prop_type = match &property.type_ {
                    Some(t) => t.as_str(),
                    None => continue,
                };

                let method_impl = match prop_type {
                    "number" => quote! {
                        fn #method_name(&self) -> Option<&[f64]> {
                            self.#field_name.as_deref()
                        }
                    },
                    "json" => quote! {
                        fn #method_name(&self) -> Option<&serde_json::Value> {
                            self.#field_name.as_ref()
                        }
                    },
                    _ => quote! {
                        fn #method_name(&self) -> Option<&[String]> {
                            self.#field_name.as_deref()
                        }
                    },
                };

                methods.push(method_impl);
            }

            impls.push(quote! {
                impl #trait_name for #struct_name {
                    #(#methods)*
                }
            });
        }

        Ok(impls)
    }

    /// Get all parent schemas (including transitive parents) for a given schema
    fn get_all_parent_schemas(&self, schema_name: &str) -> Result<Vec<String>> {
        let mut parents_set = std::collections::HashSet::new();
        let mut visited = std::collections::HashSet::new();
        self.collect_parents_recursive(schema_name, &mut parents_set, &mut visited)?;

        // Convert to Vec for iteration
        let mut parents: Vec<String> = parents_set.into_iter().collect();
        parents.sort(); // Sort for consistent output
        Ok(parents)
    }

    /// Recursively collect parent schemas
    fn collect_parents_recursive(
        &self,
        schema_name: &str,
        parents: &mut std::collections::HashSet<String>,
        visited: &mut std::collections::HashSet<String>,
    ) -> Result<()> {
        if visited.contains(schema_name) {
            return Ok(());
        }
        visited.insert(schema_name.to_string());

        let schema = self
            .registry
            .get(schema_name)
            .context(format!("Schema not found: {}", schema_name))?;

        if let Some(extends) = &schema.extends {
            for parent_name in extends {
                parents.insert(parent_name.clone());
                self.collect_parents_recursive(parent_name, parents, visited)?;
            }
        }

        Ok(())
    }

    /// Map FTM property types to Rust types
    fn map_property_type(&self, ftm_type: &str) -> TokenStream {
        match ftm_type {
            "number" => quote! { Option<Vec<f64>> },
            "date" => quote! { Option<Vec<String>> },
            "json" => quote! { Option<serde_json::Value> },
            _ => quote! { Option<Vec<String>> },
        }
    }

    /// Convert property name to valid Rust field name
    fn property_to_field_name(&self, prop_name: &str) -> Ident {
        // Convert camelCase/PascalCase to snake_case
        let snake_case = self.to_snake_case(prop_name);

        // Handle Rust keywords
        let field_name = match snake_case.as_str() {
            "type" => "type_".to_string(),
            "match" => "match_".to_string(),
            "ref" => "ref_".to_string(),
            _ => snake_case,
        };

        Ident::new(&field_name, Span::call_site())
    }

    /// Convert string to snake_case
    fn to_snake_case(&self, s: &str) -> String {
        // Handle special cases
        if s.to_uppercase() == s && s.len() <= 3 {
            // Acronyms like "ID", "API", etc. -> all lowercase
            return s.to_lowercase();
        }

        let mut result = String::new();
        let mut prev_is_upper = false;

        for (i, ch) in s.chars().enumerate() {
            if ch.is_uppercase() {
                if i > 0 && !prev_is_upper {
                    result.push('_');
                }
                result.push(ch.to_lowercase().next().unwrap());
                prev_is_upper = true;
            } else {
                result.push(ch);
                prev_is_upper = false;
            }
        }

        result
    }

    /// Write module to file with formatting
    fn write_module(&self, filename: &str, tokens: TokenStream) -> Result<()> {
        let path = self.output_dir.join(filename);

        // Parse and format the generated code
        let syntax_tree = syn::parse2(tokens).context("Failed to parse generated code")?;
        let formatted = prettyplease::unparse(&syntax_tree);

        fs::write(&path, formatted).context(format!("Failed to write file: {:?}", path))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::SchemaRegistry;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_schema(dir: &std::path::Path, name: &str, yaml: &str) {
        let path = dir.join(format!("{}.yml", name));
        let mut file = fs::File::create(path).unwrap();
        file.write_all(yaml.as_bytes()).unwrap();
    }

    #[test]
    fn test_code_generation() {
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
        let output_dir = temp_dir.path().join("generated");
        let codegen = CodeGenerator::new(registry, &output_dir);

        let result = codegen.generate_all();
        assert!(result.is_ok(), "Code generation failed: {:?}", result);

        // Check generated files exist
        assert!(output_dir.join("mod.rs").exists());
        assert!(output_dir.join("entities.rs").exists());
        assert!(output_dir.join("ftm_entity.rs").exists());
        assert!(output_dir.join("traits.rs").exists());
        assert!(output_dir.join("trait_impls.rs").exists());
    }

    #[test]
    fn test_snake_case_conversion() {
        let temp_dir = TempDir::new().unwrap();

        create_test_schema(
            temp_dir.path(),
            "Thing",
            r#"
label: Thing
properties: {}
"#,
        );

        let registry = SchemaRegistry::load_from_cache(temp_dir.path()).unwrap();
        let codegen = CodeGenerator::new(registry, "/tmp/test");

        assert_eq!(codegen.to_snake_case("firstName"), "first_name");
        assert_eq!(codegen.to_snake_case("birthDate"), "birth_date");
        assert_eq!(codegen.to_snake_case("name"), "name");
        assert_eq!(codegen.to_snake_case("ID"), "id");
        assert_eq!(codegen.to_snake_case("API"), "api");
    }

    #[test]
    fn test_trait_generation() {
        let temp_dir = TempDir::new().unwrap();

        // Create abstract base schema
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
  description:
    label: Description
    type: text
"#,
        );

        // Create abstract intermediate schema
        create_test_schema(
            temp_dir.path(),
            "LegalEntity",
            r#"
label: Legal Entity
abstract: true
extends:
  - Thing
properties:
  country:
    label: Country
    type: country
"#,
        );

        // Create concrete schemas
        create_test_schema(
            temp_dir.path(),
            "Person",
            r#"
label: Person
extends:
  - LegalEntity
properties:
  firstName:
    label: First Name
    type: name
"#,
        );

        create_test_schema(
            temp_dir.path(),
            "Company",
            r#"
label: Company
extends:
  - LegalEntity
properties:
  registrationNumber:
    label: Registration Number
    type: identifier
"#,
        );

        let registry = SchemaRegistry::load_from_cache(temp_dir.path()).unwrap();
        let output_dir = temp_dir.path().join("generated");
        let codegen = CodeGenerator::new(registry, &output_dir);

        let result = codegen.generate_all();
        assert!(result.is_ok(), "Code generation failed: {:?}", result);

        // Verify traits were generated
        let traits_content = fs::read_to_string(output_dir.join("traits.rs")).unwrap();
        assert!(traits_content.contains("pub trait Thing"));
        assert!(traits_content.contains("pub trait LegalEntity"));
        assert!(traits_content.contains("fn name(&self)"));
        assert!(traits_content.contains("fn country(&self)"));

        // Verify trait implementations were generated
        let trait_impls_content = fs::read_to_string(output_dir.join("trait_impls.rs")).unwrap();
        assert!(trait_impls_content.contains("impl Thing for Person"));
        assert!(trait_impls_content.contains("impl LegalEntity for Person"));
        assert!(trait_impls_content.contains("impl Thing for Company"));
        assert!(trait_impls_content.contains("impl LegalEntity for Company"));

        // Verify concrete structs still exist with flat structure
        let entities_content = fs::read_to_string(output_dir.join("entities.rs")).unwrap();
        assert!(entities_content.contains("pub struct Person"));
        assert!(entities_content.contains("pub struct Company"));
        assert!(entities_content.contains("pub name: Option<Vec<String>>")); // Flattened from Thing
        assert!(entities_content.contains("pub country: Option<Vec<String>>")); // Flattened from LegalEntity
    }
}
