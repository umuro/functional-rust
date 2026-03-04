# 127: Const Functions

**Difficulty:** ⭐⭐  **Level:** Intermediate

Write functions that the compiler evaluates at compile time so their results are baked directly into the binary — zero runtime cost.

## The Problem This Solves

Some values in your program are mathematically fixed — the 10th Fibonacci number, a lookup table of squares, a buffer size computed from a protocol constant. You could hardcode them, but that's error-prone and hard to maintain. You could compute them at program startup, but that adds initialization time and the values still sit in RAM. You could use a build script to generate code, but that's complex.

What you actually want is to write the logic once as an ordinary function, mark it `const`, and have the compiler run it during compilation. The result is a constant embedded in your binary — as if you'd typed the answer yourself, but verified by the computation.

This matters most in embedded systems (no heap, no std), hot paths where initialization latency matters, and anywhere you build lookup tables. `const fn` bridges the gap between "readable code" and "zero-overhead constants."

## The Intuition

A `const fn` is a function with two modes: at compile time, the compiler evaluates it and substitutes the result inline; at runtime, it works like any regular function. You get to reuse the same logic in both contexts.

The key constraint: everything inside a `const fn` must be deterministic and safe to run without a running OS — no heap allocation, no I/O, no function pointers. Within those limits, loops, arithmetic, and array manipulation all work. In practice, most mathematical functions qualify.

Think of it as giving the compiler permission to "pre-run" your function. When you write `const FIB_10: u64 = fibonacci(10);`, the compiler runs `fibonacci(10)` right there and replaces the whole expression with `55`.

## How It Works in Rust

```rust
// Mark a function `const fn` to allow compile-time evaluation
const fn fibonacci(n: u64) -> u64 {
    let mut a = 0u64;
    let mut b = 1u64;
    let mut i = 0;
    while i < n {          // loops work in const fn since Rust 1.46
        let temp = b;
        b = a + b;
        a = temp;
        i += 1;
    }
    a
}

// The compiler evaluates this right now — FIB_10 is literally 55 in the binary
const FIB_10: u64 = fibonacci(10);
const FIB_20: u64 = fibonacci(20);

// Build a lookup table at compile time — no runtime initialization
const fn build_square_table() -> [usize; 256] {
    let mut table = [0usize; 256];
    let mut i = 0;
    while i < 256 {
        table[i] = i * i;  // i * i, not square(i) — calling other const fns is fine too
        i += 1;
    }
    table
}

// This entire 256-element array is embedded in the binary as a data section
const SQUARE_TABLE: [usize; 256] = build_square_table();
```

Usage:
```rust
fn main() {
    println!("{}", FIB_10);              // just reads a constant — no computation
    println!("{}", SQUARE_TABLE[16]);    // table lookup — single memory read
}
```

The same `fibonacci` function also works at runtime:
```rust
let n: u64 = get_user_input();
let result = fibonacci(n);   // computed at runtime when n isn't known at compile time
```

## What This Unlocks

- **Lookup tables** — precompute sin/cos tables, CRC tables, hash seeds, anything indexed by a small integer. No runtime init, no lazy_static, no Mutex.
- **Protocol constants** — derive buffer sizes, magic numbers, and version fields from readable formulas rather than hardcoded hex literals.
- **`#[no_std]` embedded firmware** — `const fn` is one of the few ways to do real computation without a heap or OS, making Rust viable for microcontrollers.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Compile-time eval | Module-level `let` runs at *startup*, not compile time | `const fn` + `const` binding: runs during *compilation*, in the binary |
| Lookup tables | `let table = Array.init 256 (fun i -> i*i)` — heap, runtime init | `const SQUARES: [usize; 256] = build_squares()` — data segment, zero runtime cost |
| Restrictions | Any expression allowed in module scope | `const fn` cannot allocate, use trait objects, or call non-const functions |
| Runtime reuse | Same function works at runtime | Same: `const fn` can be called at runtime when args aren't compile-time-known |
