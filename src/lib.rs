#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Expr, Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
};

struct ExactFormat {
    template: LitStr,
    replacements: Punctuated<Replacement, Token![,]>,
}

struct Replacement {
    from: LitStr,
    _arrow: Token![=>],
    to: Expr,
}

/// Enum to represent either a string literal part or a replacement value
enum FormatPart {
    Literal(String),
    Value(Ident),
}

impl Parse for Replacement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let from = input.parse()?;
        let _arrow = input.parse()?;
        let to = input.parse()?;

        Ok(Replacement { from, _arrow, to })
    }
}

impl Parse for ExactFormat {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let template = input.parse()?;
        let _: Token![,] = input.parse()?;

        let replacements = Punctuated::parse_terminated(input)?;

        Ok(ExactFormat {
            template,
            replacements,
        })
    }
}

impl ToTokens for FormatPart {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            FormatPart::Literal(s) => {
                let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                lit.to_tokens(tokens);
            }
            FormatPart::Value(expr) => {
                expr.to_tokens(tokens);
            }
        }
    }
}

/// Macro to replace placeholders in a string with values at compile time.
#[proc_macro]
pub fn exact_format(input: TokenStream) -> TokenStream {
    let ExactFormat {
        template,
        replacements,
    } = parse_macro_input!(input as ExactFormat);

    let mut parts = vec![FormatPart::Literal(template.value())];
    let mut values = Vec::new();

    for (index, replacement) in replacements.iter().enumerate() {
        let key = replacement.from.value();
        let value = &replacement.to;
        let value_name = format!("__value{}__", index);
        let value_ident = syn::Ident::new(&value_name, value.span());
        values.push(quote! { let #value_ident = #value; });

        let mut new_parts = Vec::new();

        for part in parts {
            match part {
                FormatPart::Literal(text) => {
                    if text.contains(&key) {
                        let split_parts: Vec<&str> = text.split(&key).collect();

                        for (i, split_part) in split_parts.iter().enumerate() {
                            if !split_part.is_empty() {
                                new_parts.push(FormatPart::Literal(split_part.to_string()));
                            }

                            if i < split_parts.len() - 1 {
                                new_parts.push(FormatPart::Value(value_ident.clone()));
                            }
                        }
                    } else {
                        new_parts.push(FormatPart::Literal(text));
                    }
                }
                FormatPart::Value(expr) => {
                    new_parts.push(FormatPart::Value(expr));
                }
            }
        }

        parts = new_parts;
    }

    let expanded = if parts.is_empty() {
        quote! { { #(#values)* #template.to_string() } }
    } else {
        let format_str = "{}".repeat(parts.len());
        let format_lit = syn::LitStr::new(&format_str, proc_macro2::Span::call_site());
        quote! { { #(#values)* format!(#format_lit, #(#parts),*) } }
    };

    expanded.into()
}
