//! Defines Boolean values and operations as types.

/// A Boolean value, encoded as a type.
pub trait Bool: NotImpl {
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

pub trait ImpliesImpl<B>
where
    B: Bool,
{
    type Output: Bool;
}

impl ImpliesImpl<True> for True {
    type Output = True;
}

impl ImpliesImpl<False> for True {
    type Output = False;
}

impl ImpliesImpl<True> for False {
    type Output = True;
}

impl ImpliesImpl<False> for False {
    type Output = True;
}

/// Boolean implication.
#[allow(type_alias_bounds)]
pub type Implies<A, B>
where
    A: Bool,
    B: Bool,
= <A as ImpliesImpl<B>>::Output;

/// Boolean and.
#[allow(type_alias_bounds)]
pub type And<A, B>
where
    A: Bool,
    B: Bool,
= Not<Implies<A, Not<B>>>;

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

    #[test]
    fn implies() {
        assert_type_eq::<True, Implies<True, True>>();
        assert_type_eq::<True, Implies<False, True>>();
        assert_type_eq::<False, Implies<True, False>>();
        assert_type_eq::<True, Implies<False, False>>();
    }
}
