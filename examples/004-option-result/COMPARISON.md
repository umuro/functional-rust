# Option and Result: OCaml vs Rust

## The Core Insight
Both languages solve the "billion dollar mistake" (null references) with sum types: `Option` for optional values and `Result` for computations that may fail. The compiler forces you to handle both cases — no more `NullPointerException` or unhandled exceptions. This is perhaps the strongest argument for ML-family type systems.

## OCaml Approach
OCaml's `option` type (`None | Some 'a`) and `result` type (`Ok 'a | Error 'b`) work with pattern matching and the pipe operator:
```ocaml
let safe_div a b =
  if b = 0.0 then None else Some (a /. b)

(* Chain with |> and Option.map *)
safe_div 10.0 2.0 |> Option.map (fun x -> x *. x)
```
OCaml also has exceptions (`raise`, `try...with`), giving you a choice. Idiomatic OCaml increasingly favors `result` for recoverable errors.

## Rust Approach
Rust has `Option<T>` and `Result<T, E>` as core types with rich combinator methods:
```rust
fn safe_div(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

// The ? operator propagates errors — Rust's monadic bind
fn sqrt_of_div(a: f64, b: f64) -> Result<f64, MathError> {
    let q = safe_div(a, b).ok_or(MathError::DivisionByZero)?;
    checked_sqrt(q)
}
```
Rust has **no exceptions** — `Result` is the only way. The `?` operator makes error propagation concise.

## Key Differences
| Aspect | OCaml | Rust |
|--------|-------|------|
| Optional values | `'a option` | `Option<T>` |
| Error handling | `('a, 'b) result` + exceptions | `Result<T, E>` only |
| Chaining | `\|>` pipe + `Option.map`/`bind` | `.map()` / `.and_then()` / `?` |
| Error propagation | Manual matching or `Result.bind` | `?` operator (sugar for match) |
| Unwrapping | `Option.get` (raises) | `.unwrap()` (panics) |
| Default values | `Option.value ~default:x` | `.unwrap_or(x)` |
| Conversion | `Option.to_result` | `.ok_or(err)` / `.ok()` |

## What Rust Learners Should Notice
- **The `?` operator is magical**: It replaces what would be verbose match expressions. `let x = expr?;` unwraps `Ok`/`Some` or returns early with `Err`/`None`.
- **No exceptions in Rust**: OCaml lets you `raise` and `try/with`. Rust forces `Result` everywhere — more verbose but no hidden control flow.
- **Combinators are the same idea**: OCaml's `Option.map f x` is Rust's `x.map(f)`. Method syntax vs function syntax, same concept.
- **`unwrap()` is a code smell**: Just like OCaml's `Option.get` can raise, Rust's `.unwrap()` panics. Both should be avoided in production code.

## Further Reading
- [The Rust Book — Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [OCaml Manual — Option](https://v2.ocaml.org/api/Option.html)
- [Rust by Example — Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
