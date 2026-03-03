# Difference List — Comparison

## Core Insight
A difference list represents a list as a function, enabling O(1) append via function composition. In OCaml, functions are first-class and GC-managed. In Rust, closures must be boxed (`Box<dyn FnOnce>`) and are consumed on use.

## OCaml Approach
- `type 'a dlist = 'a list -> 'a list` — just a type alias for a function
- `let append a b = fun rest -> a (b rest)` — composition is trivial
- `Fun.id` for empty list
- Functions are freely copyable/shareable via GC

## Rust Approach
- `Box<dyn FnOnce(Vec<T>) -> Vec<T>>` — heap-allocated, consumed on call
- Each DList can only be used once (FnOnce, not Fn)
- `move` closures capture ownership of data
- Alternative: VecBuilder collects chunks for later assembly

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Type | `'a list -> 'a list` | `Box<dyn FnOnce(Vec<T>) -> Vec<T>>` |
| Append | Function composition | Box composition (heap alloc) |
| Reuse | Freely reusable | Single-use (FnOnce) |
| Empty | `Fun.id` | `Box::new(identity)` |
| Overhead | GC for closures | Box allocation per compose |

## Learner Notes
- Difference lists are more natural in GC languages where functions are cheap
- In Rust, `VecBuilder` (collecting chunks) is often more practical
- `FnOnce` means each DList is consumed when converted to Vec
- The `'static` bound on `T` is needed because closures must own their data
