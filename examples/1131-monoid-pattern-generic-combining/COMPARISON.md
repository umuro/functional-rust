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

module Sum     = struct type t = int  let empty = 0    let combine = (+) end
module Product = struct type t = int  let empty = 1    let combine = ( * ) end
module Concat  = struct type t = string let empty = "" let combine = (^) end
module All     = struct type t = bool let empty = true let combine = (&&) end

let () =
  Printf.printf "sum: %d\n"     (concat_all (module Sum)     [1;2;3;4;5]);
  Printf.printf "product: %d\n" (concat_all (module Product) [1;2;3;4;5]);
  Printf.printf "concat: %s\n"  (concat_all (module Concat)  ["hello";" ";"world"]);
  Printf.printf "all: %b\n"     (concat_all (module All)     [true; true; false])
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

pub struct Sum(pub i32);
impl Monoid for Sum {
    fn empty() -> Self { Sum(0) }
    fn combine(self, other: Self) -> Self { Sum(self.0 + other.0) }
}

// … Product, Concat, All similarly

let sum = concat_all([Sum(1), Sum(2), Sum(3), Sum(4), Sum(5)]);  // Sum(15)
```

### Rust (functional/recursive)

```rust
pub fn concat_all_recursive<M: Monoid>(items: Vec<M>) -> M {
    fn go<M: Monoid>(acc: M, mut rest: Vec<M>) -> M {
        if rest.is_empty() {
            acc
        } else {
            let head = rest.remove(0);
            go(M::combine(acc, head), rest)
        }
    }
    go(M::empty(), items)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Interface declaration | `module type MONOID = sig type t; val empty : t; val combine : t -> t -> t end` | `trait Monoid { fn empty() -> Self; fn combine(self, other: Self) -> Self; }` |
| Generic fold | `concat_all (module M : MONOID with type t = a) lst` | `concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M` |
| Sum identity | `val empty = 0` | `fn empty() -> Self { Sum(0) }` |
| Combine | `val combine = (+)` | `fn combine(self, other: Self) -> Self { Sum(self.0 + other.0) }` |
| Call-site witness | `concat_all (module Sum) [1;2;3]` | `concat_all([Sum(1), Sum(2), Sum(3)])` — type inferred |

## Key Insights

1. **First-class modules vs traits:** OCaml passes the monoid *instance* explicitly as a module value at the call site. Rust encodes the same information in the *type* of the items — the compiler resolves the `impl Monoid` statically, with zero runtime cost.

2. **Multiple instances for the same base type:** Both `i32` and `i32` can be a monoid in two ways (sum, product). OCaml creates two modules that both have `type t = int`. Rust cannot have two `impl Monoid for i32` blocks in the same crate (orphan rules + coherence), so newtypes `Sum(i32)` and `Product(i32)` each get their own `impl`. This is idiomatic and adds zero overhead.

3. **Fold equivalence:** `List.fold_left M.combine M.empty lst` ↔ `items.into_iter().fold(M::empty(), M::combine)`. Both evaluate left-to-right, accumulating into a single value. The structure is identical; only the syntax differs.

4. **`empty` as function vs value:** In OCaml `M.empty` is a stored value. In Rust `M::empty()` is a function call — necessary because Rust traits cannot have associated constants of arbitrary type without a bound. The semantic effect is the same; the function produces the identity on every invocation.

5. **Ownership and move semantics:** `combine(self, other: Self)` consumes both operands, matching the mathematical notion that combining two values produces a new one. No allocation happens for `Copy` types (`Sum`, `Product`, `All`); `Concat` moves its `String` into the new wrapper, reusing the heap buffer via `String + &str`.

## When to Use Each Style

**Use idiomatic Rust when:** you have a collection and just need the combined result — `concat_all` with a flat array or any iterator is the natural expression.

**Use recursive Rust when:** you are learning the OCaml-to-Rust correspondence explicitly, or when you need to process elements one at a time with early exit (though for that, `try_fold` is even better).
