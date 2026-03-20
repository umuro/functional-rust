📖 **[View on hightechmind.io →](https://hightechmind.io/rust/733-profile-guided-patterns)**

---

# 733-profile-guided-patterns — Profile-Guided Patterns

## Problem Statement

Writing fast code without profiling data is guesswork. Profile-guided optimization (PGO) uses runtime measurements to inform the compiler about which branches are hot, which functions should be inlined, and how data should be laid out in memory. Even without a full PGO pipeline, developers can apply the same principles manually: annotate hot/cold branches, choose Struct-of-Arrays over Array-of-Structs for SIMD-friendly access, and use `black_box` to ensure benchmarks measure real work.

## Learning Outcomes

- Apply `#[cold]` and `#[inline(never)]` to error handlers to keep them out of the hot path
- Understand the Struct-of-Arrays (SoA) pattern and why it improves cache utilization vs. Array-of-Structs (AoS)
- Use `std::hint::black_box` correctly to prevent the compiler from optimizing away benchmark subjects
- Recognize how branch likelihood affects branch predictor accuracy
- Know when to use `likely`/`unlikely` intrinsics (nightly) vs. structural code changes

## Rust Application

`sum_squares` is marked `#[inline(never)]` so it appears in profiler output with a stable symbol. `handle_overflow` is `#[cold]` — the compiler biases the branch prediction in `checked_add_hot` so the success arm gets the fast path. `PointsSoA` separates x, y, z fields into three contiguous `Vec<f32>` arrays; `sum_x` touches only the x array, fitting entirely in L1 cache. The AoS equivalent `aos_sum_x` loads all three floats per iteration even when only x is needed.

## OCaml Approach

OCaml does not expose hot/cold annotations in the standard language. Flambda2 (Jane Street's OCaml fork) provides `[@likely]` and `[@unlikely]` hints. The SoA pattern is expressible in OCaml using parallel arrays (`Array.t * Array.t * Array.t`) but is less ergonomic than the idiomatic record array. OCaml's `Bigarray` provides C-contiguous flat memory for SIMD-compatible layouts used in scientific computing (Owl, Lacaml).

## Key Differences

1. **Explicit branch hints**: Rust's `#[cold]` is stable; OCaml's equivalents require Flambda2 or Jane Street extensions.
2. **SoA ergonomics**: Rust's struct fields make SoA feel natural with named field vectors; OCaml uses tuples or separate modules for the pattern.
3. **Profiler visibility**: Rust's `#[inline(never)]` guarantees a symbol in `perf`/`Instruments`; OCaml's compiler may inline across modules unpredictably.
4. **SIMD layout**: Rust's `Vec<f32>` fields in SoA are directly SIMD-accessible via `std::simd` or `packed_simd`; OCaml requires `Bigarray` for equivalent alignment guarantees.

## Exercises

1. Refactor `PointsSoA` to store `x: Box<[f32]>` instead of `Vec<f32>` to eliminate the capacity word and make the layout more cache-friendly.
2. Benchmark `sum_x` on AoS vs. SoA for 1M points. Measure L1 cache misses using `perf stat -e cache-misses`.
3. Add a `sum_xyz` function to `PointsSoA` that computes `x[i] + y[i] + z[i]` for all i using iterator zip — compare its performance to the AoS equivalent.
