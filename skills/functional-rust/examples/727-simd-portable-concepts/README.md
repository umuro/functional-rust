# 727: SIMD Portable Concepts with std::simd

**Difficulty:** 5  **Level:** Master

Write vectorised code once with `std::simd` — compile to AVX2, NEON, or SSE2 without per-platform `#[cfg]` guards.

## The Problem This Solves

SIMD (Single Instruction, Multiple Data) lets a single CPU instruction operate on a vector of values simultaneously: add 8 floats in one instruction instead of 8. The performance gains on data-parallel workloads — audio processing, image transforms, dot products, string search — are typically 4–16× over scalar code. The problem has always been portability: AVX2 on x86, NEON on ARM, SVE on ARM64, RISC-V V extension. Writing hand-optimised SIMD meant `#[cfg(target_arch = "x86_64")]` everywhere, duplicated implementations, and fragile maintenance.

`std::simd` (the `portable_simd` nightly feature) solves this with architecture-independent vector types: `f32x8` (8 float32 lanes), `i32x4` (4 int32 lanes), `u8x16` (16 byte lanes). You write lane-wise arithmetic on these types — `a + b` on `f32x8` adds all 8 lanes in parallel. The compiler lowers these to the best native instruction set available: AVX2 on a Skylake server, NEON on an Apple Silicon Mac, SSE2 on an older x86.

The mental model shift is from "operate on element N, then N+1, then N+2" to "operate on a chunk of N elements simultaneously." This is the key insight: SIMD code is inherently data-parallel. If your algorithm has loop-carried dependencies, SIMD won't help. If it's element-wise or reducible, SIMD is a multiplier.

## The Intuition

Imagine a bank teller processing one transaction at a time versus a teller window that processes 8 transactions simultaneously because they're all the same type. That's SIMD. The `f32x8` type is a register that holds 8 floats at once. `a + b` where both are `f32x8` adds all 8 pairs in a single CPU instruction. Reductions like `.reduce_sum()` combine all 8 lanes back to a scalar.

The scalar fallback in this example mirrors the SIMD API exactly — `[f32; 8]` with manual loops. This makes the structure of the algorithm clear even without nightly Rust, and lets you verify correctness before enabling vectorisation.

## How It Works in Rust

```rust
// On nightly, enable with: #![feature(portable_simd)]
// use std::simd::{f32x8, SimdFloat};

// Stable scalar simulation — structurally identical to SIMD version.
#[derive(Clone, Copy)]
pub struct F32x8([f32; 8]);

impl F32x8 {
    pub fn splat(v: f32) -> Self { Self([v; 8]) }
    pub fn from_array(a: [f32; 8]) -> Self { Self(a) }

    // Lane-wise add — compiles to VADDPS ymm on AVX2
    pub fn add(self, rhs: Self) -> Self {
        let mut r = [0.0f32; 8];
        for i in 0..8 { r[i] = self.0[i] + rhs.0[i]; }
        Self(r)
    }

    pub fn reduce_sum(self) -> f32 { self.0.iter().sum() }
}

// Dot product over slices — processes 8 elements per iteration.
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    let mut acc = F32x8::splat(0.0);
    let chunks = a.len() / 8;
    for i in 0..chunks {
        let va = F32x8::from_array(a[i*8..i*8+8].try_into().unwrap());
        let vb = F32x8::from_array(b[i*8..i*8+8].try_into().unwrap());
        acc = acc.add(va.mul(vb));
    }
    acc.reduce_sum()
}
```

On nightly, replace `F32x8` with `std::simd::f32x8` and the loops with SIMD operator overloads — the structure stays identical.

## What This Unlocks

- **4–16× throughput on data-parallel workloads**: Dot products, image convolutions, audio sample processing, bulk string transforms — all benefit directly.
- **Write once, run everywhere**: One source, optimal code on x86 (SSE2/AVX/AVX512), ARM (NEON/SVE), and RISC-V V — no per-architecture `#[cfg]` needed.
- **Automatic vectorisation baseline**: Even without explicit SIMD, structuring loops around the 4/8/16 lane boundaries often lets the compiler auto-vectorise — explicit SIMD gives you control when it doesn't.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| SIMD access | Via C stubs or `owl-base` | `std::simd` (nightly) or `wide` crate |
| Vector types | Not in stdlib | `f32x4`, `f32x8`, `i32x16`, etc. |
| Lane operations | Not available | `a + b` on SIMD types (element-wise) |
| Horizontal reduction | Not available | `.reduce_sum()`, `.reduce_max()` |
| Conditional selection | Not available | `Mask<i32, 4>`, `.select()` |
| Platform portability | N/A | One type, best native instructions |
