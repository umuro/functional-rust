# 733: Profile-Guided Patterns

**Difficulty:** 3  **Level:** Advanced

Structure code for profiler visibility and cache friendliness — `black_box`, cold/hot annotations, and struct-of-arrays layout.

## The Problem This Solves

Writing fast code is one thing. Writing code that *profiles correctly* is another. If the compiler optimises away the hot function you're measuring, the profiler reports zero time — not because the function is fast, but because it doesn't exist in the binary. If your hot and cold paths share instruction cache space, the cold error handler evicts the hot loop's instructions on every rare error. If your data layout packs unrelated fields together, iterating over one field stalls on cache misses loading the other fields.

Profile-guided patterns are code conventions that make the profiler accurate and the hot path efficient. `black_box` prevents dead-code elimination so the profiler sees real call counts. `#[cold]` + `#[inline(never)]` on error handlers keeps them out of the hot path's instruction cache and register file. Struct-of-Arrays (SoA) layout separates fields by type — when your hot loop only touches the `x` coordinates of a million particles, SoA loads only `x` values; AoS loads the entire particle struct, wasting three-quarters of every cache line.

## The Intuition

Profiling is archaeology: you're reconstructing what the program did from traces in the execution record. The traces are accurate only if the program actually ran the code you think it ran. `black_box` keeps the code in; `#[inline(never)]` keeps the call visible as a distinct frame; `#[cold]` pushes error paths to the far end of the binary. These annotations make the binary's structure match your mental model of the program.

SoA vs AoS is about telling the CPU which data it needs together. If you always access `x`, `y`, `z` together on each particle, AoS is right. If you always process `x` for all particles before moving to `y`, SoA is right. Profilers (and hardware performance counters) will show cache-miss rates that guide the decision.

## How It Works in Rust

```rust
use std::hint::black_box;

/// black_box prevents constant-folding this computation away.
#[inline(never)]  // visible as its own profiler frame
fn sum_squares(n: u64) -> u64 {
    (0..n).map(|i| i * i).sum()
}

/// Cold: moves this out of the hot path's instruction stream.
#[cold]
#[inline(never)]
fn handle_overflow(a: u64, b: u64) -> u64 {
    eprintln!("overflow: {} + {}", a, b);
    u64::MAX
}

fn checked_add_hot(a: u64, b: u64) -> u64 {
    a.checked_add(b).unwrap_or_else(|| handle_overflow(a, b))
}

// AoS — cache-unfriendly when iterating only one field:
struct ParticleAoS { x: f32, y: f32, z: f32, mass: f32 }

// SoA — load only the fields you touch:
struct ParticlesSoA {
    x:    Vec<f32>,
    y:    Vec<f32>,
    z:    Vec<f32>,
    mass: Vec<f32>,
}

// Hot loop: iterates all x values — SoA loads only x[], no wasted cache lines.
fn sum_x(p: &ParticlesSoA) -> f32 {
    black_box(p.x.iter().sum())
}
```

Use `perf stat`, `valgrind --tool=callgrind`, or `cargo flamegraph` to measure. `#[inline(never)]` keeps functions as separate nodes in the flame graph. `black_box` ensures the work isn't elided.

## What This Unlocks

- **Accurate profiler data**: `black_box` + `#[inline(never)]` ensure the profiler sees real timings for the functions you're measuring.
- **Faster hot paths**: `#[cold]` on error handlers keeps the CPU's instruction cache and branch predictor focused on the success path.
- **Cache-efficient bulk processing**: SoA layout for data-parallel workloads (physics, audio, rendering) cuts cache misses by 4–8× when only one field is accessed per loop.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Prevent dead-code elimination | `Sys.opaque_identity` | `std::hint::black_box` |
| Inline control | `[@inline]` / `[@inline never]` | `#[inline]` / `#[inline(never)]` |
| Cold path hint | Not available | `#[cold]` |
| Likely/unlikely hints | Not available | `std::hint::likely` (nightly) |
| SoA vs AoS layout | Manual struct definitions | Manual struct definitions |
| Cache profiling | `perf`/Instruments via C FFI | `perf stat`, `cargo flamegraph` natively |
