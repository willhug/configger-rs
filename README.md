# configger-rs

This repo is very much WIP lol.

The goal of this library is to be a building block for building descriptive and flexible
configuration that can be used in multiple contexts.  One piece I want to very much lean
into is configuration as code.  I also want to make it easy to define sane defaults.

So much configuration needs to live in multiple locations.  Configger should try and solve
this by having a flexible definition language that can generate any config needed.

This is a bit greenfield for me as well, but, I think a flexible data definition language,
combined with a flexible plugin system could potentially open the door for this.

## Data definition language:

Ideally this should be a flexible graph definition of types/definitions you'd want to use for
how you configure your types.

E.g. This could be an example data definition for a database:
```
Schema: {name: String, desc: String: models: []Model}
Model:  {name: String, .. fields: []Field}
Field:  {name: String, type: String?, ..}
```

The data definition language should be flexible enough to be able to extend any of the types
listed here to have more information.  An example could be a plugin wanting to specify an
override of a field to make it nullable could write a NullableField that _extends_ Field.

## Plugins

Plugins should be thought of as the "sinks" for the data defined language.  They can list
a set of dependencies they have (e.g. Schema, Model, Field) and will be given types to
generate code (or whatever) using the dependencies as a base. (Note a part of this might be
trying to get better codegen tools if needed).  Plugins can also write validation to ensure
that the configuration is valid (e.g. if someone deleted a database column).

## The "Schema"

To actually use a set of data definitions and plugins, the user will create a "Schema" of
their configuration.  So a user might say "I want be able to define models that goes into
a sql database, and can be exposed via GraphQL, JSON and Protobuf", which could look
something like this:

```
builder = configger::new!(ModelSQLDatabase, ModelGraphQL, ModelProto, ModelJSON)
```

Which would generate a new schema that could be build.

## Using the schema

The schema should be programatically used by building it out.

```
let mut builder = configger::new!(ModelSQLDatabase, ModelGraphQL, ModelProto, ModelJSON);

let mut schema = builder.new_schema("default")
schema.no_protobuf();

let mut users = schema.new_model("users").set_partitioned();

users.int_field("id").primary();
users.str_field("name").max_size(100);
...

builder.build()
```

Ideally a user can also implement "sane default" functions.  So a company that uses
snowflakes internally could write a function:

```
users
    .snowflake_field("id")
    .autogenerate() // Automatically calls into our own snowflake service
```

And it should be very easy to do that.  It will need to be supported in the plugins,
so the plugin API needs to be super clean & easy to add new use cases (and testable)

## Use cases

Use cases I'd like to help solve:

- Model definition:
    - Protobuf definitions
    - Database schemas
    - Sane validation
    - GraphQL exposure
    - Privacy (would be harder to do)


- Event/Logging definition:
    - Protobuf definitions
    - "Which sinks should this go to?"
    - Sane validation


- Service definition
    - GCP/AWS/etc
    - Kubernetes definition
    - Local/Staging/Production config
    - Autoscaling policies


- Configuration rollout
    - Feature flags
    - Percent rollouts
    - Rollout values
    - min/max values
    - enforce values must be increasing (unless forced)

## Thoughts

- Mostly thinking of plugins doing codegen, but, could they be macros for rust code?
- How will extensions to the API work?  Is there much to the "core" here?  Or will most of the work be the data & plugins?
- Will there be a set of "core" functionality, how can we make building that seamless.
- Is the macro engine in rust strong enough to handle merging the types together.
- Can we setup snapshot tests in rust?