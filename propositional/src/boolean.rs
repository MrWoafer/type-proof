//! Defines Boolean values and operations as types.

/// A Boolean value, encoded as a type.
pub trait Bool {
    const VALUE: bool;
}

/// The Boolean value True.
pub struct True {}

impl Bool for True {
    const VALUE: bool = true;
}

/// The Boolean value False.
pub struct False {}

impl Bool for False {
    const VALUE: bool = false;
}

pub trait NotImpl {
    type Output: Bool;
}

impl NotImpl for True {
    type Output = False;
}

impl NotImpl for False {
    type Output = True;
}

/// Boolean negation.
#[allow(type_alias_bounds)]
pub type Not<B>
where
    B: Bool,
= <B as NotImpl>::Output;

pub trait AndImpl<B>
where
    B: Bool,
{
    type Output: Bool;
}

impl AndImpl<True> for True {
    type Output = True;
}

impl AndImpl<False> for True {
    type Output = False;
}

impl AndImpl<True> for False {
    type Output = False;
}

impl AndImpl<False> for False {
    type Output = False;
}

/// Boolean and.
#[allow(type_alias_bounds)]
pub type And<A, B>
where
    A: Bool,
    B: Bool,
= <A as AndImpl<B>>::Output;

/// Boolean or.
#[allow(type_alias_bounds)]
pub type Or<A, B>
where
    A: Bool,
    B: Bool,
= Not<And<Not<A>, Not<B>>>; // using De Morgan's laws

/// Boolean xor.
#[allow(type_alias_bounds)]
pub type Xor<A, B>
where
    A: Bool,
    B: Bool,
= And<Or<A, B>, Not<And<A, B>>>;

#[cfg(test)]
mod tests {
    use crate::type_utils::assert_type_eq;

    use super::*;

    #[test]
    fn not() {
        assert_type_eq::<False, Not<True>>();
        assert_type_eq::<True, Not<False>>();
    }

    #[test]
    fn and() {
        assert_type_eq::<True, And<True, True>>();
        assert_type_eq::<False, And<False, True>>();
        assert_type_eq::<False, And<True, False>>();
        assert_type_eq::<False, And<False, False>>();
    }

    #[test]
    fn or() {
        assert_type_eq::<True, Or<True, True>>();
        assert_type_eq::<True, Or<False, True>>();
        assert_type_eq::<True, Or<True, False>>();
        assert_type_eq::<False, Or<False, False>>();
    }

    #[test]
    fn xor() {
        assert_type_eq::<False, Xor<True, True>>();
        assert_type_eq::<True, Xor<False, True>>();
        assert_type_eq::<True, Xor<True, False>>();
        assert_type_eq::<False, Xor<False, False>>();
    }
}
