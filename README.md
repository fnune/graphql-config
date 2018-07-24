[![Build Status](https://travis-ci.org/brainlessdeveloper/graphql-config.svg?branch=master)](https://travis-ci.org/brainlessdeveloper/graphql-config)

# graphql-config

This crate provides deserialize for GraphQL configs following the [graphql-config](https://github.com/prismagraphql/graphql-config/blob/master/specification.md) specification.

`GraphQLConfiguration` is the type of the whole JSON document. It contains the top-level configuration (which serializes in the `root` field) and also optionally project-specific configuration in the `projects` field. The shapes of the top-level configuration and project-specific configurations are exactly the same.

This library does not support [experimental configuration options](https://github.com/prismagraphql/graphql-config/blob/master/specification.md#experimental-configuration-options) yet.

Currently, this library follows the spec as per [version 2.0.1 of the graphql-config specification](https://github.com/prismagraphql/graphql-config/tree/v2.0.1).

### Example

```rust
let config = json!({
    "schemaPath": "./schema.graphql",
    "includes": ["./graphql/*.graphql"],
    "projects": {
        "amazingLibrary": {
            "schemaPath": "./amazingLibrary.schema.graphql"
        }
    }
});

let expected = GraphQLConfiguration {
    root: GraphQLProjectConfiguration {
        name: None,
        schema_path: Some("./schema.graphql".into()),
        includes: Some(vec!["./graphql/*.graphql".to_owned()]),
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
    }),
};

let deserialized = serde_json::from_value::<GraphQLConfiguration>(config)?;

assert_eq!(deserialized, expected);
```

License: Apache-2.0 OR MIT
