# 728: Inline Hints — #[inline], #[cold], #[target_feature]

**Difficulty:** 4  **Level:** Expert

Guide LLVM's optimisation decisions without changing semantics — inlining, cold-path biasing, and per-function CPU feature unlocking.

## The Problem This Solves

Rust's optimizer (LLVM) makes inlining decisions automatically: small functions get inlined, large functions don't. Most of the time this is correct. But there are cases where you know better than the heuristic: a tiny arithmetic helper that is called millions of times per second should always be inlined to enable constant folding at the call site. An error handler that is called once in ten million requests should be marked cold so it doesn't compete with hot-path code for registers.

`#[inline]`, `#[inline(always)]`, `#[inline(never)]`, and `#[cold]` are the levers. They don't change what the program does — they change how LLVM generates machine code for it. Used correctly, they can eliminate call overhead, improve branch prediction, shrink binary size, and make profiler output more readable.

`#[target_feature(enable = "avx2")]` is different: it unlocks architecture-specific instructions for a single function without requiring a global `-C target-feature` flag. This lets you ship one binary with a runtime CPU dispatch: detect AVX2 at startup, call the AVX2 path; fall back to the SSE2 path otherwise.

## The Intuition

Inlining is the compiler copy-pasting your function body into every call site. The benefit: the optimiser sees both the caller's context and the callee's logic at once, enabling constant folding, dead-code elimination, and loop fusion. The cost: binary size grows with every inlined copy. `#[inline]` is a nudge. `#[inline(always)]` is a command (but can still be overridden in some configurations).

`#[cold]` is a branch-prediction hint at the function level. It says: "calls to this function are rare." The CPU branch predictor learns to default to the not-calling branch. Register allocators prioritise the hot path. Compilers move cold code to less-preferred code regions. One `#[cold]` on your error handler can measurably improve performance on the happy path.

## How It Works in Rust

```rust
/// Always inline — enables constant folding at call sites.
#[inline(always)]
pub fn fast_abs(x: i64) -> i64 {
    if x < 0 { -x } else { x }
}

/// Never inline — keeps a stable frame for profiling.
#[inline(never)]
pub fn heavy_computation(data: &[i64]) -> i64 {
    data.iter().map(|&x| x * x).sum()
}

/// Cold — moves this out of the hot path's instruction stream.
#[cold]
#[inline(never)]
fn handle_error(msg: &str) -> ! {
    panic!("fatal: {}", msg)
}

fn checked_op(a: u64, b: u64) -> u64 {
    // Compiler knows the `else` branch is cold — biases branch prediction.
    a.checked_add(b).unwrap_or_else(|| { handle_error("overflow"); })
}

/// unsafe: calling this on a CPU without AVX2 is an illegal-instruction fault.
#[target_feature(enable = "avx2")]
unsafe fn avx2_dot(a: &[f32], b: &[f32]) -> f32 {
    // AVX2 intrinsics available here without global target feature.
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

// Runtime dispatch:
fn dot(a: &[f32], b: &[f32]) -> f32 {
    if std::is_x86_feature_detected!("avx2") {
        unsafe { avx2_dot(a, b) }
    } else {
        a.iter().zip(b).map(|(x, y)| x * y).sum()
    }
}
```

## What This Unlocks

- **Zero-overhead abstractions**: Mark trivial wrapper functions `#[inline(always)]` — the wrapper disappears entirely, as if you'd written the underlying code directly.
- **Better branch prediction on error paths**: `#[cold]` on error/validation functions biases the CPU toward the success path — measurable improvement in tight loops.
- **Portable fat binaries**: Ship one binary, dispatch at runtime to AVX2/SSE4/scalar paths with `#[target_feature]` — best performance on every CPU without recompiling.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Force inlining | `[@inline]` / flambda optimiser | `#[inline(always)]` |
| Prevent inlining | `[@inline never]` | `#[inline(never)]` |
| Cold path hint | Not available | `#[cold]` |
| Per-function CPU features | Not available | `#[target_feature(enable = "...")]` |
| Runtime CPU detection | Not available | `is_x86_feature_detected!("avx2")` |
| Link-time optimisation | `flambda` at build time | `lto = true` in `Cargo.toml` |
