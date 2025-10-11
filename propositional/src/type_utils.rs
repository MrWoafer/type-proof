//! Provides some helpers for working with Rust types.

/// This trait just says a type is equal to itself, for making [`assert_type_eq`] work.
///
/// This is automatically implemented on every type.
pub trait TypeEq {
    type SelfType;
}

impl<T> TypeEq for T {
    type SelfType = Self;
}

/// Statically asserts that two types are the same, throwing a type checker error if they are not.
///
/// # Example
///
/// ```
/// use propositional::type_utils::assert_type_eq;
///
/// type X = usize;
/// assert_type_eq::<X, usize>();
///
/// // Throws type checker error
/// // assert_type_eq::<f32, usize>();
/// ```
pub fn assert_type_eq<T, U>()
where
    T: TypeEq<SelfType = U>,
{
}
