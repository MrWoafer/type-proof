//! Defines formulas as types.

use std::marker::PhantomData;

use crate::{
    boolean::{self, Bool},
    peano::{N0, N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, Nat},
};

pub use propositional_macros::var;

/// A logical formula.
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

/// Deduces the value of an expression given the value of each of its propositional variables.
///
/// A valuation type should implement [`Valuation`] for each of its propositional variables, and
/// [`Valuation`] will automatically be implemented for all formulas constructible from those variables.
///
/// ```
/// use propositional::{
///     boolean::{False, True},
///     formula::{Implies, Not, P0, P2, Valuation},
///     type_utils::assert_type_eq,
/// };
///
/// type P = Implies<P0, Not<P2>>;
/// struct V;
/// impl Valuation<P0> for V {
///     type Value = True;
/// }
/// impl Valuation<P2> for V {
///     type Value = False;
/// }
///
/// // V automatically implements Valuation<P> given Valuation<P0> and Valuation<P2>
/// type PVal = <V as Valuation<P>>::Value;
/// assert_type_eq::<PVal, True>();
/// ```
pub trait Valuation<P>
where
    P: Formula,
{
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
        assert_type_eq::<var!(0), P0>();
        assert_type_eq::<var!(3), P3>();
    }
}
