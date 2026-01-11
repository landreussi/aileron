use std::{env, fs, path::Path};

use rsgen_avro::{FieldOverride, GeneratorBuilder, ImplementAvroSchema, Result, Source};

#[cfg(feature = "macro")]
pub use avrogant_macro::include_avro;

pub struct AvroCompiler {
    generator: GeneratorBuilder,
}

impl Default for AvroCompiler {
    fn default() -> Self {
        Self {
            generator: GeneratorBuilder::new(),
        }
    }
}

impl AvroCompiler {
    /// Initializes a Builder with default attributes.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the precision used for decimal logical types.
    ///
    /// This controls how many digits are preserved when generating
    /// Rust types for Avro decimal fields.
    pub fn precision(mut self, precision: usize) -> Self {
        self.generator = self.generator.precision(precision);
        self
    }

    /// Enables or disables nullable fields.
    ///
    /// When enabled, fields that are unions with `null` will be
    /// generated as `Option<T>`.
    pub fn nullable(mut self, nullable: bool) -> Self {
        self.generator = self.generator.nullable(nullable);
        self
    }

    /// Uses `avro_rs::types::Value::Union` instead of `Option<T>`.
    ///
    /// This is useful when you need full control over union handling
    /// rather than Rust-native option types.
    pub fn use_avro_rs_unions(mut self, use_avro_rs_unions: bool) -> Self {
        self.generator = self.generator.use_avro_rs_unions(use_avro_rs_unions);
        self
    }

    /// Enables generation of chrono-based date and time types.
    ///
    /// Logical Avro date/time types will map to `chrono` types
    /// instead of primitive integers.
    pub fn use_chrono_dates(mut self, use_chrono_dates: bool) -> Self {
        self.generator = self.generator.use_chrono_dates(use_chrono_dates);
        self
    }

    /// Enables or disables deriving builder structs for generated types.
    ///
    /// When enabled, a builder pattern will be generated alongside
    /// the main structs.
    ///
    /// Your code should depend on [`derive_builder`] crate.
    pub fn derive_builders(mut self, derive_builders: bool) -> Self {
        self.generator = self.generator.derive_builders(derive_builders);
        self
    }

    /// Controls whether Avro schema implementations are generated.
    ///
    /// Your code should depend on [`apache-avro`] crate.
    pub fn implement_avro_schema(mut self, impl_schemas: ImplementAvroSchema) -> Self {
        self.generator = self.generator.implement_avro_schema(impl_schemas);
        self
    }

    /// Adds additional `#[derive(...)]` attributes to generated types.
    pub fn extra_derives(mut self, extra_derives: Vec<String>) -> Self {
        self.generator = self.generator.extra_derives(extra_derives);
        self
    }

    /// Overrides multiple fields in generated structs.
    pub fn override_fields(mut self, overrides: Vec<FieldOverride>) -> Self {
        self.generator = self.generator.override_fields(overrides);
        self
    }

    /// Overrides a single field in a generated struct.
    pub fn override_field(mut self, over: FieldOverride) -> Self {
        self.generator = self.generator.override_field(over);
        self
    }

    /// Generate rust files containing the types for the received schemas.
    pub fn compile(self, schemas: &[impl AsRef<Path>]) -> Result<()> {
        let generator = self.generator.build()?;
        for schema in schemas {
            let mut buf = vec![];

            let path = schema.as_ref();
            let schema = path
                .to_str()
                .expect("Schema path is not a valid UTF-8 string");
            let file_name = path.file_prefix().expect("Invalid schema path, ensure it has a file name with it's extension (e.g.: foo_*_bar.avsc)");
            let output_dir =
                env::var_os("OUT_DIR").expect("OUT_DIR environment variable is not present");
            let path = Path::new(&output_dir).join(file_name).join(".rs");

            generator.generate(&Source::GlobPattern(schema), &mut buf)?;
            let content = String::from_utf8(buf)
                .expect("Content of schema files is not a valid UTF-8 string");
            fs::write(&path, content)?;

            println!("cargo::rerun-if-changed={}", schema);
        }

        Ok(())
    }
}
