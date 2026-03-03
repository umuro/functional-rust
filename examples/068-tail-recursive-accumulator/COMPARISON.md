# Tail-Recursive Accumulator Pattern: OCaml vs Rust

## The Core Insight
This pattern reveals a fundamental philosophical difference: OCaml optimizes recursion (via TCO) to make it as efficient as loops; Rust provides powerful iterators that make loops as expressive as recursion. Both achieve stack safety, but through opposite strategies.

## OCaml Approach
OCaml guarantees tail-call optimization: if a function's last action is a recursive call, the compiler reuses the current stack frame. The accumulator pattern (`let rec go acc = function ...`) transforms any linear recursion into tail position. This is essential for processing large lists — without it, `sum_naive` on 100K elements would overflow the stack. The pattern is so fundamental that OCaml programmers internalize it early.

## Rust Approach
Rust does NOT guarantee TCO (though LLVM sometimes applies it as an optimization). Instead, idiomatic Rust uses iterators: `list.iter().sum()` is stack-safe by design because it compiles to a simple loop. The iterator approach is also more composable — you can chain `.filter()`, `.map()`, `.take()` etc. For Fibonacci, a simple `for` loop with mutable accumulators is clearest. Recursive patterns still work for small inputs or tree structures.

## Side-by-Side
| Concept | OCaml | Rust |
|---------|-------|------|
| Stack safety | Guaranteed TCO | Iterators / loops |
| Sum | `let rec go acc = function ...` | `iter().sum()` |
| Reverse | `go (h :: acc) t` | `iter().rev().collect()` |
| Fibonacci | `go a b (n-1)` | `for` loop with mutation |
| Large input | TCO handles it | Iterators handle it |
| Style | Recursion is idiomatic | Iteration is idiomatic |

## What Rust Learners Should Notice
- Don't write recursive functions in Rust for linear data — use iterators. They're zero-cost abstractions that compile to the same machine code as hand-written loops
- The accumulator pattern is still useful for tree structures where iterators don't naturally apply
- Rust's slice patterns (`[h, rest @ ..]`) are powerful for recursive code but less common than iterator chains
- `iter().sum()` requires the `Sum` trait — Rust's trait system handles the dispatch
- When you see OCaml's tail recursion, think "what iterator chain would express this in Rust?"

## Further Reading
- [The Rust Book — Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Cornell CS3110 — Tail Recursion](https://cs3110.github.io/textbook/chapters/data/lists.html)
