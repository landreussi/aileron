# Aileron

## A customizable proc macro for converting Avro schemas into Rust types.

### Rationale:

This crate is suitable when you need to generate a 100% compatible rust types for an avro schema.
It will transform the entire schema in types, so it will make your code need the schema to actually compile.

This makes the struct mapping from a avro schema more ergonomic and less error-prone.
