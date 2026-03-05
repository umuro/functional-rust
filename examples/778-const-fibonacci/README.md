📖 **[View on hightechmind.io →](https://hightechmind.io/rust/778-const-fibonacci)**

---

# 778: Fibonacci Computed at Compile Time

**Difficulty:** 3  **Level:** Intermediate

A table of the first 93 Fibonacci numbers computed by the compiler, stored in the binary's read-only segment — O(1) lookup at zero runtime cost.

## The Problem This Solves

Fibonacci numbers appear in surprising places: Bloom filter sizing, hash table growth factors, fractal rendering, art and architecture, financial models. Computing them at runtime is straightforward, but it's wasteful when you always need the same precomputed values. `fib(80)` is always 23,416,728,348,161,557,660 — computing it every time is pointless.

The recursive implementation is instructive but exponentially slow: `fib(40)` requires ~1 billion recursive calls. The memoized iterative version is O(n) but allocates a `Vec`. For a table of fixed size (the first 93 values fit in `u64`), the right answer is to compute the entire table at compile time and embed it in the binary.

This also demonstrates a subtle point: `const fn` cannot use `for` loops over iterators yet, but it *can* use `while` loops and direct array indexing. Knowing this constraint shapes how you write compile-time code.

## The Intuition

This is the same idea as a sin/cos lookup table in embedded graphics, or a prime sieve embedded in a number theory library. You compute values that are mathematically fixed, store them as a `const` array, and at runtime just do an array lookup.

`FIB_TABLE[n]` is a single memory read. Compared to even the memoized runtime version (which allocates a `Vec`), it's faster, uses less memory, and has no allocation cost. Compared to the recursive version, the speedup is astronomical.

## How It Works in Rust

```rust
// const fn: iterative — loops are required (recursive const fn depth is limited)
const fn fib_iter(n: u32) -> u64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut i = 2;
    while i <= n {               // MUST be while, not for
        let c = a + b;
        a = b;
        b = c;
        i += 1;
    }
    b
}

// Build the entire table at compile time using a const block
// fib(93) = 12,200,160,415,121,876,738 — the last value that fits in u64
const FIB_TABLE: [u64; 93] = {
    let mut t = [0u64; 93];
    t[0] = 0;
    t[1] = 1;
    let mut i = 2usize;
    while i < 93 {               // while loop — no iterators in const yet
        t[i] = t[i-1] + t[i-2]; // same recurrence, but evaluated by the compiler
        i += 1;
    }
    t                            // the array becomes the value of the const expression
};

// O(1) lookup — the compiler already did the work
pub fn fib(n: usize) -> Option<u64> {
    FIB_TABLE.get(n).copied()    // None if n >= 93 (would overflow u64)
}

// For comparison — runtime versions you should NOT use for fixed lookups:
pub fn fib_recursive(n: u32) -> u64 {
    match n { 0 => 0, 1 => 1, n => fib_recursive(n-1) + fib_recursive(n-2) }
    // fib_recursive(40) ≈ 1 billion calls — don't do this
}

// When you need truly large Fibonacci numbers (n > 92), use matrix exponentiation
// F(n) via 2×2 matrix: O(log n), works for arbitrary precision types
fn fib_matrix(n: u64) -> u64 { ... }  // see example.rs for full implementation
```

The const block `{ let mut t = ...; while ...; t }` is a *const block expression* — the entire block is evaluated at compile time, and the final expression `t` is the value. This is how you build complex constants that can't be expressed as a single formula.

```rust
// Verification — the compiler checks this at compile time:
const _: () = assert!(FIB_TABLE[10] == 55);    // 0,1,1,2,3,5,8,13,21,34,55
const _: () = assert!(FIB_TABLE[20] == 6765);
// If these fail, the BUILD fails — not a runtime error
```

Key points:
- `[u64; 93]` — only 93 × 8 = 744 bytes in the binary's `.rodata` section
- `FIB_TABLE.get(n).copied()` returns `None` for `n >= 93` — clean bounds handling
- The const block `{ ... }` computes the table during compilation — `rustc` runs the logic
- `while` not `for` — iterator-based for loops are not const-stable yet (1.84 as of this writing)
- For Fibonacci numbers beyond `fib(92)`, use big integers (`num-bigint` crate) or matrix exponentiation

## What This Unlocks

- **O(1) mathematical lookups**: primes, factorials, Catalan numbers, Pascal's triangle rows — anything with a fixed table beats computing at runtime
- **Embedded/no-std**: the table is in `.rodata` — no stack, no heap, no allocation at startup
- **Compile-time correctness**: `const _: () = assert!(FIB_TABLE[10] == 55)` fails the build if the table is wrong — verified before deployment

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Compile-time table | Module-level `let` (run at startup) | `const FIB_TABLE: [u64; 93] = { ... }` |
| Const iteration | N/A | `while` loop with index — `for` not const-stable |
| Table in binary | Startup-initialized `Array.t` | `const` array in `.rodata` — zero runtime cost |
| Overflow protection | `Int64` arithmetic | Explicit `u64` — `fib(93)` overflows, hence `[u64; 93]` |
| Bounds safety | `Array.get` raises `Invalid_argument` | `.get(n).copied()` returns `Option<u64>` |
| Very large Fibonacci | Zarith arbitrary precision | `num-bigint` crate or matrix exponentiation |
