# OCaml vs Rust: Monoid Pattern — Generic Combining

## Side-by-Side Code

### OCaml

```ocaml
module type MONOID = sig
  type t
  val empty : t
  val combine : t -> t -> t
end

let concat_all (type a) (module M : MONOID with type t = a) (lst : a list) =
  List.fold_left M.combine M.empty lst

module Sum     = struct type t = int  let empty = 0    let combine = ( + ) end
module Product = struct type t = int  let empty = 1    let combine = ( * ) end
module Concat  = struct type t = string let empty = "" let combine = ( ^ ) end
module All     = struct type t = bool let empty = true let combine = ( && ) end

let () =
  Printf.printf "sum: %d\n"     (concat_all (module Sum)     [1;2;3;4;5]);
  Printf.printf "product: %d\n" (concat_all (module Product) [1;2;3;4;5]);
  Printf.printf "concat: %s\n"  (concat_all (module Concat)  ["hello";" ";"world"]);
  Printf.printf "all: %b\n"     (concat_all (module All)     [true; true; false])
```

### Rust (idiomatic — trait + newtype + iterator fold)

```rust
pub trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

pub fn concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M {
    items.into_iter().fold(M::empty(), M::combine)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sum(pub i32);

impl Monoid for Sum {
    fn empty() -> Self { Sum(0) }
    fn combine(self, other: Self) -> Self { Sum(self.0 + other.0) }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Product(pub i32);

impl Monoid for Product {
    fn empty() -> Self { Product(1) }
    fn combine(self, other: Self) -> Self { Product(self.0 * other.0) }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct All(pub bool);

impl Monoid for All {
    fn empty() -> Self { All(true) }
    fn combine(self, other: Self) -> Self { All(self.0 && other.0) }
}

impl Monoid for String {
    fn empty() -> Self { String::new() }
    fn combine(self, other: Self) -> Self { self + &other }
}
```

### Rust (functional/recursive — mirrors OCaml's structural recursion)

```rust
pub fn concat_all_recursive<M: Monoid + Clone>(items: &[M]) -> M {
    match items {
        []             => M::empty(),
        [x]            => x.clone(),
        [head, tail @ ..] => head.clone().combine(concat_all_recursive(tail)),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Typeclass / interface | `module type MONOID` | `trait Monoid` |
| Identity element | `val empty : t` | `fn empty() -> Self` |
| Binary operation | `val combine : t -> t -> t` | `fn combine(self, other: Self) -> Self` |
| Generic fold | `concat_all : (module MONOID with type t = 'a) -> 'a list -> 'a` | `fn concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M` |
| Sum instance | `module Sum : MONOID with type t = int` | `impl Monoid for Sum` (newtype) |
| Product instance | `module Product : MONOID with type t = int` | `impl Monoid for Product` (newtype) |

## Key Insights

1. **Module type ≅ trait.** OCaml's `module type MONOID` and Rust's `trait Monoid` are the same abstraction at the type-theory level. Both specify `empty` and `combine`; both enable generic fold without knowing the concrete type.

2. **Newtype pattern solves the coherence problem.** OCaml allows `module Sum` and `module Product` to provide different implementations for the same `int` because modules are passed explicitly as arguments. Rust's trait system enforces *coherence*: at most one `impl Monoid for i32` may exist in the crate graph. The newtype wrapper (`struct Sum(i32)`) creates a fresh type that can have its own `impl` without conflicting.

3. **`IntoIterator` generalises `List.fold_left`.** OCaml's `List.fold_left` only works on lists. Rust's `Iterator::fold` works on any `IntoIterator` — arrays, slices, `Vec`, ranges, lazy iterators — making `concat_all` more powerful than the OCaml original with no extra effort.

4. **Value semantics in `combine`.** Both languages pass arguments to `combine` by value. In OCaml this is automatic (immutable values are cheap to copy). In Rust, `fn combine(self, other: Self)` makes ownership explicit: the inputs are consumed and a new value is returned, matching OCaml's semantics exactly without any hidden copying.

5. **Monoid laws are untypeable, but testable.** Neither OCaml nor Rust can encode the identity and associativity laws in the type system. Both rely on convention. The Rust tests explicitly verify `combine(empty(), x) == x`, `combine(x, empty()) == x`, and `combine(combine(a, b), c) == combine(a, combine(b, c))` for concrete instances.

## When to Use Each Style

**Use idiomatic Rust (iterator fold + newtype)** when building library APIs where the monoid instances need to be usable across modules and crates — the types are reusable, zero-overhead, and composable with any iterator.

**Use recursive Rust** when teaching the OCaml structural-recursion pattern explicitly, or when the input is a recursive data structure (e.g., a tree) where fold naturally decomposes into base case and recursive case.
