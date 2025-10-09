#[allow(unused)]
pub trait TypeEq<T> {}
impl<T> TypeEq<T> for T {}

#[allow(unused)]
pub fn assert_type_eq<T, U>()
where
    T: TypeEq<U>,
{
}
