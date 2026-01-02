//! Aileron
//!
//! A toolkit to use avro schemas as rust types.

use proc_macro::TokenStream;
use rsgen_avro::{GeneratorBuilder, ImplementAvroSchema, Source};
use syn::{
    Expr, Ident, LitStr, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct IncludeAvroInput {
    path: String,
    builder: GeneratorBuilder,
}

impl Parse for IncludeAvroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let path = input.parse::<LitStr>()?.value();
        let mut builder = GeneratorBuilder::default();

        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }

            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let val: Expr = input.parse()?;

            match key.to_string().as_str() {
                "precision" => builder = builder.precision(parse_usize(val)?),
                "use_avro_rs_unions" => builder = builder.use_avro_rs_unions(parse_bool(val)?),
                "use_chrono_dates" => builder = builder.use_chrono_dates(parse_bool(val)?),
                "derive_builders" => builder = builder.derive_builders(parse_bool(val)?),
                "extra_derives" => builder = builder.extra_derives(parse_vec(val)?),
                "impl_avro_schema" => builder = builder.implement_avro_schema(parse_enum(val)?),
                "override_fields" => unimplemented!("`override_fields` is not implemented yet"),
                "override_field" => unimplemented!("`override_field` is not implemented yet"),
                _ => {
                    return Err(syn::Error::new(key.span(), format!("Unknown flag: {key}")));
                }
            }
        }

        Ok(Self { path, builder })
    }
}

fn parse_bool(expr: Expr) -> Result<bool> {
    if let Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Bool(b),
        ..
    }) = expr
    {
        Ok(b.value)
    } else {
        Err(syn::Error::new_spanned(
            expr,
            "Expected boolean (true/false)",
        ))
    }
}

fn parse_usize(expr: Expr) -> Result<usize> {
    if let Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Int(i),
        ..
    }) = expr
    {
        i.base10_parse()
    } else {
        Err(syn::Error::new_spanned(expr, "Expected integer"))
    }
}

fn parse_vec(expr: Expr) -> Result<Vec<String>> {
    if let Expr::Array(arr) = expr {
        let mut out = Vec::new();
        for elem in arr.elems {
            if let Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(s),
                ..
            }) = elem
            {
                out.push(s.value());
            }
        }
        Ok(out)
    } else {
        Err(syn::Error::new_spanned(
            expr,
            "Expected array of strings: [\"A\", \"B\"]",
        ))
    }
}

fn parse_enum(expr: Expr) -> Result<ImplementAvroSchema> {
    let ident = match expr {
        Expr::Path(p) => p.path.segments.last().unwrap().ident.clone(),
        _ => return Err(syn::Error::new_spanned(expr, "Expected Enum variant")),
    };

    match ident.to_string().as_str() {
        "None" => Ok(ImplementAvroSchema::None),
        "Derive" => Ok(ImplementAvroSchema::Derive),
        "CopyBuildSchema" => Ok(ImplementAvroSchema::CopyBuildSchema),
        _ => Err(syn::Error::new(
            ident.span(),
            "Unknown variant for ImplementAvroSchema",
        )),
    }
}

/// Includes an avro schema into compatible types.
///
/// This macro just converts all the schema into a valid and compatible rust definitions for that
/// schema.
///
/// ```rust
/// aileron::include_avro!("schemas/person.avsc");
/// ```
///
/// This also supports globbing i.e, you could just pass `"*.avsc"` and it will get all
/// the files that matches with the pattern:
/// ```rust
/// aileron::include_avro!("schemas/*.avsc");
/// ```
///
/// This supports all customizations supported by
/// [rsgen_avro::GeneratorBuilder](https://docs.rs/rsgen-avro/latest/rsgen_avro/struct.GeneratorBuilder.html), except `override_fields`,
/// you could just pass it as an argument.
/// This will make the `Person` struct implement both `rkyv::Serialize` and `serde::Serialize`
/// (which is derived by default):
/// ```rust
///    aileron::include_avro!(
///       "tests/person.avsc",
///       precision = 4,
///       impl_avro_schema = Derive,
///       derive_builders = true,
///       use_chrono_dates = true,
///       use_avro_rs_unions = true,
///       extra_derives = ["rkyv::Archive", "rkyv::Serialize", "Default"],
///   );
///
/// ```
#[proc_macro]
pub fn include_avro(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as IncludeAvroInput);
    let source = Source::GlobPattern(&input.path);
    let mut buffer = vec![];

    input
        .builder
        .build()
        .expect("Could not initialize avro generator")
        .generate(&source, &mut buffer)
        .expect("Could not generate type definitions");

    String::from_utf8(buffer)
        .expect("Buffer is not a valid UTF-8 String")
        .parse()
        .expect("Could not parse the generated string into code")
}
