//! A typed version of Peano arithmetic.
//!
//! Each natural number has its own type - for example, `0` has the type [`N0`].
//! There are operations defined on these typed natural numbers - for example, [`N5`] is the same
//! type as [`Add<N2, N3>`].
//!
//! # Construction
//!
//! We start with the natural number `0` defined as a type [`N0`]. We then have a type [`Succ<N>`] and define
//! `1` as [`Succ<N0>`], `2` as [`Succ<N2>`], etc.
//!
//! We provide type aliases [`N1`], [`N2`], ... for the first few natural numbers.
//!
//! Arbitrarily large natural numbers can be created using the [`N!`] macro.
//!
//! Arithmetic operations are defined via type aliases, so there's a bijection between natural numbers and types.
//!
//! ## Example
//!
//! ```
//! use type_proof::{
//!     peano::{Add, N2, N3, N5},
//!     type_utils::assert_type_eq,
//! };
//!
//! assert_type_eq::<Add<N2, N3>, N5>();
//! ```

use std::marker::PhantomData;

pub use type_proof_macros::N;

/// A natural number (0, 1, 2, ...), encoded as a type.
pub trait Nat {
    /// The natural number as a [`usize`].
    const VALUE: usize;

    /// The value of `Self + N`.
    type Add<N: Nat>: Nat;

    /// The value of `Self * N`.
    type Mul<N: Nat>: Nat;

    /// The value of `N ^ Self`.
    ///
    /// Note `0 ^ 0` is taken to be `1`.
    ///
    /// Don't confuse with `Self ^ N`.
    type Pow<N: Nat>: Nat;
}

/// The successor of `N`, i.e. `N + 1`.
pub struct Succ<N>
where
    N: Nat,
{
    _n: PhantomData<N>,
}

impl<N> Nat for Succ<N>
where
    N: Nat,
{
    const VALUE: usize = N::VALUE + 1;

    type Add<M: Nat> = Succ<N::Add<M>>;

    type Mul<M: Nat> = <N::Mul<M> as Nat>::Add<M>;

    type Pow<M: Nat> = <N::Pow<M> as Nat>::Mul<M>;
}

/// The natural number 0.
pub struct N0 {}

impl Nat for N0 {
    const VALUE: usize = 0;

    type Add<N: Nat> = N;

    type Mul<N: Nat> = N0;

    type Pow<N: Nat> = N1;
}

/// The natural number 1.
pub type N1 = Succ<N0>;

/// The natural number 2.
pub type N2 = Succ<N1>;

/// The natural number 3.
pub type N3 = Succ<N2>;

/// The natural number 4.
pub type N4 = Succ<N3>;

/// The natural number 5.
pub type N5 = Succ<N4>;

/// The natural number 6.
pub type N6 = Succ<N5>;

/// The natural number 7.
pub type N7 = Succ<N6>;

/// The natural number 8.
pub type N8 = Succ<N7>;

/// The natural number 9.
pub type N9 = Succ<N8>;

/// The natural number 10.
pub type N10 = Succ<N9>;

/// Addition:
/// `N + M`
#[allow(type_alias_bounds)]
pub type Add<N, M>
where
    N: Nat,
    M: Nat,
= <N as Nat>::Add<M>;

/// Multiplication:
/// `N * M`
#[allow(type_alias_bounds)]
pub type Mul<N, M>
where
    N: Nat,
    M: Nat,
= <N as Nat>::Mul<M>;

/// Exponentiation:
/// `N ^ M`
///
/// Note `0 ^ 0` is taken to be `1`.
#[allow(type_alias_bounds)]
pub type Pow<N, M>
where
    N: Nat,
    M: Nat,
= <M as Nat>::Pow<N>;

#[cfg(test)]
mod tests {
    use crate::type_utils::assert_type_eq;

    use super::*;

    #[test]
    fn value() {
        assert_eq!(N0::VALUE, 0);
        assert_eq!(N1::VALUE, 1);
        assert_eq!(N5::VALUE, 5);
        assert_eq!(N8::VALUE, 8);
    }

    #[test]
    fn add() {
        assert_type_eq::<N0, Add<N0, N0>>();
        assert_type_eq::<N3, Add<N3, N0>>();
        assert_type_eq::<N2, Add<N0, N2>>();
        assert_type_eq::<N5, Add<N3, N2>>();
    }

    #[test]
    fn mul() {
        assert_type_eq::<N0, Mul<N0, N0>>();
        assert_type_eq::<N0, Mul<N3, N0>>();
        assert_type_eq::<N0, Mul<N0, N2>>();
        assert_type_eq::<N3, Mul<N3, N1>>();
        assert_type_eq::<N2, Mul<N1, N2>>();
        assert_type_eq::<N6, Mul<N3, N2>>();
    }

    #[test]
    fn pow() {
        assert_type_eq::<N1, Pow<N0, N0>>();
        assert_type_eq::<N0, Pow<N0, N2>>();
        assert_type_eq::<N1, Pow<N3, N0>>();
        assert_type_eq::<N1, Pow<N1, N2>>();
        assert_type_eq::<N9, Pow<N3, N2>>();
    }

    #[test]
    fn nat_macro() {
        assert_type_eq::<N!(0), N0>();
        assert_type_eq::<N!(4), N4>();

        type N15Macro = N!(15);
        assert_eq!(N15Macro::VALUE, 15);
    }
}
