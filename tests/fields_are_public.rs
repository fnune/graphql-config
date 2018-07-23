extern crate graphql_config;

use graphql_config::*;

#[macro_use]
extern crate serde_json;

fn test_deserialization(json: serde_json::Value, expected: GraphQLConfiguration) {
    let deserialized = serde_json::from_value::<GraphQLConfiguration>(json).unwrap();

    assert_eq!(deserialized, expected);
}

#[test]
fn it_surfaces_fields_as_public() {
    let config = json!({
        "schemaPath": "./schema.graphql",
        "name": "George"
    });

    let expected = GraphQLConfiguration {
        root: GraphQLProjectConfiguration {
            name: Some("George".to_owned()),
            schema_path: Some("./schema.graphql".into()),
            includes: None,
            excludes: None,
            extensions: None,
        },
        projects: None,
    };

    test_deserialization(config, expected);
}
