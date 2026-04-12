# OCaml vs Rust: Monoid Pattern

## Side-by-Side Code

### OCaml

```ocaml
module type MONOID = sig
  type t
  val empty   : t
  val combine : t -> t -> t
end

module Sum : MONOID with type t = int = struct
  type t = int
  let empty = 0
  let combine x y = x + y
end

module Product : MONOID with type t = int = struct
  type t = int
  let empty = 1
  let combine x y = x * y
end

(* Generic reduce using first-class modules *)
let reduce (type a) (module M : MONOID with type t = a) lst =
  List.fold_left M.combine M.empty lst

let () =
  assert (reduce (module Sum)     [1;2;3;4;5] = 15);
  assert (reduce (module Product) [1;2;3;4;5] = 120)
```

### Rust (idiomatic)

```rust
pub trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sum(pub i32);

impl Monoid for Sum {
    fn empty() -> Self { Sum(0) }
    fn combine(self, other: Self) -> Self { Sum(self.0 + other.0) }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Product(pub i32);

impl Monoid for Product {
    fn empty() -> Self { Product(1) }
    fn combine(self, other: Self) -> Self { Product(self.0 * other.0) }
}

pub fn reduce_monoid<T: Monoid + Clone>(items: &[T]) -> T {
    items.iter().cloned().fold(T::empty(), |acc, x| acc.combine(x))
}
```

### Rust (functional/recursive)

```rust
pub fn reduce_recursive<T: Monoid + Clone>(items: &[T]) -> T {
    match items {
        [] => T::empty(),
        [x] => x.clone(),
        [head, rest @ ..] => head.clone().combine(reduce_recursive(rest)),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| abstraction mechanism | `module type MONOID` (signature) | `trait Monoid` |
| empty | `val empty : t` | `fn empty() -> Self` |
| combine | `val combine : t -> t -> t` | `fn combine(self, other: Self) -> Self` |
| dispatch | first-class module passed at call site | monomorphized via trait bound at compile time |
| disambiguation for `int` | `module Sum` and `module Product` (different namespaces) | `struct Sum(i32)` and `struct Product(i32)` (newtype wrappers) |

## Key Insights

1. **Abstraction mechanism:** OCaml uses first-class modules — the monoid implementation is passed explicitly as a value at the call site: `reduce (module Sum) list`. Rust uses trait bounds resolved at compile time: `reduce_monoid::<Sum>(items)`. The OCaml approach is more dynamic (you can choose the implementation at runtime); Rust's approach is monomorphized (each instantiation compiles to a specialized function, with zero runtime overhead).
2. **Type disambiguation:** In OCaml, `Sum` and `Product` are distinct modules even though both wrap `int` — module namespaces prevent any conflict. In Rust, you cannot implement the same trait twice for the same type, so `Sum(i32)` and `Product(i32)` are newtype wrappers — thin structs that each get their own `impl Monoid`.
3. **Clone requirement:** OCaml `reduce` uses `List.fold_left` with `M.empty` and `M.combine`, sharing the accumulator via GC. Rust's `reduce_monoid` needs `T: Clone` to call `.cloned()` on the slice iterator, producing owned values for `fold`. The accumulator is moved into each `combine` call rather than shared.
4. **Consuming combine:** Rust's `combine(self, other: Self) -> Self` takes ownership of both operands, modeling the "consume inputs, produce result" functional style. This avoids the need for copying inside `combine` — the values are simply consumed.
5. **Monoid laws:** Neither OCaml nor Rust can enforce the three monoid laws (closure, associativity, identity) in the type system — they are a semantic contract that `impl` authors must uphold. OCaml module sealing doesn't help here either; both languages rely on documentation and tests.

## When to Use Each Style

**Use idiomatic Rust (iterator fold) when:** Combining a slice of `Monoid` values — `iter().cloned().fold(T::empty(), T::combine)` is a zero-overhead, inlined loop in release builds.
**Use recursive Rust when:** Teaching the OCaml parallel, or when the collection is a custom recursive structure (tree, list) rather than a flat slice.
