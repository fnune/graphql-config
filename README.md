# graphql-config

This crate provides deserialize for GraphQL configs following the [graphql-config](https://github.com/prismagraphql/graphql-config/blob/master/specification.md) specification.

`GraphQLConfiguration` is the type of the whole JSON document. It contains the top-level configuration (which serializes in the `root` field) and also optionally project-specific configuration in the `projects` field. The shapes of the top-level configuration and project-specific configurations are exactly the same.

This library does not support [experimental configuration options](https://github.com/prismagraphql/graphql-config/blob/master/specification.md#experimental-configuration-options) yet.

Currently, this library follows the spec as per [version 2.0.1 of the graphql-config specification](https://github.com/prismagraphql/graphql-config/tree/v2.0.1).
