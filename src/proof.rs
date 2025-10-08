//! Encodes proofs as types.

use std::marker::PhantomData;

use crate::formula::{Formula, Implies, Not};

/// A proof of [`Proof::Proves`].
pub trait Proof {
    type Proves: Formula;
}

/// Gives a type-check error if `Pr` is not a proof of `P`.
pub fn assert_proves<Pr, P>()
where
    Pr: Proof<Proves = P>,
{
}

/// The axiom `P -> (Q -> P)`
#[allow(type_alias_bounds)]
pub type Axiom1<P, Q>
where
    P: Formula,
    Q: Formula,
= Implies<P, Implies<Q, P>>;

/// The axiom `(P -> (Q -> R)) -> ((P -> Q) -> (P -> R))`
#[allow(type_alias_bounds)]
pub type Axiom2<P, Q, R>
where
    P: Formula,
    Q: Formula,
    R: Formula,
= Implies<Implies<P, Implies<Q, R>>, Implies<Implies<P, Q>, Implies<P, R>>>;

/// The axiom `(¬Q -> ¬P) -> (P -> Q)`
#[allow(type_alias_bounds)]
pub type Axiom3<P, Q>
where
    P: Formula,
    Q: Formula,
= Implies<Implies<Not<Q>, Not<P>>, Implies<P, Q>>;

impl<P, Q> Proof for Axiom1<P, Q>
where
    P: Formula,
    Q: Formula,
{
    type Proves = Self;
}

impl<P, Q, R> Proof for Axiom2<P, Q, R>
where
    P: Formula,
    Q: Formula,
    R: Formula,
{
    type Proves = Self;
}

impl<P, Q> Proof for Axiom3<P, Q>
where
    P: Formula,
    Q: Formula,
{
    type Proves = Self;
}

/// The modus ponens inference rule:
/// `{P, P -> Q} proves Q`
pub struct MP<P, Q> {
    _p: PhantomData<P>,
    _q: PhantomData<Q>,
}

impl<P, Q, PrP, PrI> Proof for MP<PrP, PrI>
where
    P: Formula,
    Q: Formula,
    PrP: Proof<Proves = P>,
    PrI: Proof<Proves = Implies<P, Q>>,
{
    type Proves = Q;
}

#[cfg(test)]
mod tests {
    use crate::formula::{P0, P2};

    use super::*;

    #[test]
    fn prove_p_implies_p() {
        type Desired<P> = Implies<P, P>;

        type Step1<P> = Axiom1<P, Implies<P0, P>>;
        type Step2<P> = Axiom2<P, Implies<P0, P>, P>;
        type Step3<P> = MP<Step1<P>, Step2<P>>;
        type Step4<P> = Axiom1<P, P0>;
        type Step5<P> = MP<Step4<P>, Step3<P>>;

        assert_proves::<Step5<P0>, Desired<P0>>();
        assert_proves::<Step5<P2>, Desired<P2>>();
    }
}
