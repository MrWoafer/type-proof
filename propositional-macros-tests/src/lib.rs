#[cfg(test)]
mod tests {
    use propositional::peano::Nat;
    use propositional_macros::nat;

    #[test]
    fn nat() {
        type N0Macro = nat!(0);
        assert_eq!(N0Macro::VALUE, 0);

        type N15Macro = nat!(15);
        assert_eq!(N15Macro::VALUE, 15);
    }
}
