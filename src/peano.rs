//! A typed version of Peano arithmetic.
//!
//! Each natural number has its own type - for example, `0` has the type [`N0`].
//! There are operations defined on these typed natural numbers - for example, [`N5`] is the same
//! type as [`Add<N2, N3>`].

use std::marker::PhantomData;

/// A natural number: 0, 1, 2, ...
pub trait Nat {
    const VALUE: usize;
}

/// The successor of `N`. I.e. `N + 1`.
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

pub trait AddTo<N>
where
    N: Nat,
{
    type Result: Nat;
}

impl<N> AddTo<N0> for N
where
    N: Nat,
{
    type Result = N;
}

impl<N, M> AddTo<Succ<M>> for N
where
    N: Nat,
    M: Nat,
    N: AddTo<M>,
{
    type Result = Succ<Add<N, M>>;
}

/// Addition:
/// `N + M`
#[allow(type_alias_bounds)]
pub type Add<N, M>
where
    N: Nat,
    M: Nat,
= <N as AddTo<M>>::Result;

pub trait MulWith<N>
where
    N: Nat,
{
    type Result: Nat;
}

impl<N> MulWith<N0> for N
where
    N: Nat,
{
    type Result = N0;
}

impl<N, M> MulWith<Succ<M>> for N
where
    N: Nat,
    M: Nat,
    N: MulWith<M>,
    Mul<N, M>: AddTo<N>,
{
    type Result = Add<Mul<N, M>, N>;
}

/// Multiplication:
/// `N * M`
#[allow(type_alias_bounds)]
pub type Mul<N, M>
where
    N: Nat,
    M: Nat,
= <N as MulWith<M>>::Result;

pub trait ToPow<N>
where
    N: Nat,
{
    type Result: Nat;
}

impl<N> ToPow<N0> for N
where
    N: Nat,
{
    type Result = N1;
}

impl<N, M> ToPow<Succ<M>> for N
where
    N: Nat,
    M: Nat,
    N: ToPow<M>,
    Pow<N, M>: MulWith<N>,
{
    type Result = Mul<Pow<N, M>, N>;
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
= <N as ToPow<M>>::Result;

#[cfg(test)]
mod tests {
    use super::*;

    trait TypeEq<T> {}
    impl<T> TypeEq<T> for T {}

    fn assert_type_eq<T, U>()
    where
        T: TypeEq<U>,
    {
    }

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
        assert_type_eq::<N1, Pow<N3, N0>>();
        assert_type_eq::<N1, Pow<N1, N2>>();
        assert_type_eq::<N9, Pow<N3, N2>>();
    }
}
