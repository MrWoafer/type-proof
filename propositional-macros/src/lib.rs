//! Macros for the [`propositional`] crate.

use proc_macro::TokenStream;
use quote::quote;
use syn::{LitInt, parse_macro_input};

/// Creates a natural number at the type level.
#[proc_macro]
pub fn nat(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitInt);
    let value = input
        .base10_parse::<usize>()
        .expect("natural numbers are non-negative");

    let mut output = quote! { ::propositional::peano::N0 };
    for _ in 0..value {
        output = quote! { ::propositional::peano::Succ<#output> };
    }

    TokenStream::from(output)
}

/// Creates a propositional variable with the given natural number as the index.
#[proc_macro]
pub fn var(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitInt);

    let nat = quote! { ::propositional_macros::nat!(#input) };
    let output = quote! { ::propositional::formula::Variable<#nat> };

    TokenStream::from(output)
}
