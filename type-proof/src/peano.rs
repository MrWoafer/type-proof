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
//! Arbitrarily large natural numbers can be created using the [`nat!`] macro.
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

pub use type_proof_macros::nat;

/// A natural number: 0, 1, 2, ...
pub trait Nat {
    /// The natural number as a [`usize`].
    const VALUE: usize;
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
}

/// The natural number 0.
pub struct N0 {}

impl Nat for N0 {
    const VALUE: usize = 0;
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
= <N as AddImpl<M>>::Output;

/// Defines the value of `Self + N`.
///
/// Used to define the more convenient [`Add<N, M>`].
pub trait AddImpl<N>
where
    N: Nat,
{
    /// The value of `Self + N`.
    type Output: Nat;
}

impl<N> AddImpl<N0> for N
where
    N: Nat,
{
    type Output = N;
}

impl<N, M> AddImpl<Succ<M>> for N
where
    N: Nat,
    M: Nat,
    N: AddImpl<M>,
{
    type Output = Succ<Add<N, M>>;
}

/// Multiplication:
/// `N * M`
#[allow(type_alias_bounds)]
pub type Mul<N, M>
where
    N: Nat,
    M: Nat,
= <N as MulImpl<M>>::Output;

/// Defines the value of `Self * N`.
///
/// Used to define the more convenient [`Mul<N, M>`].
pub trait MulImpl<N>
where
    N: Nat,
{
    /// The value of `Self * N`.
    type Output: Nat;
}

impl<N> MulImpl<N0> for N
where
    N: Nat,
{
    type Output = N0;
}

impl<N, M> MulImpl<Succ<M>> for N
where
    N: Nat,
    M: Nat,
    N: MulImpl<M>,
    Mul<N, M>: AddImpl<N>,
{
    type Output = Add<Mul<N, M>, N>;
}

/// Exponentiation:
/// `N ^ M`
///
/// Note `0 ^ 0` is taken to be `1`.
#[allow(type_alias_bounds)]
pub type Pow<N, M>
where
    N: Nat,
    M: Nat,
= <N as PowImpl<M>>::Output;

/// Defines the value of `Self ^ N`.
///
/// Note `0 ^ 0` is taken to be `1`.
///
/// Used to define the more convenient [`Pow<N, M>`].
pub trait PowImpl<N>
where
    N: Nat,
{
    /// The value of `Self ^ N`.
    type Output: Nat;
}

impl<N> PowImpl<N0> for N
where
    N: Nat,
{
    type Output = N1;
}

impl<N, M> PowImpl<Succ<M>> for N
where
    N: Nat,
    M: Nat,
    N: PowImpl<M>,
    Pow<N, M>: MulImpl<N>,
{
    type Output = Mul<Pow<N, M>, N>;
}

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
        assert_type_eq::<nat!(0), N0>();
        assert_type_eq::<nat!(4), N4>();

        type N15Macro = nat!(15);
        assert_eq!(N15Macro::VALUE, 15);
    }
}
