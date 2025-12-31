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

#[proc_macro]
pub fn include_avro(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as IncludeAvroInput);
    let source = Source::GlobPattern(&input.path);
    let mut buffer = vec![];

    input
        .builder
        .build()
        .unwrap()
        .generate(&source, &mut buffer)
        .unwrap();
    String::from_utf8(buffer).unwrap().parse().unwrap()
}
