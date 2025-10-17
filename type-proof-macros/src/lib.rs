#![deny(
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    rustdoc::missing_crate_level_docs,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::bare_urls,
    rustdoc::unescaped_backticks,
    missing_docs
)]

//! Macros for the `type-proof` crate.
//!
//! The macros are re-exported in `type-proof`, so it's recommended to just use them through that.

use proc_macro::TokenStream;
use quote::quote;
use syn::{LitInt, parse_macro_input};

/// Creates a typed version of the given natural number.
///
/// Useful for creating natural numbers beyond the hard-coded type aliases.
///
/// # Example
///
/// ```
/// use type_proof::{
///     peano::{N, N3},
///     type_utils::assert_type_eq,
/// };
///
/// assert_type_eq::<N!(3), N3>();
/// ```
#[proc_macro]
#[allow(non_snake_case)]
pub fn N(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitInt);
    let value = input
        .base10_parse::<usize>()
        .expect("natural numbers are non-negative");

    let mut output = quote! { ::type_proof::peano::N0 };
    for _ in 0..value {
        output = quote! { ::type_proof::peano::Succ<#output> };
    }

    TokenStream::from(output)
}

/// Creates a typed propositional variable with the given natural number as the index.
///
/// Useful for creating propositional variables with indices beyond the hard-coded type aliases.
///
/// # Example
///
/// ```
/// use type_proof::{
///     formula::{P, P3},
///     type_utils::assert_type_eq,
/// };
///
/// assert_type_eq::<P!(3), P3>();
/// ```
#[proc_macro]
#[allow(non_snake_case)]
pub fn P(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitInt);

    let nat = quote! { ::type_proof_macros::N!(#input) };
    let output = quote! { ::type_proof::formula::Var<#nat> };

    TokenStream::from(output)
}
