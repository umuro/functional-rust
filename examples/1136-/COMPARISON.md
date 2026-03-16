# OCaml vs Rust: Monoid — Fold with Closures and Traits

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

let () =
  Printf.printf "sum:     %d\n" (concat_all (module Sum)     [1;2;3;4;5]);
  Printf.printf "product: %d\n" (concat_all (module Product) [1;2;3;4;5]);
  Printf.printf "concat:  %s\n" (concat_all (module Concat)  ["hello";" ";"world"]);
  Printf.printf "all:     %b\n" (concat_all (module All)     [true; true; false])
```

### Rust — Solution 1: Closure-based (direct OCaml analogy)

```rust
// OCaml's (module M : MONOID with type t = a) becomes two plain arguments:
// the identity value and the combining function.
pub fn concat_with<T>(
    empty: T,
    combine: impl Fn(T, T) -> T,
    items: impl IntoIterator<Item = T>,
) -> T {
    items.into_iter().fold(empty, combine)
}

// Call sites mirror OCaml's `concat_all (module Sum) [...]`:
let sum     = concat_with(0,    |a, b| a + b,   [1, 2, 3, 4, 5]);  // 15
let product = concat_with(1,    |a, b| a * b,   [1, 2, 3, 4, 5]);  // 120
let text    = concat_with("".to_owned(), |a, b| a + &b, [...]); // "hello world"
let all     = concat_with(true, |a, b| a && b,  [true, true, false]); // false
```

### Rust — Solution 2: Trait-based (idiomatic Rust)

```rust
pub trait Monoid: Sized {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

pub fn concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M {
    items.into_iter().fold(M::empty(), M::combine)
}

// Newtypes replace OCaml module names — Sum and Product can both wrap i32
pub struct Sum(pub i32);
impl Monoid for Sum {
    fn empty() -> Self { Sum(0) }
    fn combine(self, other: Self) -> Self { Sum(self.0 + other.0) }
}

pub struct Product(pub i32);
impl Monoid for Product {
    fn empty() -> Self { Product(1) }
    fn combine(self, other: Self) -> Self { Product(self.0 * other.0) }
}
```

### Rust — Solution 3: std::iter::Sum / Product

```rust
// The stdlib already knows about numeric monoids:
let sum:     i32 = [1, 2, 3, 4, 5].iter().sum();      // Sum monoid built-in
let product: i32 = [1, 2, 3, 4, 5].iter().product();  // Product monoid built-in
```

## Type Signatures

| Concept | OCaml | Rust (closure) | Rust (trait) |
|---------|-------|----------------|--------------|
| Interface | `module type MONOID` | closure pair `(T, Fn(T,T)->T)` | `trait Monoid` |
| Generic fold | `concat_all (module M) lst` | `concat_with(empty, combine, items)` | `concat_all<M: Monoid>(items)` |
| Sum identity | `val empty = 0` | literal `0` | `fn empty() -> Self { Sum(0) }` |
| Sum combine | `val combine = (+)` | `\|a, b\| a + b` | `fn combine(self, other: Self) -> Self { Sum(self.0 + other.0) }` |
| Multiple instances of same base type | two modules with `type t = int` | two calls with different `empty`/`combine` | two newtypes `Sum(i32)`, `Product(i32)` |

## Key Insights

1. **First-class modules ≈ closure pairs:** OCaml's `(module M : MONOID with type t = a)` packages an identity value and a combining function under a name.  Rust closures achieve the same effect by passing these two pieces explicitly — the `concat_with` signature is a direct structural translation of the OCaml module type.

2. **Traits encode static dispatch:** The trait approach moves the `empty` and `combine` implementations from the call site into the type itself.  Rust's compiler monomorphises `concat_all<Sum>` and `concat_all<Product>` into two specialised functions at compile time — equivalent to C++ templates — with zero runtime overhead.  OCaml's first-class modules use runtime dispatch via a vtable.

3. **Newtypes solve the coherence problem:** OCaml can have `module Sum` and `module Product` both with `type t = int` because modules are distinguished by name.  Rust's orphan and coherence rules prohibit two `impl Monoid for i32` in the same crate, so newtypes `Sum(i32)` and `Product(i32)` each carry their own `impl` without conflict.

4. **`empty` as function vs. stored value:** In OCaml `M.empty` is a stored module field evaluated once at module definition time.  In Rust `M::empty()` is a function producing the identity on each call.  For purely functional (no side-effects) identities the effect is identical; the function form is necessary because Rust trait associated items cannot be arbitrary values of unconstrained type.

5. **Stdlib monoids:** For the numeric cases, `std::iter::Sum` and `std::iter::Product` are pre-built monoids; calling `.sum()` or `.product()` on any iterator of a numeric type is idiomatic Rust and has no boilerplate.

## When to Use Each Style

**Use the closure-based style when:** you need a one-off fold with an unusual identity/combine that doesn't justify a new type, or when teaching the correspondence with OCaml's first-class modules.

**Use the trait-based style when:** the same monoid instance is reused in many places, or when you want the type system to enforce monoid laws at boundaries (e.g., in generic data structures like segment trees or parallel fold).

**Use `std::iter::sum()` / `.product()` when:** you just need numeric aggregation and both the identity and combine are the standard arithmetic ones — this is the most idiomatic Rust for that case.
