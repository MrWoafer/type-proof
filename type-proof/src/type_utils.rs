//! Provides some helpers for working with Rust types.

/// This trait just says a type is equal to itself, for making [`assert_type_eq`] work.
///
/// This is automatically implemented on every type.
pub trait TypeEq {
    /// This is always the type that the trait is implemented on.
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
/// use type_proof::type_utils::assert_type_eq;
///
/// type X = usize;
/// assert_type_eq::<X, usize>();
/// ```
///
/// ```compile_fail
/// # use type_proof::type_utils::assert_type_eq;
/// #
/// assert_type_eq::<f32, usize>();
/// ```
pub fn assert_type_eq<T, U>()
where
    T: TypeEq<SelfType = U>,
{
}
