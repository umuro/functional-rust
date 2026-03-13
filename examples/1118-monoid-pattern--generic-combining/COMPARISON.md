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

module Sum     = struct type t = int    let empty = 0    let combine = (+)  end
module Product = struct type t = int    let empty = 1    let combine = ( * ) end
module Concat  = struct type t = string let empty = ""   let combine = (^)  end
module All     = struct type t = bool   let empty = true let combine = (&&) end
```

### Rust (idiomatic)

```rust
pub trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

pub fn concat_all<T: Monoid>(items: impl IntoIterator<Item = T>) -> T {
    items.into_iter().fold(T::empty(), T::combine)
}

pub struct Sum(pub i64);
impl Monoid for Sum {
    fn empty() -> Self { Sum(0) }
    fn combine(self, other: Self) -> Self { Sum(self.0 + other.0) }
}
// … Product, Concat, All follow the same pattern
```

### Rust (functional/recursive)

```rust
pub fn concat_all_rec<T: Monoid + Clone>(items: &[T]) -> T {
    match items {
        [] => T::empty(),
        [head, rest @ ..] => head.clone().combine(concat_all_rec(rest)),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Type class / module signature | `module type MONOID = sig type t ... end` | `trait Monoid { fn empty() -> Self; fn combine(self, Self) -> Self; }` |
| Generic fold function | `val concat_all : (module MONOID with type t = 'a) -> 'a list -> 'a` | `fn concat_all<T: Monoid>(impl IntoIterator<Item=T>) -> T` |
| Identity element | `val empty : t` (module field) | `fn empty() -> Self` (associated function) |
| Binary operation | `val combine : t -> t -> t` | `fn combine(self, other: Self) -> Self` |
| Sum instance | `module Sum = struct type t = int let empty = 0 … end` | `impl Monoid for Sum { … }` on newtype `struct Sum(i64)` |
| Dispatch mechanism | First-class module passed at call site | Monomorphization via generic type parameter |

## Key Insights

1. **First-class modules ↔ trait impls:** OCaml's `(module Sum)` at the call site is the runtime carrier of the implementation; Rust's `T` in `concat_all<T: Monoid>` is resolved at compile time by the type system—no extra argument needed.

2. **Multiple instances for the same base type:** OCaml can have `Sum` and `Product` both with `type t = int` in separate modules and pass either one. Rust cannot have two `impl Monoid for i64`, so newtype wrappers (`Sum(i64)`, `Product(i64)`) provide the same capability without ambiguity.

3. **Identity as value vs associated function:** OCaml stores `empty` as a module-level value. Rust makes it `fn empty() -> Self`—a constructor-like associated function—which is required because trait methods can't be associated constants for non-`Copy` types in the general case (though `const` in traits is stabilizing).

4. **`fold` universality:** `List.fold_left` in OCaml is a specialized list function. Rust's `Iterator::fold` works on any type implementing `IntoIterator`—arrays, slices, `Vec`, lazy chains—making the generic combiner more reusable.

5. **Zero-overhead abstraction:** Rust monomorphizes `concat_all::<Sum>` and `concat_all::<Product>` into separate specialized functions at compile time, matching the performance of hand-written loops. OCaml's first-class modules carry vtables, so there is a small indirection cost.

## When to Use Each Style

**Use idiomatic Rust (`concat_all` with `Iterator::fold`) when:** folding any iterable collection, especially when performance matters or when you want to compose with other iterator adapters.

**Use recursive Rust (`concat_all_rec`) when:** demonstrating the OCaml structural recursion pattern for educational purposes, or when working with a recursive algebraic data structure (e.g., a linked list type) where pattern matching is more natural than iteration.
