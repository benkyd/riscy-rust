use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, LitStr, Token};

struct ParsedInput {
    expression: Expr,
    _comma: Token![,],
    token_string: LitStr,
}

fn parse_input(input: syn::parse::ParseStream) -> syn::Result<ParsedInput> {
    Ok(ParsedInput {
        expression: input.parse()?,
        _comma: input.parse()?,
        token_string: input.parse()?,
    })
}

#[proc_macro]
pub fn match_mask(input: TokenStream) -> TokenStream {
    let parsed =
        syn::parse::Parser::parse(parse_input, input).unwrap();

    let input = parsed.expression;

    let tokens: String = parsed
        .token_string
        .value()
        .parse()
        .expect("Failed to parse token string");

    let mut ones: u32 = 0;
    let mut zeros: u32 = 0;

    for (idx, bit) in tokens.chars().rev().enumerate() {
        match bit {
            '1' => {
                ones |= 1 << idx;
                zeros |= 1 << idx;
            }
            '0' => {
                zeros |= 1 << idx;
            }
            _ => continue,
        }
    }

    let expanded = quote! {
        #input & #zeros ^ #ones == 0
    };

    TokenStream::from(expanded)
}
