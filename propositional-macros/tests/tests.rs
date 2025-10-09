use propositional::{
    formula::{P0, P3},
    peano::Nat,
    type_utils::assert_type_eq,
};
use propositional_macros::{nat, var};

#[test]
fn nat() {
    type N0Macro = nat!(0);
    assert_eq!(N0Macro::VALUE, 0);

    type N15Macro = nat!(15);
    assert_eq!(N15Macro::VALUE, 15);
}

#[test]
fn var() {
    assert_type_eq::<var!(0), P0>();
    assert_type_eq::<var!(3), P3>();
}
