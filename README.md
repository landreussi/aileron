# Avrogant

## A toolkit to use Avro schemas as Rust types.

### Rationale:

This crate is suitable when you need to generate a 100% compatible rust types for an avro schema.
It will transform the entire schema in types, so it will make your code depend on the schema to actually compile.
This makes the struct mapping from a avro schema more ergonomic and less error-prone.

Think of it like [`tonic`](https://github.com/hyperium/tonic), but for avro instead of protocol buffers.

### Use cases:

You could convert avro schemas into rust types by calling the `include_avro` macro:
```rust filename="avro/person.rs"
avrogant::include_avro!("schemas/person.avsc");
```

or you could build those types automatically using build scripts:
```rust filename="build.rs"
fn main() {
    avrogant::AvroCompiler::new()
        .compile(&["schemas/person.avsc"])
        .unwrap();
}
```


and then include it on your code:
```rust filename="src/main.rs"
use crate::avro::Person;

mod avro {
    include!(concat!(env!("OUT_DIR"), "/person.rs"));
}
```

You could customize the way your type is generated, like deriving more traits in the generated types.
Please refer to the [docs](https://docs.rs/avrogant/latest/avrogant) to see more.
