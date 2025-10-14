//! Encodes proofs as types.
//!
//! # Proof system
//!
//! We a Hilbert proof system known as [<code>P<sub>2</sub></code>](https://en.wikipedia.org/wiki/Hilbert_system#Schematic_form_of_P2)
//! (no relation to our propositional variable type alias [`P2`]).
//!
//! We write `⊢ P` to mean that our proof system proves the formula `P`. We also say that `P` is a *theorem*.
//!
//! ## Axioms
//!
//! We have three axiom schemas: for all formulas `P, Q, R`,
//!
//! 1) `⊢ P -> (Q -> P)`
//! 2) `⊢ (P -> (Q -> R)) -> ((P -> Q) -> (P -> R))`
//! 3) `⊢ (¬Q -> ¬P) -> (P -> Q)`
//!
//! (Technical point: we have omitted the parentheses round the outer `->` clauses to improve readability.)
//!
//! These respectively have the type aliases [`Axiom1<P, Q>`], [`Axiom2<P, Q, R>`], [`Axiom3<P, Q>`].
//!
//! ## Inference rules
//!
//! We have one inference rule, called [modus ponens](https://en.wikipedia.org/wiki/Modus_ponens):
//!
//! For all formulas `P, Q`, if `⊢ P` and `⊢ (P -> Q)`, then `⊢ Q`.
//!
//! This is given by the type [`MP<PrP, PrI>`]. `PrP` is a proof of `P`, and `PrI` is a proof of `(P -> Q)`. We
//! actually allow [`MP`] to take in proofs of `P` and `(P -> Q)` instead of just the statements themselves to
//! allow breaking proofs down into smaller theorems. This doesn't affect the validity of proofs or what can/can't
//! be proved, since a full formal proof can be obtained by just substituting the proofs in.
//!
//! ## Proofs
//!
//! A proof of a formula `P` is a sequence of statements where:
//! - Each statement is of the form `⊢ Q` for some formula `Q`.
//! - Each statement is either an axiom or follows from two previous statements (not necessarily immediately
//! previous) via modus ponens.
//! - The final statement is `⊢ P`.
//!
//! This is represented by the trait [`Proof`]. A valid proof of a [`Formula`] `P` will implement [`Proof`] with
//! [`Proof::Proves`] equal to `P`; an invalid proof will not. The [`assert_proves`] function is provided as an easy
//! way to check that a type implements [`Proof`] with the correct [`Proof::Proves`]. See below for an example.
//!
//! ## Example
//!
//! Proof that for any formula `P`, we have `⊢ P -> P`:
//!
//! ```text
//! 1. ⊢ P -> ((p0 -> P) -> P)                                      [Axiom 1 with P and (p0 -> P)]
//! 2. ⊢ (P -> ((p0 -> P) -> P)) -> ((P -> (p0 -> P)) -> (P -> P))  [Axiom 2 with P, (p0 -> P) and P]
//! 3. ⊢ (P -> (p0 -> P)) -> (P -> P)                               [MP on 1. and 3.]
//! 4. ⊢ P -> (p0 -> P)                                             [Axiom 1 with P and p0]
//! 5. ⊢ P -> P                                                     [MP on 4. and 3.]
//! ```
//!
//! This can be represented in our types as follows:
//!
//! ```
//! use type_proof::{
//!     formula::{Formula, Implies, P0},
//!     proof::{Axiom1, Axiom2, MP, assert_proves},
//! };
//!
//! type ToProve<P> = Implies<P, P>;
//!
//! type Step1<P> = Axiom1<P, Implies<P0, P>>;
//! type Step2<P> = Axiom2<P, Implies<P0, P>, P>;
//! type Step3<P> = MP<Step1<P>, Step2<P>>;
//! type Step4<P> = Axiom1<P, P0>;
//! type Step5<P> = MP<Step4<P>, Step3<P>>;
//!
//! // The fact that this type checks means that the proof is valid for any formula P
//! fn for_all<P: Formula>() {
//!     assert_proves::<Step5<P>, ToProve<P>>();
//! }
//! ```
//!
//! [`P2`]: crate::formula::P2

use std::marker::PhantomData;

use crate::formula::{Formula, Implies, Not};

/// A proof of [`Proof::Proves`].
pub trait Proof {
    /// The formula that this proof proves.
    type Proves: Formula;

    /// How many statements this proof involves.
    const LENGTH: usize;

    /// Displays the proof as a string.
    fn display() -> String {
        Self::display_offset(1)
    }

    /// Displays the proof as a string, with line numbers starting from `start_line_num`.
    fn display_offset(start_line_num: usize) -> String;
}

/// Statically asserts that `Pr` is a proof of `P`, throwing a type checker error if it is not.
///
/// # Example
///
/// See the [module documentation](self).
pub fn assert_proves<Pr, P>()
where
    Pr: Proof<Proves = P>,
{
}

/// The axiom schema `⊢ P -> (Q -> P)`, where `P, Q` are any formulas.
#[allow(type_alias_bounds)]
pub type Axiom1<P, Q>
where
    P: Formula,
    Q: Formula,
= Implies<P, Implies<Q, P>>;

/// The axiom schema `⊢ (P -> (Q -> R)) -> ((P -> Q) -> (P -> R))`, where `P, Q, R` are any formulas.
#[allow(type_alias_bounds)]
pub type Axiom2<P, Q, R>
where
    P: Formula,
    Q: Formula,
    R: Formula,
= Implies<Implies<P, Implies<Q, R>>, Implies<Implies<P, Q>, Implies<P, R>>>;

/// The axiom schema `⊢ (¬Q -> ¬P) -> (P -> Q)`, where `P, Q` are any formulas.
#[allow(type_alias_bounds)]
pub type Axiom3<P, Q>
where
    P: Formula,
    Q: Formula,
= Implies<Implies<Not<Q>, Not<P>>, Implies<P, Q>>;

fn proof_display_formula<P>(line_num: usize) -> String
where
    P: Formula,
{
    format!("{:<3} {}", format!("{}.", line_num), P::display())
}

impl<P, Q> Proof for Axiom1<P, Q>
where
    P: Formula,
    Q: Formula,
{
    type Proves = Self;

    const LENGTH: usize = 1;

    fn display_offset(start_line_num: usize) -> String {
        proof_display_formula::<Self>(start_line_num)
    }
}

impl<P, Q, R> Proof for Axiom2<P, Q, R>
where
    P: Formula,
    Q: Formula,
    R: Formula,
{
    type Proves = Self;

    const LENGTH: usize = 1;

    fn display_offset(start_line_num: usize) -> String {
        proof_display_formula::<Self>(start_line_num)
    }
}

impl<P, Q> Proof for Axiom3<P, Q>
where
    P: Formula,
    Q: Formula,
{
    type Proves = Self;

    const LENGTH: usize = 1;

    fn display_offset(start_line_num: usize) -> String {
        proof_display_formula::<Self>(start_line_num)
    }
}

/// The modus ponens inference rule:
/// if `⊢ P` and `⊢ (P -> Q)`, then `⊢ Q`.
///
/// `PrP` should be a proof of `P`. `PrI` should be a proof of `(P -> Q)`.
pub struct MP<PrP, PrI> {
    _p: PhantomData<PrP>,
    _q: PhantomData<PrI>,
}

impl<P, Q, PrP, PrI> Proof for MP<PrP, PrI>
where
    P: Formula,
    Q: Formula,
    PrP: Proof<Proves = P>,
    PrI: Proof<Proves = Implies<P, Q>>,
{
    type Proves = Q;

    const LENGTH: usize = PrP::LENGTH + PrI::LENGTH + 1;

    fn display_offset(start_line_num: usize) -> String {
        format!(
            "{}\n{}\n{}",
            PrP::display_offset(start_line_num),
            PrI::display_offset(start_line_num + PrP::LENGTH),
            proof_display_formula::<Q>(start_line_num + PrP::LENGTH + PrI::LENGTH)
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::formula::{P0, P2};

    use super::*;

    #[test]
    #[allow(unused)]
    fn prove_p_implies_p() {
        type ToProve<P> = Implies<P, P>;

        type Step1<P> = Axiom1<P, Implies<P0, P>>;
        type Step2<P> = Axiom2<P, Implies<P0, P>, P>;
        type Step3<P> = MP<Step1<P>, Step2<P>>;
        type Step4<P> = Axiom1<P, P0>;
        type Step5<P> = MP<Step4<P>, Step3<P>>;

        fn for_all<P: Formula>() {
            assert_proves::<Step5<P>, ToProve<P>>();
        }
    }

    #[test]
    fn display() {
        // Proving p2 -> p2

        type Step1 = Axiom1<P2, Implies<P0, P2>>;
        type Step2 = Axiom2<P2, Implies<P0, P2>, P2>;
        type Step3 = MP<Step1, Step2>;
        type Step4 = Axiom1<P2, P0>;
        type Step5 = MP<Step4, Step3>;

        let expected_display = "1.  (p2 -> (p0 -> p2))
2.  (p2 -> ((p0 -> p2) -> p2))
3.  ((p2 -> ((p0 -> p2) -> p2)) -> ((p2 -> (p0 -> p2)) -> (p2 -> p2)))
4.  ((p2 -> (p0 -> p2)) -> (p2 -> p2))
5.  (p2 -> p2)";

        assert_eq!(Step5::display(), expected_display);
    }
}
