📖 **[View on hightechmind.io →](https://hightechmind.io/rust/256-memoization-fibonacci)**

---

# Example 256: Memoization — Fibonacci with Hashtable Cache
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement transparent memoization using a hash-table cache wrapper, demonstrated
with Fibonacci — the recursive calls hit the cache instead of recomputing.

## Learning Outcomes

- How Rust makes mutable state explicit (`&mut self`) vs OCaml's hidden `Hashtbl`
- The `RefCell<HashMap>` pattern for interior mutability inside closures
- Why Rust cannot directly express OCaml's `let rec … and` mutual recursion for closures, and three idiomatic workarounds
- `thread_local!` as Rust's equivalent of module-level mutable state

## OCaml Approach

OCaml's `memoize` is a generic HOF that captures a `Hashtbl` in a closure and
returns a new function that checks the table before calling the original.
Recursive memoization uses `let rec … and` mutual recursion to wire `fib'`
and `memo_fib` together at binding time — a language feature with no direct
Rust equivalent.

## Rust Approach

Rust offers three clean patterns.  The **struct-based** approach owns the
`HashMap` on the heap and exposes a `&mut self` method — mutable state is
visible in the type signature.  The **HOF approach** mirrors OCaml's `memoize`
with `RefCell<HashMap>` for interior mutability, then threads an explicit cache
reference through a named inner function to enable recursion.  The
**thread-local** approach uses `thread_local!` to hide the cache entirely,
giving call-site transparency identical to the OCaml version.

## Key Differences

1. **Mutable state visibility:** OCaml hides the `Hashtbl` inside a closure; idiomatic Rust surfaces it via `&mut self` or `RefCell`
2. **Mutual recursion:** OCaml's `let rec … and` allows closures to reference each other at definition time; Rust requires explicit cache parameter passing or global state
3. **Interior mutability:** OCaml's GC handles aliasing freely; Rust uses `RefCell` to borrow-check at runtime inside otherwise-immutable closures
4. **Thread safety:** OCaml's `Hashtbl` is not thread-safe; Rust's `thread_local!` makes the scope explicit, and `Mutex<HashMap>` would be needed for sharing

## Exercises

1. Generalize the memoization approach into a reusable `memoize` higher-order function that wraps any `Fn(u64) -> u64` with a `HashMap` cache.
2. Implement memoized mutual recursion: two functions `is_even` and `is_odd` that call each other, each with its own cache, and verify they produce correct results for large inputs without redundant calls.
3. Implement bottom-up dynamic programming for Fibonacci using a fixed-size array instead of a `HashMap`, compare memory and performance with the top-down memoized version, and explain the trade-offs.
