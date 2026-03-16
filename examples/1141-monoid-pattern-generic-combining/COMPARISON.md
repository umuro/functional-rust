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
module Concat = struct type t = string let empty = "" let combine = (^) end
module All = struct type t = bool let empty = true let combine = (&&) end
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

#[derive(Clone, Copy)] pub struct Sum(pub i64);
impl Monoid for Sum {
    fn empty() -> Self { Sum(0) }
    fn combine(self, other: Self) -> Self { Sum(self.0 + other.0) }
}
```

### Rust (functional/recursive)

```rust
pub fn concat_all_recursive<M: Monoid + Clone>(items: &[M]) -> M {
    match items {
        [] => M::empty(),
        [x] => x.clone(),
        [x, rest @ ..] => x.clone().combine(concat_all_recursive(rest)),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Typeclass spec | `module type MONOID = sig type t; val empty : t; val combine : t -> t -> t end` | `trait Monoid { fn empty() -> Self; fn combine(self, other: Self) -> Self; }` |
| Generic function | `(type a) (module M : MONOID with type t = a) (lst : a list) -> a` | `fn concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M` |
| Identity element | `val empty : t` (module field) | `fn empty() -> Self` (associated function) |
| Fold | `List.fold_left M.combine M.empty lst` | `items.into_iter().fold(M::empty(), M::combine)` |
| Integer sum instance | `module Sum = struct type t = int let empty = 0 let combine = (+) end` | `struct Sum(i64); impl Monoid for Sum { ... }` |

## Key Insights

1. **Module type → trait:** OCaml's `module type MONOID` bundles a type `t` and values into a single specification. Rust's `trait Monoid` achieves the same by using `Self` as the associated type — cleaner because the trait and the type are the same entity.

2. **First-class modules → monomorphized generics:** OCaml passes the module as a runtime argument (`(module Sum)`), enabling dynamic dispatch at a call site. Rust resolves the monoid implementation at compile time through type inference and monomorphization — zero runtime overhead.

3. **Newtype disambiguation:** OCaml needs separate modules (`Sum`, `Product`) to attach different behavior to `int`. Rust needs wrapper newtypes (`Sum(i64)`, `Product(i64)`) for the same reason — Rust's orphan rules prevent implementing the same trait twice for `i64` directly.

4. **`fold` symmetry:** Both languages express `concat_all` as a left fold with `empty` as the accumulator. The structure is identical; only the syntax differs (`List.fold_left f z xs` vs `xs.fold(z, f)`).

5. **Slice pattern matching:** The recursive Rust version uses `[x, rest @ ..]` — a direct syntactic parallel to OCaml's `x :: rest` — making the structural recursion on sequences visually equivalent between the two languages.

## When to Use Each Style

**Use idiomatic Rust when:** combining a collection of values (sums, products, string joins, boolean reductions) — the `fold`-based version is O(n) with no allocations and composes naturally with iterators.

**Use recursive Rust when:** teaching the OCaml parallel explicitly, or when operating on recursive data structures where the fold mirrors the structural recursion of the type itself.
