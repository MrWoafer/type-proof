![CI](https://github.com/MrWoafer/type-proof/actions/workflows/ci.yml/badge.svg)

# type-proof

A Rust crate for type-checked [propositional logic](https://en.wikipedia.org/wiki/Propositional_logic) proofs.

## Key features

- Construction of Boolean values with True and False each being its own type
  - Includes type-level Boolean operations
- Construction of the natural numbers with each number being its own type
  - Includes type-level arithmetic operations
- Construction of propositional formulas with each formula being its own type
- Construction of a proof system where a proof is valid if and only if it type checks
- Valuation of formulas at the type level

## More detail

Instead of defining Booleans / numbers / formulas / proofs as instances of a type (e.g. a natural number as a `usize`), we define each Boolean / number / formula / proof as its own type: e.g. 0 is the type `N0`, 1 is the type `N1`, etc. These types never need to be instantiated - we work solely with the definitions of the types.

For example, the formula `(p0 -> ¬p2)` is constructed as the following type:

```rust
use type_proof::formula::{Implies, Not, P0, P2};

type P = Implies<P0, Not<P2>>;
```

Likewise, proofs can be defined as types. The type of a **valid** proof will automatically implement the `Proof` trait. This trait contains an associated type, which will be that of the formula it proves. Therefore, the validity of a proof can be checked by the type checker, by determining whether the `Proof` trait is implemented.

Here's an example of a proof of `(P -> P)`, where `P` is an arbitrary formula:

```rust
use type_proof::{
    formula::{Formula, Implies, P0},
    proof::{Axiom1, Axiom2, MP, assert_proves},
};

type ToProve<P> = Implies<P, P>;

type Step1<P> = Axiom1<P, Implies<P0, P>>;
type Step2<P> = Axiom2<P, Implies<P0, P>, P>;
type Step3<P> = MP<Step1<P>, Step2<P>>;
type Step4<P> = Axiom1<P, P0>;
type Step5<P> = MP<Step4<P>, Step3<P>>;

fn for_all<P: Formula>() {
    // This type checks, so it's a valid proof
    assert_proves::<Step5<P>, ToProve<P>>();
}
```

For more details on the construction of the types and on the details of the logical language / proof system, see the individual modules' documentation.

As well as writing proofs, we can also evaluate formulas for different values of the propositional variables, evaluated at the type level to either the `True` type or the `False` type:

```rust
use type_proof::{
    boolean::{False, True},
    formula::{And, Not, P0, P2, Valuation},
    type_utils::assert_type_eq,
};

type P = And<P0, Not<P2>>;

struct V;
impl Valuation<P0> for V {
    type Value = True;
}
impl Valuation<P2> for V {
    type Value = False;
}

type ValueOfP = <V as Valuation<P>>::Value;
assert_type_eq::<ValueOfP, True>();
```

## Future work

- Currently, we can only prove tautologies. It would be good to allow proofs to have hypotheses
- A macro to parse formulas into a type
- A macro to make it easier to create valuations
