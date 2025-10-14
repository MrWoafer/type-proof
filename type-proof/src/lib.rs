#![deny(
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    rustdoc::missing_crate_level_docs,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::bare_urls,
    rustdoc::unescaped_backticks
)]
#![warn(missing_docs)]

//! A crate for type-checked propositional logic proofs.

extern crate self as type_proof; // so that the path to this crate works in macros when used in this crate

pub mod boolean;
pub mod formula;
pub mod peano;
pub mod proof;
pub mod type_utils;
