# Branchless Programming
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


> **Functional Rust** · [hightechmind.io](https://hightechmind.io)

## Problem Statement

Modern CPUs speculatively execute both sides of a branch before knowing which path
is taken, relying on a branch predictor trained on past behavior. When the prediction
is correct, branches are nearly free. When the predictor is wrong—either because the
branch outcome is data-dependent and unpredictable, or because the branch alternates
irregularly—a misprediction flushes the pipeline and costs 10–20 cycles. For tight
loops over large, unsorted data (sorting networks, cryptography, parsers, games),
branch mispredictions can dominate execution time.

Branchless programming replaces conditional jumps with arithmetic: boolean conditions
are converted to 0/1 integers, and the desired value is selected via multiplication or
bitwise masking. The CPU computes both outcomes and selects one without speculating.
The technique is standard in constant-time cryptographic code (preventing timing side
channels), SIMD kernels, and performance-critical inner loops.

## Learning Outcomes

- Explain branch prediction, misprediction penalties, and pipeline stalls
- Convert conditional expressions to arithmetic using sign-extension masks
- Implement branchless `min`, `max`, `abs`, `clamp`, and `select` for integers
- Recognize when the compiler already generates branchless code (check with `objdump`)
- Understand constant-time programming and why branches leak secret data

## Rust Application

```rust
// Arithmetic right-shift trick: extends sign bit to all bits
// x >> 63 == 0xFFFFFFFFFFFFFFFF if x < 0, else 0x0000000000000000
fn branchless_min_i64(a: i64, b: i64) -> i64 {
    let diff = a - b;
    // mask = all 1s if diff < 0 (a < b), else all 0s
    let mask = diff >> 63;
    b + (mask & diff)   // b if a >= b, else b + (a - b) = a
}

fn branchless_max_i64(a: i64, b: i64) -> i64 {
    let diff = a - b;
    let mask = diff >> 63;
    a - (mask & diff)   // a if a >= b, else a - (a - b) = b
}

fn branchless_abs_i64(x: i64) -> i64 {
    let mask = x >> 63;
    (x + mask) ^ mask   // two's-complement negation via XOR
}

// Generic select: returns a if cond, else b (cond must be 0 or 1)
fn select_i64(cond: i64, a: i64, b: i64) -> i64 {
    // cond_mask = 0xFF..FF if cond != 0, else 0x00..00
    let cond_mask = -cond;   // -1 if cond==1, 0 if cond==0
    (a & cond_mask) | (b & !cond_mask)
}

// Constant-time byte comparison (for cryptographic equality checks)
fn ct_eq_bytes(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() { return false; }
    let mut diff: u8 = 0;
    for (&x, &y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;   // accumulate differences without branching
    }
    diff == 0
}
```

Note: Rust's optimizer often generates branchless code for `a.min(b)` and `a.max(b)`
on integer types. Manual branchless code is needed when the compiler does not, or when
constant-time guarantees are required regardless of optimization level.

## OCaml Approach

OCaml integers are tagged (63-bit on 64-bit systems), so arithmetic right-shift tricks
require care with the tag bit. The standard library uses branching `min`/`max`. For
constant-time crypto, OCaml code typically relies on external C via FFI:

```ocaml
(* Tagged int: arithmetic shift includes tag bit — avoid for branchless tricks *)
let branchless_min_unsafe a b =
  (* Only safe if a and b are untagged ints (Bigarray, Bytes) *)
  let diff = a - b in
  let mask = diff asr 62 in   (* asr 62 to avoid tag bit *)
  b + (mask land diff)

(* Safe OCaml — uses compare, may branch *)
let safe_min a b = if a < b then a else b
```

For constant-time code, OCaml projects use `Hacl-star` bindings or C stubs.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Integer representation | Untagged `i64`/`i32` | Tagged 63-bit int |
| Arithmetic right shift | `>> 63` fills all bits | `asr 62` needed (tag bit) |
| Compiler branchless output | `a.min(b)` often branchless | Standard `min` may branch |
| Constant-time stdlib | `subtle` crate | External C (`Hacl-star`) |
| SIMD branchless blend | `_mm_blendv_epi8` via `std::arch` | Requires C FFI |

## Exercises

1. Verify that `i64::min(a, b)` compiles to a `cmov` (conditional move) instruction
   by inspecting `cargo rustc --release -- --emit asm` output. Does branchless change
   with `-C target-cpu=native`?
2. Implement a branchless `clamp(x: i64, lo: i64, hi: i64) -> i64` using two
   applications of `branchless_min`/`branchless_max`.
3. Benchmark sorting an array of 10,000 random `i32` values using `sort_unstable` vs
   a hand-written sorting network for `N=8` elements. Measure branch mispredictions
   with `perf stat -e branch-misses`.
4. Implement `ct_select` from the `subtle` crate interface: a `Choice` type wrapping
   a secret boolean and a `ConditionallySelectable` trait for `u64`.
5. Apply the arithmetic right-shift min to a scalar loop over `Vec<i32>` and compare
   the generated assembly with the LLVM auto-vectorized version.
