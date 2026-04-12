# Portable SIMD Concepts
**Difficulty:** ⭐  
**Category:** Functional Programming  



> **Functional Rust** · [hightechmind.io](https://hightechmind.io)

## Problem Statement

Single Instruction, Multiple Data (SIMD) instructions process N data elements with one
CPU operation. SSE4.2 processes 4 floats simultaneously; AVX2 processes 8; AVX-512
processes 16. A scalar loop that multiplies 1 million floats takes ~1M multiply
instructions; an AVX2 loop takes ~125K—an 8× speedup from instruction count alone.
SIMD is essential in signal processing (FFT, FIR filters), image processing
(convolution, color conversion), machine learning (matrix multiply, softmax), physics
(N-body, fluid simulation), and cryptography (AES-NI, polynomial hashing).

The challenge: x86 SSE/AVX, ARM NEON, and RISC-V V extension have different intrinsics,
different vector widths, and different semantics. Code written against `_mm256_add_ps`
is not portable. Rust's `std::simd` (portable SIMD, stabilizing) and the `wide` crate
provide lane-generic vector types that compile to optimal intrinsics per target.

## Learning Outcomes

- Understand SIMD lanes, vector width, and the lane-parallel execution model
- Implement scalar algorithms in a lane-parallel style that mirrors SIMD semantics
- Use `std::simd::f32x4` / `f32x8` portable vector types (nightly)
- Apply `chunks_exact` to process arrays in SIMD-width batches with scalar remainder
- Recognize SIMD alignment requirements and when to use `#[repr(align(32))]`

## Rust Application

```rust
// Scalar version — conceptually one "lane"
fn dot_product_scalar(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum()
}

// Lane-parallel scalar: processes LANES elements conceptually in parallel
// This mirrors how SIMD code is written, enabling drop-in replacement
const LANES: usize = 4;

fn dot_product_lanes(a: &[f32], b: &[f32]) -> f32 {
    let mut sums = [0.0f32; LANES];

    // Main loop: chunks of LANES
    let chunks_a = a.chunks_exact(LANES);
    let chunks_b = b.chunks_exact(LANES);
    let remainder_a = chunks_a.remainder();
    let remainder_b = chunks_b.remainder();

    for (ca, cb) in chunks_a.zip(chunks_b) {
        for i in 0..LANES {
            sums[i] += ca[i] * cb[i];   // SIMD: one fma instruction per lane
        }
    }

    // Horizontal sum of accumulator lanes
    let mut total: f32 = sums.iter().sum();

    // Scalar remainder (len % LANES elements)
    for (&x, &y) in remainder_a.iter().zip(remainder_b.iter()) {
        total += x * y;
    }
    total
}

// With portable SIMD (nightly):
// #![feature(portable_simd)]
// use std::simd::f32x4;
// fn dot_simd(a: &[f32], b: &[f32]) -> f32 {
//     let mut acc = f32x4::splat(0.0);
//     for (ca, cb) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
//         let va = f32x4::from_slice(ca);
//         let vb = f32x4::from_slice(cb);
//         acc += va * vb;
//     }
//     acc.reduce_sum() + /* scalar remainder */
// }

// Element-wise operations on aligned data
fn scale_add(data: &mut [f32], scale: f32, offset: f32) {
    let chunks = data.chunks_exact_mut(LANES);
    for chunk in chunks {
        for x in chunk.iter_mut() {
            *x = *x * scale + offset;   // fma per lane
        }
    }
    // remainder handled by the for loop continuing into data tail
}
```

The lane-parallel scalar pattern is an exact semantic model of SIMD code. The compiler
often auto-vectorizes it; when it does not, replacing the inner array with `f32x4` is
a one-line change.

## OCaml Approach

OCaml has no portable SIMD abstraction. SIMD in OCaml requires C stubs or the
`owl-base` scientific computing library, which calls BLAS/LAPACK (SIMD-optimized C):

```ocaml
(* Pure OCaml: scalar, no SIMD *)
let dot_product a b =
  Array.fold_left2 (fun acc x y -> acc +. x *. y) 0.0 a b

(* With Owl: delegates to SIMD-optimized CBLAS *)
(* let result = Owl.Dense.Ndarray.D.dot a b *)
```

OCaml 5 has an experimental `Simd` module for internal compiler use but no stable
portable SIMD API is available to users.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Portable SIMD API | `std::simd` (nightly), `wide` crate | None (requires C FFI) |
| Auto-vectorization | LLVM vectorizes `chunks_exact` loops | Limited; scalar loops stay scalar |
| Alignment control | `#[repr(align(32))]`, `aligned` crate | `Bigarray` with alignment hints |
| SIMD intrinsics | `std::arch::x86_64::_mm256_*` | C stubs only |
| Lane width | Const generic `LANES`, `f32x4` etc. | Not applicable |

## Exercises

1. Enable nightly and rewrite `dot_product_lanes` using `std::simd::f32x4`. Compare
   the generated assembly with and without `target-cpu=native` using `cargo asm`.
2. Implement a SIMD-width-agnostic `map_inplace<const L: usize>(data: &mut [f32], f: impl Fn(f32) -> f32)` 
   that processes chunks of `L` with a scalar inner loop and benchmark `L=4` vs `L=8`.
3. Implement a SIMD-friendly prefix sum (scan) over `&[f32]` using the lane-parallel
   model. Verify correctness and benchmark vs `iter().scan()`.
4. Use the `wide` crate's `f32x8` to implement an element-wise sigmoid approximation
   `1.0 / (1.0 + (-x).exp())` and benchmark vs scalar for 1M elements.
5. Implement an aligned buffer type `AlignedBuf<T, const ALIGN: usize>` using
   `#[repr(align)]` and verify with `std::mem::align_of_val` that SIMD loads are safe.
