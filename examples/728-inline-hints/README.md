📖 **[View on hightechmind.io →](https://hightechmind.io/rust/728-inline-hints)**

---

# 728-inline-hints — Inline Hints

## Problem Statement

Modern CPUs execute code most efficiently when hot paths are inlined and cold paths stay out of the instruction cache. Without compiler hints the optimizer must guess which call sites to inline and which branches are rarely taken. Rust exposes `#[inline]`, `#[inline(always)]`, `#[inline(never)]`, and `#[cold]` as explicit hints to LLVM, letting the programmer communicate profiling knowledge without changing program semantics. This matters in systems code — kernel drivers, game engines, parsers — where a mis-inlined function on a hot loop can cost 10–30% throughput.

## Learning Outcomes

- Distinguish `#[inline]`, `#[inline(always)]`, and `#[inline(never)]` and know when each applies
- Understand how `#[cold]` influences branch prediction and cache layout
- Use `std::hint::black_box` to prevent dead-code elimination in benchmarks
- Recognize that `#[inline]` is required for cross-crate inlining in Rust
- Know when NOT to inline (large functions that bloat callers)

## Rust Application

`add` carries `#[inline]` (cross-crate suggestion), `fast_abs` uses `#[inline(always)]` to enable constant folding when arguments are known at compile time, and `heavy_computation` uses `#[inline(never)]` to keep a stable profiler symbol. The `#[cold]` attribute on `handle_error` and `allocation_failed` tells LLVM to deprioritize those paths in register allocation and branch layout so the hot success path gets priority.

## OCaml Approach

OCaml's native compiler inlines aggressively based on function size heuristics. The programmer can use `[@inline always]` (Jane Street Core/Flambda) or `[@unrolled n]` for loops. In standard OCaml there is no `[@cold]` equivalent; cold-path separation is purely compiler-driven. OCaml inlines across modules naturally during native compilation without requiring special annotations.

## Key Differences

1. **Explicit vs implicit**: Rust exposes inlining as user-facing attributes; standard OCaml hides it behind heuristics unless Flambda is active.
2. **Cross-crate boundary**: Rust requires `#[inline]` for functions inlined across crate (library) boundaries; OCaml inlines across `.cmo`/`.cmx` modules automatically.
3. **Cold annotations**: Rust's `#[cold]` directly sets LLVM branch weights; OCaml's equivalent exists only under Flambda profiles.
4. **Target features**: Rust's `#[target_feature]` for SIMD dispatch has no standard OCaml parallel — OCaml uses C stubs or vendored SIMD via external libraries.

## Exercises

1. Add `#[inline(always)]` to `heavy_computation` and use `cargo asm` to observe how the binary layout changes compared to `#[inline(never)]`.
2. Write a `parse_f64` function with a fast decimal path and a `#[cold]` scientific-notation path. Benchmark both branches with `std::hint::black_box`.
3. Implement a SIMD dot product using `#[target_feature(enable = "avx2")]` with a scalar fallback, dispatched at runtime via `is_x86_feature_detected!`.
