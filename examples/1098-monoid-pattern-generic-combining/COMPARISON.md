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

module Sum = struct type t = int let empty = 0 let combine = (+) end
module Product = struct type t = int let empty = 1 let combine = ( * ) end

let () =
  Printf.printf "sum: %d\n" (concat_all (module Sum) [1;2;3;4;5]);
  Printf.printf "product: %d\n" (concat_all (module Product) [1;2;3;4;5])
```

### Rust (idiomatic)
```rust
pub trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

pub fn concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M {
    items.into_iter().fold(M::empty(), M::combine)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sum(pub i64);

impl Monoid for Sum {
    fn empty() -> Self { Sum(0) }
    fn combine(self, other: Self) -> Self { Sum(self.0 + other.0) }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Product(pub i64);

impl Monoid for Product {
    fn empty() -> Self { Product(1) }
    fn combine(self, other: Self) -> Self { Product(self.0 * other.0) }
}
```

### Rust (recursive)
```rust
pub fn concat_all_recursive<M: Monoid + Clone>(items: &[M]) -> M {
    match items {
        [] => M::empty(),
        [x] => x.clone(),
        [x, rest @ ..] => M::combine(x.clone(), concat_all_recursive(rest)),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Module type / Trait | `module type MONOID = sig type t val empty : t val combine : t -> t -> t end` | `trait Monoid { fn empty() -> Self; fn combine(self, other: Self) -> Self; }` |
| Generic fold function | `val concat_all : (module MONOID with type t = 'a) -> 'a list -> 'a` | `fn concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M` |
| Identity element | `M.empty` (module field) | `M::empty()` (associated function) |
| Binary operation | `M.combine` (module field, curried) | `M::combine` (method, two args) |
| Newtype for int | Not needed — modules disambiguate | `struct Sum(pub i64)` / `struct Product(pub i64)` |

## Key Insights

1. **First-class modules vs traits:** OCaml's first-class modules are value-level — you pass `(module Sum)` as a runtime argument. Rust's traits are type-level — `M: Monoid` is resolved at compile time via monomorphization. Both achieve the same abstraction, but OCaml's is more dynamic and Rust's is zero-cost.

2. **Newtype pattern is essential in Rust:** OCaml can define `Sum` and `Product` both with `type t = int` because modules are distinct values. In Rust, `i64` can only implement `Monoid` once, so we wrap it in newtypes (`Sum(i64)`, `Product(i64)`) to create distinct types with distinct trait implementations.

3. **Currying vs explicit arguments:** OCaml's `combine = (+)` is naturally curried — `(+)` is already `int -> int -> int`. Rust's `combine(self, other: Self)` takes two explicit arguments. The `fold` call adapts naturally: OCaml's `List.fold_left M.combine M.empty lst` becomes Rust's `items.into_iter().fold(M::empty(), M::combine)`.

4. **Iterator generality:** OCaml's `concat_all` takes `'a list`. Rust's version takes `impl IntoIterator<Item = M>`, making it work with any iterator — vectors, arrays, ranges, or lazy chains — not just lists.

5. **Clone requirement in recursive version:** The recursive Rust version needs `Clone` because it works with `&[M]` (borrowed slice) but must produce an owned `M`. OCaml's garbage collector handles this transparently — values are reference-counted, so "copying" is just bumping a counter.

## When to Use Each Style

**Use idiomatic Rust (fold) when:** You want the standard, efficient approach. The fold version works with any iterator, avoids Clone bounds, and is the natural Rust way to express monoidal aggregation. This is the default choice.

**Use recursive Rust when:** You're teaching the OCaml-to-Rust translation pattern, or when the combining logic has complex branching that doesn't map cleanly to a single fold. The recursive version also demonstrates slice pattern matching (`[x, rest @ ..]`), which is a powerful Rust feature for list-like processing.
