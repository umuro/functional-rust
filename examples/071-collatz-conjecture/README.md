📖 **[View on hightechmind.io →](https://hightechmind.io/rust/071-collatz-conjecture)**

---

# Example 071: Collatz Conjecture

**Difficulty:** ⭐
**Category:** Recursion
**Concept:** Computing the Collatz (3n+1) sequence step count. Demonstrates simple recursion with guards, a Result-typed safe API, and the iterative equivalent. A classic exercise for pattern matching and error handling.
**OCaml → Rust insight:** Both languages express the Collatz logic identically with pattern matching; the difference is OCaml uses `if/else` chains while Rust's `match` with guards (`n if n % 2 == 0`) is more idiomatic.
