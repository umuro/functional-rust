# Fibonacci with HashMap Memoization — Comparison

## Core Insight
Memoization transforms exponential O(2^n) Fibonacci into O(n). Both languages support hash-based caching, but OCaml's mutable Hashtbl is simpler to thread through recursion than Rust's HashMap due to borrowing constraints.

## OCaml Approach
- `Hashtbl.create` for imperative memoization — simple and direct
- `Map` module for a more functional flavor using immutable maps with a ref cell
- CPS (continuation-passing style) variant shows functional composition with memoization
- The `find_opt` / pattern match idiom is clean and readable

## Rust Approach
- `HashMap` passed as `&mut` parameter — explicit but requires threading through calls
- Closure-based memoizer captures HashMap in a `move` closure
- Generic `Memoize` struct demonstrates the complexity of self-referential memoization in Rust
- Rust's borrow checker makes recursive memoization harder: you can't borrow `cache` mutably while also recursing

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Hash map type | `Hashtbl.t` (mutable) | `HashMap<K, V>` |
| Memoization pattern | `find_opt` + `add` | `get` + `insert` |
| Closure capture | Trivial — closures capture freely | Requires `move` + ownership management |
| Recursive memo | Natural — just call `fib` recursively | Borrow checker friction with `&mut` |
| Immutable variant | `Map` module + `ref` cell | Not idiomatic — would need `RefCell` |
| Performance | GC-managed hash table | Zero-cost HashMap with predictable perf |
