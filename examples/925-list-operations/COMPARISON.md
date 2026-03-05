# List Operations: OCaml vs Rust

## The Core Insight
List operations are where functional programming shines — expressing computation as recursive transformations rather than indexed loops. OCaml's singly-linked lists with pattern matching feel natural; Rust's `Vec` and iterators offer a different but equally expressive approach, with ownership making aliasing explicit.

## OCaml Approach
OCaml lists are singly-linked and immutable. Pattern matching with `head :: tail` destructures the list naturally:
```ocaml
let rec sum = function
  | [] -> 0
  | head :: tail -> head + sum tail
```
The `::` constructor is O(1) prepend, and tail-recursive versions use an accumulator to avoid stack overflow. Higher-order functions like `map` and `filter` are built the same way — recursive descent with pattern matching.

## Rust Approach
Rust uses `Vec<T>` (contiguous, growable array) rather than linked lists. Iterator chains provide the functional style:
```rust
pub fn sum(list: &[i64]) -> i64 {
    list.iter().sum()
}
```
For recursive patterns, `split_first()` gives `Option<(&T, &[T])>`, mimicking OCaml's head/tail destructuring. Ownership means we must decide: borrow with `&[T]` or consume with `Vec<T>`.

## Key Differences
| Aspect | OCaml | Rust |
|--------|-------|------|
| List type | Singly-linked (immutable) | Vec (contiguous array) |
| Destructuring | `h :: t` pattern | `split_first()` method |
| Memory | GC handles everything | Ownership: borrow or clone |
| Prepend | O(1) with `::` | O(n) with `insert(0, x)` |
| Append | O(n) with `@` | O(1) amortized with `push` |
| Tail recursion | Compiler optimizes TCO | Not guaranteed — prefer iterators |
| Idiomatic style | Recursion + pattern matching | Iterator chains |

## What Rust Learners Should Notice
- **Iterators replace recursion**: Rust's iterator combinators (`map`, `filter`, `fold`, `sum`) are zero-cost abstractions — they compile to the same code as hand-written loops.
- **No free `head :: tail`**: Rust slices don't destructure like OCaml lists. `split_first()` is the closest equivalent, returning `Option<(&T, &[T])>`.
- **Ownership matters for append**: In OCaml, `append [1;2] [3;4]` shares the tail. In Rust, you must clone or move elements.
- **Tail recursion isn't guaranteed**: Unlike OCaml, Rust doesn't guarantee TCO. For large lists, prefer iterators or explicit loops.

## Further Reading
- [The Rust Book — Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [OCaml Manual — Lists](https://v2.ocaml.org/api/List.html)
- [Rust by Example — Vectors](https://doc.rust-lang.org/rust-by-example/std/vec.html)
