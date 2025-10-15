//! Macros for the [`type_proof`] crate.

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
///     formula::{P3, var},
///     type_utils::assert_type_eq,
/// };
///
/// assert_type_eq::<var!(3), P3>();
/// ```
#[proc_macro]
pub fn var(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitInt);

    let nat = quote! { ::type_proof_macros::N!(#input) };
    let output = quote! { ::type_proof::formula::Var<#nat> };

    TokenStream::from(output)
}
