// exact_format/src/lib.rs
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Expr, LitStr, Token, parse::{Parse, ParseStream}, punctuated::Punctuated};

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
    Value(Expr),
}

impl Parse for Replacement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let from = input.parse()?;
        let _arrow = input.parse()?;
        let to = input.parse()?;
        
        Ok(Replacement {
            from,
            _arrow,
            to,
        })
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
            },
            FormatPart::Value(expr) => {
                expr.to_tokens(tokens);
            },
        }
    }
}

#[proc_macro]
pub fn exact_format(input: TokenStream) -> TokenStream {
    let ExactFormat { template, replacements } = parse_macro_input!(input as ExactFormat);
    
    let mut parts = vec![FormatPart::Literal(template.value())];
    
    for replacement in replacements.iter() {
        let key = replacement.from.value();
        let value = &replacement.to;
        
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
                                new_parts.push(FormatPart::Value((*value).clone()));
                            }
                        }
                    } else {
                        new_parts.push(FormatPart::Literal(text));
                    }
                },
                FormatPart::Value(expr) => {
                    new_parts.push(FormatPart::Value(expr));
                },
            }
        }
        
        parts = new_parts;
    } 
    
    let expanded = if parts.is_empty() {
        quote! { #template.to_string() }
    } else {
        let format_str = "{}".repeat(parts.len());
        let format_lit = syn::LitStr::new(&format_str, proc_macro2::Span::call_site());
        quote! { format!(#format_lit, #(#parts),*) }
    };
    
    expanded.into()
}