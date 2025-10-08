//! Defines formulas as types.

use std::marker::PhantomData;

use crate::peano::{N0, N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, Nat};

/// A logical formula.
pub trait Formula {}

/// A propositional variable.
///
/// `N` is the index of the propositional variable.
pub struct Variable<N>
where
    N: Nat,
{
    _n: PhantomData<N>,
}

impl<N> Formula for Variable<N> where N: Nat {}

/// The propositional variable `p0`.
pub type P0 = Variable<N0>;

/// The propositional variable `p1`.
pub type P1 = Variable<N1>;

/// The propositional variable `p2`.
pub type P2 = Variable<N2>;

/// The propositional variable `p3`.
pub type P3 = Variable<N3>;

/// The propositional variable `p4`.
pub type P4 = Variable<N4>;

/// The propositional variable `p5`.
pub type P5 = Variable<N5>;

/// The propositional variable `p6`.
pub type P6 = Variable<N6>;

/// The propositional variable `p7`.
pub type P7 = Variable<N7>;

/// The propositional variable `p8`.
pub type P8 = Variable<N8>;

/// The propositional variable `p9`.
pub type P9 = Variable<N9>;

/// The propositional variable `p10`.
pub type P10 = Variable<N10>;

/// Logical negation:
/// `¬P`
pub struct Not<P>
where
    P: Formula,
{
    _p: PhantomData<P>,
}

impl<P> Formula for Not<P> where P: Formula {}

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
}
