# 1051: Fibonacci with HashMap Memoization

**Difficulty:** Intermediate
**Category:** Dynamic Programming / Memoization
**Concept:** Top-down memoization using hash maps to cache recursive Fibonacci results
**Key Insight:** OCaml's Hashtbl and Rust's HashMap both enable O(n) memoized Fibonacci, but Rust's ownership rules make the memoization closure pattern more complex — requiring explicit lifetime management or interior mutability patterns.
