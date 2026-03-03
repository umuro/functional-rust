# OCaml vs Rust: Monoid as a Category

## Side-by-Side Code

### OCaml

```ocaml
module type MONOID = sig
  type t
  val empty  : t
  val append : t -> t -> t
end

module MonoidLaws (M : MONOID) = struct
  let check_identity x =
    M.append M.empty x = x &&
    M.append x M.empty = x

  let check_associativity x y z =
    M.append (M.append x y) z = M.append x (M.append y z)
end

module StringMonoid : MONOID with type t = string = struct
  type t = string
  let empty  = ""
  let append = ( ^ )
end

let compose_morphisms ms =
  List.fold_left StringMonoid.append StringMonoid.empty ms
```

### Rust (idiomatic)

```rust
pub trait Monoid {
    fn empty() -> Self;
    fn append(self, other: Self) -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringMonoid(pub String);

impl Monoid for StringMonoid {
    fn empty() -> Self { StringMonoid(String::new()) }
    fn append(self, other: Self) -> Self { StringMonoid(self.0 + &other.0) }
}

pub fn compose_morphisms<M: Monoid>(morphisms: impl IntoIterator<Item = M>) -> M {
    morphisms.into_iter().fold(M::empty(), |acc, m| acc.append(m))
}
```

### Rust (functional/recursive)

```rust
pub fn compose_morphisms_recursive<M: Monoid + Clone>(morphisms: &[M]) -> M {
    match morphisms {
        []             => M::empty(),
        [x]            => x.clone(),
        [head, rest @ ..] => head.clone().append(compose_morphisms_recursive(rest)),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Monoid abstraction | `module type MONOID` | `trait Monoid` |
| Identity element | `val empty : t` | `fn empty() -> Self` |
| Binary operation | `val append : t -> t -> t` | `fn append(self, other: Self) -> Self` |
| Law checker | `module MonoidLaws(M : MONOID)` | `fn check_identity<M: Monoid + PartialEq + Clone>(x: M) -> bool` |
| Fold morphisms | `List.fold_left M.append M.empty ms` | `iter.fold(M::empty(), \|acc, m\| acc.append(m))` |
| Multiple instances | Separate named modules | Newtypes + separate `impl` blocks |

## Key Insights

1. **Modules vs traits:** OCaml's first-class module system lets you pass monoid implementations as functor arguments; Rust achieves the same via trait bounds on generic functions.
2. **Newtype pattern:** Because Rust allows only one `impl Trait for Type`, newtypes (`SumMonoid`, `ProductMonoid`) let you give the same underlying type (e.g. `i64`) multiple distinct monoid structures without conflicting `impl` blocks.
3. **`empty` as a method:** OCaml's module value `empty : t` becomes a static method `fn empty() -> Self` in Rust — this is the idiomatic way to express a type-level constant in a trait.
4. **Ownership and `append`:** Rust's `append(self, other: Self) -> Self` consumes both values, mirroring OCaml's value semantics; references are unnecessary here because monoids typically build new values.
5. **Fold = categorical composition:** Both languages express the composition of a sequence of morphisms as a left fold with the identity as the initial accumulator — this is the direct computational realisation of the category axioms.

## When to Use Each Style

**Use idiomatic Rust (iterator fold) when:** you have a collection of monoid values to reduce at runtime and want clean, allocation-free iteration.
**Use recursive Rust when:** you want to make the structural recursion explicit and mirror the OCaml proof-style reasoning about the base and inductive cases of list composition.
