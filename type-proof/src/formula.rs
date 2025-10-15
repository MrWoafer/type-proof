//! Defines propositional formulas as types.
//!
//! # Language
//!
//! Our logical language consists of the the following symbols:
//! - Propositional variables `p0, p1, p2, ...`
//! - Logical negation `¬`
//! - Logical implication `->`
//! - Parentheses `( )`
//!
//! Note that `{¬, ->}` is a [functionally complete](https://en.wikipedia.org/wiki/Functional_completeness)
//! set of Boolean operators, so other common operators (e.g. `∧, ∨`) can be expressed in terms of them.
//!
//! # Formulas
//!
//! A propositional formula is defined precisely using the following rules:
//!
//! - All propositional variables (type [`Var<N>`])
//! - `¬P`, where `P` is a formula (type [`Not<P>`])
//! - `(P -> Q)` where `P, Q` are formulas (type [`Implies<P, Q>`])
//!
//! The parentheses around `P -> Q` are to ensure unique readability / make operator precedence explicit. This
//! also means that there's a simple bijection between the Rust types defined above and well-formed formulas in
//! our language.
//!
//! There are some predefined type aliases for [`Var<N>`] for the first few natural numbers - for example, [`P0`]
//! is an alias for [`Var<N0>`]. Propositional variables with arbitrarily large indices can be created using the
//! [`P!`] macro.
//!
//! Some operators expressable in terms of `¬, ->` are defined via type aliases - for example, `∧` has the alias
//! [`And<P, Q>`].
//!
//! ## Example
//!
//! The formula `(p0 -> ¬p2)` can be defined as a type like so:
//!
//! ```
//! use type_proof::formula::{Implies, Not, P0, P2};
//!
//! type P = Implies<P0, Not<P2>>;
//! ```
//!
//! # Valuations
//!
//! Given an assignment of [`True`] or [`False`] to each propositional variable in a formula, the formula can then
//! be assigned a value of [`True`] or [`False`], using the following rules:
//!
//! ```text
//! |   P   |  ¬P   |
//! | ----- | ----- |
//! | False | True  |
//! | True  | False |
//!
//! |   P   |   Q   | (P -> Q) |
//! | ----- | ----- | -------- |
//! | False | False |   True   |
//! | False | True  |   True   |
//! | True  | False |   False  |
//! | True  | True  |   True   |
//! ```
//!
//! Valuations can be done via the [`Valuation`] trait.
//!
//! ## Example
//!
//! To find the value of `(p0 -> ¬p2)` under the assignment `p0 = True, p2 = False`:
//!
//! ```
//! use type_proof::{
//!     boolean::{False, True},
//!     formula::{Implies, Not, P0, P2, Valuation},
//!     type_utils::assert_type_eq,
//! };
//!
//! // The formula (p0 -> ¬p2)
//! type P = Implies<P0, Not<P2>>;
//!
//! // Our valuation
//! struct V;
//! impl Valuation<P0> for V {
//!     type Value = True;
//! }
//! impl Valuation<P2> for V {
//!     type Value = False;
//! }
//!
//! // V automatically implements Valuation<P> given Valuation<P0> and Valuation<P2>
//! type ValueOfP = <V as Valuation<P>>::Value;
//! assert_type_eq::<ValueOfP, True>();
//! ``````
//!
//! [`True`]: crate::boolean::True
//! [`False`]: crate::boolean::False

use std::marker::PhantomData;

use crate::{
    boolean::{self, Bool},
    peano::{N0, N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, Nat},
};

pub use type_proof_macros::P;

/// A propositional formula, encoded as a type.
pub trait Formula {
    /// Displays the formula as a string.
    fn display() -> String;
}

/// A propositional variable.
///
/// `N` is the index of the propositional variable.
pub struct Var<N>
where
    N: Nat,
{
    _n: PhantomData<N>,
}

impl<N> Formula for Var<N>
where
    N: Nat,
{
    fn display() -> String {
        format!("p{}", N::VALUE)
    }
}

/// The propositional variable `p0`.
pub type P0 = Var<N0>;

/// The propositional variable `p1`.
pub type P1 = Var<N1>;

/// The propositional variable `p2`.
pub type P2 = Var<N2>;

/// The propositional variable `p3`.
pub type P3 = Var<N3>;

/// The propositional variable `p4`.
pub type P4 = Var<N4>;

/// The propositional variable `p5`.
pub type P5 = Var<N5>;

/// The propositional variable `p6`.
pub type P6 = Var<N6>;

/// The propositional variable `p7`.
pub type P7 = Var<N7>;

/// The propositional variable `p8`.
pub type P8 = Var<N8>;

/// The propositional variable `p9`.
pub type P9 = Var<N9>;

/// The propositional variable `p10`.
pub type P10 = Var<N10>;

/// Logical negation:
/// `¬P`
pub struct Not<P>
where
    P: Formula,
{
    _p: PhantomData<P>,
}

impl<P> Formula for Not<P>
where
    P: Formula,
{
    fn display() -> String {
        format!("¬{}", P::display())
    }
}

/// Logical implication:
/// `P -> Q`
pub struct Implies<P, Q>
where
    P: Formula,
    Q: Formula,
{
    _p: PhantomData<P>,
    _q: PhantomData<Q>,
}

impl<P, Q> Formula for Implies<P, Q>
where
    P: Formula,
    Q: Formula,
{
    fn display() -> String {
        format!("({} -> {})", P::display(), Q::display())
    }
}

/// Logical conjunction (and):
/// `P ∧ Q`
#[allow(type_alias_bounds)]
pub type And<P, Q>
where
    P: Formula,
    Q: Formula,
= Not<Implies<P, Not<Q>>>;

/// Logical disjunction (or):
/// `P ∨ Q`
#[allow(type_alias_bounds)]
pub type Or<P, Q>
where
    P: Formula,
    Q: Formula,
= Not<And<Not<P>, Not<Q>>>; // using De Morgan's laws

/// Logical equivalence:
/// `P <-> Q`
#[allow(type_alias_bounds)]
pub type Iff<P, Q>
where
    P: Formula,
    Q: Formula,
= And<Implies<P, Q>, Implies<Q, P>>;

/// Deduces the value of an expression given the value of each of its propositional variables.
///
/// A valuation type should implement [`Valuation`] for each of its propositional variables, and
/// [`Valuation`] will automatically be implemented for all formulas constructible from those variables.
///
/// # Example
///
/// See the [module documentation](self).
pub trait Valuation<P>
where
    P: Formula,
{
    /// The value of the valuation on `P`.
    type Value: Bool;
}

impl<V, P> Valuation<Not<P>> for V
where
    P: Formula,
    V: Valuation<P>,
{
    type Value = boolean::Not<<V as Valuation<P>>::Value>;
}

impl<V, P, Q> Valuation<Implies<P, Q>> for V
where
    P: Formula,
    Q: Formula,
    V: Valuation<P> + Valuation<Q>,
    // This bound is needed because Rust doesn't know that only True and False implement Bool
    <V as Valuation<P>>::Value: boolean::ImpliesImpl<<V as Valuation<Q>>::Value>,
{
    type Value = boolean::Implies<<V as Valuation<P>>::Value, <V as Valuation<Q>>::Value>;
}

#[cfg(test)]
mod tests {
    use crate::{
        boolean::{False, True},
        type_utils::assert_type_eq,
    };

    use super::*;

    #[test]
    fn display() {
        type F = Not<Implies<P0, Not<P2>>>;
        assert_eq!(F::display(), "¬(p0 -> ¬p2)");
    }

    #[test]
    fn valuation() {
        // ¬(p0 -> ¬p2)
        type P = Not<Implies<P0, Not<P2>>>;

        struct V1;
        impl Valuation<P0> for V1 {
            type Value = True;
        }
        impl Valuation<P2> for V1 {
            type Value = False;
        }

        type PVal1 = <V1 as Valuation<P>>::Value;
        assert_type_eq::<PVal1, False>();

        struct V2;
        impl Valuation<P0> for V2 {
            type Value = False;
        }
        impl Valuation<P2> for V2 {
            type Value = True;
        }

        type PVal2 = <V2 as Valuation<P>>::Value;
        assert_type_eq::<PVal2, False>();
    }

    #[test]
    fn var_macro() {
        assert_type_eq::<P!(0), P0>();
        assert_type_eq::<P!(3), P3>();
    }
}
