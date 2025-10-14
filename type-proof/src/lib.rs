//! A crate for type-checked propositional logic proofs.

extern crate self as type_proof; // so that the path to this crate works in macros when used in this crate

pub mod boolean;
pub mod formula;
pub mod peano;
pub mod proof;
pub mod type_utils;
