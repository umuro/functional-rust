# Pattern Matching: OCaml vs Rust

## The Core Insight
Pattern matching is where OCaml and Rust feel most alike. Both languages use algebraic data types (OCaml variants / Rust enums) with exhaustive matching — the compiler ensures every case is handled. This eliminates entire classes of bugs that plague languages without sum types.

## OCaml Approach
OCaml's variant types and `match`/`function` expressions are the language's crown jewel:
```ocaml
type shape = Circle of float | Rectangle of float * float

let area = function
  | Circle r -> Float.pi *. r *. r
  | Rectangle (w, h) -> w *. h
```
Guard clauses (`when`) add conditional logic within patterns. The compiler warns on non-exhaustive matches and unused cases — a safety net that catches bugs at compile time.

## Rust Approach
Rust's `enum` + `match` is a direct descendant of ML-family pattern matching:
```rust
enum Shape { Circle(f64), Rectangle(f64, f64) }

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => PI * r * r,
        Shape::Rectangle(w, h) => w * h,
    }
}
```
Rust adds ownership to the mix: matching can move, borrow, or copy inner values. The `ref` keyword and `&` patterns control this explicitly.

## Key Differences
| Aspect | OCaml | Rust |
|--------|-------|------|
| Sum types | `type t = A \| B of int` | `enum T { A, B(i32) }` |
| Exhaustiveness | Compiler warning | Compiler error (stricter) |
| Guards | `when` clause | `if` guard |
| Binding | Automatic copy | Move/borrow semantics |
| Nested match | Natural | Natural |
| Or-patterns | `A \| B -> ...` | `A \| B => ...` |
| Wildcard | `_` | `_` |
| `function` sugar | Yes (one-arg match) | No equivalent |

## What Rust Learners Should Notice
- **Exhaustiveness is enforced**: Unlike OCaml's warning, Rust makes non-exhaustive matches a hard error. This is stricter and safer.
- **Ownership in patterns**: When you match on `Shape::Circle(r)`, `r` is a copy (for `f64`) or a move (for `String`). Use `&Shape::Circle(r)` or `ref` to borrow.
- **No `function` keyword**: OCaml's `let f = function | A -> ... | B -> ...` has no Rust equivalent. You always write `fn f(x: T) -> U { match x { ... } }`.
- **Guards work the same**: `Some(x) if x > 0 => ...` in Rust mirrors OCaml's `Some x when x > 0 -> ...`.

## Further Reading
- [The Rust Book — Patterns and Matching](https://doc.rust-lang.org/book/ch18-00-patterns-and-matching.html)
- [OCaml Manual — Pattern Matching](https://v2.ocaml.org/manual/patterns.html)
- [Rust Reference — Enum types](https://doc.rust-lang.org/reference/items/enumerations.html)
