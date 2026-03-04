# 723: Branchless Programming

**Difficulty:** 3  **Level:** Expert

Eliminate branch mispredictions by computing results arithmetically instead of conditionally.

## The Problem This Solves

Modern CPUs execute instructions speculatively — they guess the branch outcome and start executing the predicted path before the condition is resolved. When the guess is wrong (a branch misprediction), the CPU must flush the pipeline and restart, costing 10–20 clock cycles per mispredict. For a tight loop processing unpredictably-distributed data (random min/max, clamp with arbitrary inputs), mispredictions dominate runtime.

Branchless code replaces conditional jumps with arithmetic or bitwise operations that always produce the correct result in constant time. There is no branch to mispredict. The trade-off: the CPU must execute both computation paths and combine them, versus speculatively executing one. For data where the branch is truly unpredictable (50/50 distribution), branchless wins. For data where one branch dominates (99% of values are positive), the branch predictor wins and branchless loses.

## The Intuition

A conditional `if a < b { a } else { b }` generates a conditional jump (`jl`, `jg`) in machine code. A branchless version computes a bitmask from the comparison and combines both values arithmetically — no jump, no speculation, no pipeline flush.

The key insight: arithmetic right-shift on signed integers sign-extends the most significant bit to all bits. `(a - b) >> 63` produces `0` if `a >= b` and `-1` (all-bits-set / `0xFFFF...`) if `a < b`. AND-ing with a value either selects it (mask = -1) or zeroes it (mask = 0). This is how branchless min/max works.

The more important insight: **LLVM already does this for you**. `a.min(b)` on integers compiles to a `cmov` (conditional move) instruction on x86-64 — no branch, no misprediction risk. Write idiomatic Rust first, measure, and only reach for explicit branchless tricks if profiling reveals a hot branch that LLVM isn't optimising.

## How It Works in Rust

```rust
// ── Explicit branchless (pedagogical) ────────────────────────────────────
#[inline(always)]
pub fn min_branchless(a: i64, b: i64) -> i64 {
    let diff = a.wrapping_sub(b);
    let mask = diff >> 63; // 0 or all-bits-set (0xFFFF..FFFF)
    b + (diff & mask)      // b + (a - b) if a < b, else b + 0
}

// ── LLVM-idiomatic (prefer this — compiles to CMOV) ──────────────────────
#[inline(always)]
pub fn min_idiomatic(a: i64, b: i64) -> i64 { a.min(b) }

// ── Branchless absolute value ─────────────────────────────────────────────
#[inline(always)]
pub fn abs_branchless(x: i64) -> i64 {
    let mask = x >> 63;   // 0 if positive, -1 if negative
    (x + mask) ^ mask     // two's complement negate when negative
}

// ── Branchless select: choose a or b based on a boolean ──────────────────
/// `cond` must be exactly 0 or 1 (not a general bool cast).
#[inline(always)]
pub fn select(cond: u64, a: i64, b: i64) -> i64 {
    let mask = (cond as i64).wrapping_neg(); // 0 → 0, 1 → -1
    (a & mask) | (b & !mask)
}

// ── When to profile before optimising ────────────────────────────────────
// Branchy: fast if data is predictable (99% positive values)
fn sum_positive_branchy(data: &[i64]) -> i64 {
    data.iter().filter(|&&x| x > 0).sum()
}
// Branchless: fast if data is unpredictable (random signs)
fn sum_positive_branchless(data: &[i64]) -> i64 {
    data.iter().map(|&x| x & -(x > 0) as i64).sum()
}
```

## What This Unlocks

- **Sorting networks and search algorithms** — binary search, sorting networks, and comparison-based algorithms used in databases and in-memory indexes benefit from `cmov`-based comparisons.
- **Game physics and graphics** — frustum culling, collision detection, and shader-style vector math process millions of unpredictable comparisons per frame.
- **Cryptographic implementations** — constant-time code (no secret-dependent branches) is a security requirement; branchless arithmetic prevents timing side-channels.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Integer min/max | `min a b` (may branch) | `a.min(b)` → `CMOV` (branchless in release) |
| Explicit bit mask | `Int64.logand / asr` | `wrapping_sub`, `>>`, `&` on primitives |
| Absolute value | `abs x` | `x.abs()` (branchless in LLVM) |
| Constant-time select | Manual bitmask | Manual bitmask (no stdlib primitive) |
| Compiler optimisation | Reasonably good | LLVM produces CMOV for simple if/else on integers |
