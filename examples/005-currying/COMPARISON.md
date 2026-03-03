# Currying and Partial Application: OCaml vs Rust

## The Core Insight
Currying is automatic and pervasive in OCaml — every function is curried, and partial application is free. Rust requires explicit closures, but the resulting pattern is equally powerful. Understanding this difference reveals how each language thinks about functions as values.

## OCaml Approach
In OCaml, `let add a b = a + b` is syntactic sugar for `let add = fun a -> fun b -> a + b`. Every multi-argument function is actually a chain of single-argument functions:
```ocaml
let add5 = add 5           (* partially apply: returns fun b -> 5 + b *)
let result = add5 3         (* 8 *)
let big = List.filter (greater_than 4) [1;5;3;8]  (* [5;8] *)
```
This makes function composition and predicate building incredibly natural. The `|>` pipe operator complements currying beautifully.

## Rust Approach
Rust functions are not automatically curried. To achieve partial application, you return closures:
```rust
fn add(n: i64) -> impl Fn(i64) -> i64 {
    move |x| x + n
}
let add5 = add(5);
```
The `move` keyword captures `n` by value (ownership transfer). For generic partial application, you need explicit lifetime and trait bounds — more verbose but equally expressive.

## Key Differences
| Aspect | OCaml | Rust |
|--------|-------|------|
| Currying | Automatic (all functions) | Manual (return closures) |
| Partial application | Free (`f a`) | Explicit (`move \|x\| ...`) |
| Closure capture | Automatic (GC) | `move` or borrow (ownership) |
| Function types | `'a -> 'b -> 'c` | `Fn(A) -> impl Fn(B) -> C` |
| Type inference | Full | Partial (return types need annotation) |
| Performance | Allocation per partial app | Zero-cost when inlined |
| Trait bounds | None needed | `Fn`/`FnMut`/`FnOnce` |

## What Rust Learners Should Notice
- **`Fn` vs `FnMut` vs `FnOnce`**: These three traits determine how a closure captures its environment. `Fn` borrows immutably, `FnMut` borrows mutably, `FnOnce` consumes. OCaml doesn't have this distinction (GC handles it).
- **`impl Fn` vs `Box<dyn Fn>`**: `impl Fn` is monomorphized (zero-cost, can't be stored in collections). `Box<dyn Fn>` is heap-allocated (can be stored, has runtime cost). OCaml closures are always heap-allocated.
- **`move` is the key**: Without `move`, closures borrow from their environment. With `move`, they take ownership. This is the ownership system at work in closures.
- **Curried APIs are less idiomatic in Rust**: While possible, Rust code typically uses method chaining and builder patterns rather than curried functions.

## Further Reading
- [The Rust Book — Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [OCaml Manual — Functions](https://v2.ocaml.org/manual/expr.html#ss:expr-function)
- [Rust Reference — Closure types](https://doc.rust-lang.org/reference/types/closure.html)
