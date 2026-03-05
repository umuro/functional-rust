# 776: const fn: Compile-Time Computation

**Difficulty:** 3  **Level:** Intermediate

Functions marked `const fn` are evaluated at compile time when called in a const context — the result is baked into the binary, costing zero CPU cycles at runtime.

## The Problem This Solves

Some computations are deterministic and only need to happen once. Factorial of 10 is always 3,628,800. The FNV-1a hash of `"Content-Type"` is always the same. A power-of-two lookup table has the same values in every build. Computing these at runtime wastes CPU cycles and can obscure the code's intent.

The naive solution is to precompute by hand and hard-code the constants. But hard-coded magic numbers are maintenance nightmares: when someone asks "where does 3628800 come from?", the answer is buried in comments or tribal knowledge. `const FAC10: u64 = factorial(10)` is self-documenting and verifiable — the compiler checks that `factorial(10)` equals what you expect.

The killer use case is lookup tables. A CRC-32 table is 256 u32 values computed from the CRC polynomial. Before `const fn`, you'd either compute it at program startup (wasting time and making the table mutable) or paste 256 hard-coded numbers. With `const fn`, you compute it once, the linker places it in the binary's read-only segment, and runtime lookup is a pure array index.

## The Intuition

In Python, you'd write a module-level expression: `FAC10 = math.factorial(10)` — Python evaluates it when the module loads. In Rust, `const fn` moves that evaluation to compile time. The difference: Python's evaluation happens every time the process starts; Rust's happens once per build, and the result is embedded in the binary.

Think of `const fn` as the boundary between runtime and compile-time. A function marked `const fn` can be called either way: in a `const` context it runs at compile time; in a runtime context it runs normally. This means your `factorial` function can serve both purposes without duplication.

## How It Works in Rust

```rust
// The const fn marker — this function CAN be called at compile time
const fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

const fn isqrt(n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x { x = y; y = (x + n / x) / 2; }
    x
}

// Called in const context — evaluated at compile time, zero runtime cost
const FAC10: u64 = factorial(10);    // 3_628_800 — in binary as a constant
const FAC15: u64 = factorial(15);    // 1_307_674_368_000
const SQRT_1000: u64 = isqrt(1000); // 31

// FNV-1a hash at compile time — useful for string-keyed dispatch tables
const fn fnv1a_32(bytes: &[u8]) -> u32 {
    let mut hash: u32 = 0x811c9dc5;
    let mut i = 0;
    while i < bytes.len() {         // MUST use while, not for — for isn't const yet
        hash ^= bytes[i] as u32;
        hash = hash.wrapping_mul(0x01000193);
        i += 1;
    }
    hash
}

const HELLO_HASH: u32 = fnv1a_32(b"hello");  // compile-time hash of a string literal

// Compile-time lookup table — computed once, stored in read-only binary segment
const POW2_TABLE: [u64; 16] = {
    let mut t = [0u64; 16];
    let mut i = 0u32;
    while i < 16 {
        t[i as usize] = 1u64 << i;   // 1, 2, 4, 8, 16, ...
        i += 1;
    }
    t
};

// At runtime: POW2_TABLE[n] is just an array index — O(1), no computation
fn pow2_fast(n: u32) -> u64 {
    POW2_TABLE[n as usize]
}

// const fn with const generics — clamping at compile time
const fn clamp<const LO: i32, const HI: i32>(v: i32) -> i32 {
    if v < LO { LO } else if v > HI { HI } else { v }
}
const CLAMPED: i32 = clamp::<0, 100>(150);  // = 100, computed at compile time
```

Constraints on `const fn`:
- No trait objects (`dyn Trait`)
- No heap allocation (`Box`, `Vec`, `String`)
- Loops: use `while` — `for` over iterators is not yet const-stable
- Function pointers: limited support
- Most arithmetic, conditionals, array indexing: fully supported

## What This Unlocks

- **Zero-runtime-cost lookup tables**: CRC-32, sin/cos LUTs, ASCII category tables, prime sieves — computed once by the compiler, accessed as a plain array index at runtime
- **Self-documenting constants**: `const BUFFER_SIZE: usize = next_power_of_two(MAX_CONNECTIONS)` is clearer than `const BUFFER_SIZE: usize = 1024` with a comment
- **Embedded and no-std code**: no dynamic allocation, no startup cost — all precomputed values are in the binary's `.rodata` section

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Compile-time eval | `[@@unrolled]` or manual ppx | `const fn` — native to the language |
| Computed constants | Module-level `let c = f x` (runtime) | `const C: T = f(x)` where `f` is `const fn` |
| Lookup tables | `let table = Array.init 256 compute` (startup) | `const TABLE: [T; N] = { ... }` — in binary |
| Iteration in const | N/A | `while` loops — `for` not yet const-stable |
| Restrictions | N/A | No heap, no trait objects, no closures |
| Verified at compile | No | Yes — `const` expressions are fully evaluated by the compiler |
