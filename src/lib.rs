//! This crate provides deserialize for GraphQL configs following the [graphql-config](https://github.com/prismagraphql/graphql-config/blob/master/specification.md) specification.
//!
//! `GraphQLConfiguration` is the type of the whole JSON document. It contains the top-level configuration (which serializes in the `root` field) and also optionally project-specific configuration in the `projects` field. The shapes of the top-level configuration and project-specific configurations are exactly the same.
//!
//! This library does not support [experimental configuration options](https://github.com/prismagraphql/graphql-config/blob/master/specification.md#experimental-configuration-options) yet.
//!
//! Currently, this library follows the spec as per [version 2.0.1 of the graphql-config specification](https://github.com/prismagraphql/graphql-config/tree/v2.0.1).
//!
//! ## Example
//!
//! ```
//! # extern crate serde;
//! # extern crate graphql_config;
//! # #[macro_use]
//! # extern crate serde_json;
//! # #[macro_use]
//! # extern crate maplit;
//! # use graphql_config::*;
//! # use std::io;
//! # fn main() -> io::Result<()> {
//! let config = json!({
//!     "schemaPath": "./schema.graphql",
//!     "includes": ["./graphql/*.graphql"],
//!     "projects": {                 
//!         "amazingLibrary": {
//!             "schemaPath": "./amazingLibrary.schema.graphql"
//!         }
//!     }
//! });
//!
//! let expected = GraphQLConfiguration {
//!     root: GraphQLProjectConfiguration {
//!         name: None,
//!         schema_path: Some("./schema.graphql".into()),
//!         includes: Some(vec!["./graphql/*.graphql".to_owned()]),
//!         excludes: None,
//!         extensions: None,
//!     },
//!     projects: Some(btreemap!{
//!         "amazingLibrary".to_owned() => GraphQLProjectConfiguration {
//!             schema_path: Some("./amazingLibrary.schema.graphql".into()),
//!             name: None,
//!             includes: None,
//!             excludes: None,
//!             extensions: None,
//!         },
//!     }),
//! };
//!
//! let deserialized = serde_json::from_value::<GraphQLConfiguration>(config)?;
//!
//! assert_eq!(deserialized, expected);
//! # Ok(())
//! # }
//! ```

#![deny(missing_docs)]

extern crate serde;

#[cfg(not(test))]
extern crate serde_json;

#[cfg(test)]
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
#[macro_use]
extern crate maplit;

/// `GraphQLConfiguration` is the type of the whole JSON document. It contains
/// the top-level configuration (which serializes in the `root` field) and also
/// optionally project-specific configuration in the `projects` field. The shapes
/// of the top-level configuration and project-specific configurations are exactly
/// the same.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GraphQLConfiguration {
    /// A `BTreeMap` of project names as strings to `GraphQLProjectConfiguration`.
    /// Names of projects are not snake-cased during deserialization.
    pub projects: Option<::std::collections::BTreeMap<String, GraphQLProjectConfiguration>>,
    /// Top-level configuration goes into `root`.
    #[serde(flatten)]
    pub root: GraphQLProjectConfiguration,
}

/// The top-level configuration and project-specific
/// configurations share this shape.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLProjectConfiguration {
    /// The name of the project. The specification says this should default to
    /// the key of the project object if absent, this this not enforced.
    pub name: Option<String>,
    /// A file with schema IDL.
    pub schema_path: Option<::std::path::PathBuf>,
    /// For multiple applications with overlapping files,
    /// these configuration options may be helpful.
    pub includes: Option<Vec<String>>,
    /// For multiple applications with overlapping files,
    /// these configuration options may be helpful.
    pub excludes: Option<Vec<String>>,
    /// If you'd like to specify any other configurations,
    /// graphql-config provides a reserved namespace for it.
    pub extensions: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_deserialization(json: serde_json::Value, expected: GraphQLConfiguration) {
        let deserialized = serde_json::from_value::<GraphQLConfiguration>(json).unwrap();

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn it_works_with_schema_path() {
        let config = json!({
            "schemaPath": "./schema.graphql"
        });

        let expected = GraphQLConfiguration {
            root: GraphQLProjectConfiguration {
                name: None,
                schema_path: Some("./schema.graphql".into()),
                includes: None,
                excludes: None,
                extensions: None,
            },
            projects: None,
        };

        test_deserialization(config, expected);
    }

    #[test]
    fn it_works_with_nested_project_configs() {
        let config = json!({
            "projects": {                 
                "amazingLibrary": {
                    "schemaPath": "./amazingLibrary.schema.graphql"
                }
            }
        });

        let expected = GraphQLConfiguration {
            root: GraphQLProjectConfiguration {
                name: None,
                schema_path: None,
                includes: None,
                excludes: None,
                extensions: None,
            },
            projects: Some(btreemap!{
                "amazingLibrary".to_owned() => GraphQLProjectConfiguration {
                    schema_path: Some("./amazingLibrary.schema.graphql".into()),
                    name: None,
                    includes: None,
                    excludes: None,
                    extensions: None,
                }
            }),
        };

        test_deserialization(config, expected);
    }

    #[test]
    fn it_works_with_multiple_nested_project_configs() {
        let config = json!({
            "projects": {                 
                "amazingLibrary": {
                    "schemaPath": "./amazingLibrary.schema.graphql"
                },
                "evenMoreAmazingLibrary": {
                    "schemaPath": "./evenMoreAmazingLibrary.schema.graphql"
                }
            }
        });

        let expected = GraphQLConfiguration {
            root: GraphQLProjectConfiguration {
                name: None,
                schema_path: None,
                includes: None,
                excludes: None,
                extensions: None,
            },
            projects: Some(btreemap!{
                "amazingLibrary".to_owned() => GraphQLProjectConfiguration {
                    schema_path: Some("./amazingLibrary.schema.graphql".into()),
                    name: None,
                    includes: None,
                    excludes: None,
                    extensions: None,
                },
                "evenMoreAmazingLibrary".to_owned() => GraphQLProjectConfiguration {
                    schema_path: Some("./evenMoreAmazingLibrary.schema.graphql".into()),
                    name: None,
                    includes: None,
                    excludes: None,
                    extensions: None,
                }
            }),
        };

        test_deserialization(config, expected);
    }

    #[test]
    fn it_works_with_multiple_nested_project_configs_and_a_root_config() {
        let config = json!({
            "schemaPath": "./greatRootLibrary.schema.graphql",
            "projects": {                 
                "amazingLibrary": {
                    "schemaPath": "./amazingLibrary.schema.graphql"
                },
                "evenMoreAmazingLibrary": {
                    "schemaPath": "./evenMoreAmazingLibrary.schema.graphql"
                }
            }
        });

        let expected = GraphQLConfiguration {
            root: GraphQLProjectConfiguration {
                name: None,
                schema_path: Some("./greatRootLibrary.schema.graphql".into()),
                includes: None,
                excludes: None,
                extensions: None,
            },
            projects: Some(btreemap!{
                "amazingLibrary".to_owned() => GraphQLProjectConfiguration {
                    schema_path: Some("./amazingLibrary.schema.graphql".into()),
                    name: None,
                    includes: None,
                    excludes: None,
                    extensions: None,
                },
                "evenMoreAmazingLibrary".to_owned() => GraphQLProjectConfiguration {
                    schema_path: Some("./evenMoreAmazingLibrary.schema.graphql".into()),
                    name: None,
                    includes: None,
                    excludes: None,
                    extensions: None,
                }
            }),
        };

        test_deserialization(config, expected);
    }

    #[test]
    fn it_works_with_extensions() {
        let config = json!({
            "extensions": {
                "lastUpdatedAt": 1532367255884u64
            }
        });

        let expected = GraphQLConfiguration {
            root: GraphQLProjectConfiguration {
                name: None,
                schema_path: None,
                includes: None,
                excludes: None,
                extensions: Some(
                    btreemap!{ "lastUpdatedAt".to_owned() => json!(1532367255884u64) },
                ),
            },
            projects: None,
        };

        test_deserialization(config, expected);
    }

    #[test]
    fn it_works_with_excludes_and_includes() {
        let config = json!({
            "includes": ["./projectA/graphql/*.graphql"],
            "excludes": ["./projectA/graphql/*.not_graphql"]
        });

        let expected = GraphQLConfiguration {
            root: GraphQLProjectConfiguration {
                name: None,
                schema_path: None,
                includes: Some(vec!["./projectA/graphql/*.graphql".to_owned()]),
                excludes: Some(vec!["./projectA/graphql/*.not_graphql".to_owned()]),
                extensions: None,
            },
            projects: None,
        };

        test_deserialization(config, expected);
    }
}
