# OCaml vs Rust: Branchless Programming

## Side-by-Side Code

### OCaml
```ocaml
(* Branchless min: arithmetic right-shift produces 0 or all-ones mask *)
let min_branchless (a : int) (b : int) =
  let diff = a - b in
  b + (diff land (diff asr 62))

(* Branchless abs using sign-mask technique *)
let abs_branchless (x : int) =
  let mask = x asr 62 in
  (x + mask) lxor mask

(* Clamp via composition — no branches *)
let clamp_branchless lo hi x =
  min_branchless hi (max_branchless lo x)
```

### Rust (idiomatic — LLVM emits CMOV)
```rust
#[inline(always)]
pub fn min_idiomatic(a: i64, b: i64) -> i64 { a.min(b) }

#[inline(always)]
pub fn clamp_idiomatic(lo: i64, hi: i64, x: i64) -> i64 { x.clamp(lo, hi) }

#[inline(always)]
pub fn abs_idiomatic(x: i64) -> i64 { x.wrapping_abs() }
```

### Rust (explicit branchless bitmask)
```rust
#[inline(always)]
pub fn min_branchless(a: i64, b: i64) -> i64 {
    let diff = a.wrapping_sub(b);
    let mask = diff >> 63; // 0 or -1 (all bits set)
    b + (diff & mask)
}

#[inline(always)]
pub fn abs_branchless(x: i64) -> i64 {
    let mask = x >> 63;
    (x + mask) ^ mask
}

#[inline(always)]
pub fn select_branchless(cond: bool, a: i64, b: i64) -> i64 {
    let mask = -(cond as i64); // 0 or -1
    (a & mask) | (b & !mask)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Integer type | `int` (63-bit on 64-bit platform) | `i64` (explicit 64-bit) |
| Arithmetic right-shift | `asr` operator | `>> 63` (arithmetic on signed) |
| Bitwise AND | `land` | `&` |
| Bitwise XOR | `lxor` | `^` |
| Safe subtraction | `-` (may overflow on edge cases) | `wrapping_sub` (defined overflow) |
| Abs | `abs` (stdlib, may branch) | `wrapping_abs()` |
| Clamp | custom composition | `x.clamp(lo, hi)` |

## Key Insights

1. **Arithmetic right-shift is the core primitive.** Both OCaml (`asr`) and Rust (`>> 63` on `i64`) sign-extend the MSB to produce a bitmask of 0 or -1. The difference: OCaml's `int` is 63-bit, so `asr 62` is used; Rust's `i64` is 64-bit, so `>> 63` is correct.

2. **Rust's idiomatic path is already branchless.** LLVM compiles `a.min(b)`, `x.clamp(lo, hi)`, and `x.wrapping_abs()` to CMOV (conditional move) instructions on x86-64 — no conditional jump, no pipeline flush. The explicit bitmask technique is a teaching tool and a fallback for targets where CMOV is unavailable.

3. **`wrapping_sub` vs bare subtraction.** OCaml integers are 63-bit and overflow is technically undefined in some contexts; Rust makes the overflow semantics explicit with `wrapping_sub`, which is essential for the bitmask trick to be well-defined.

4. **`select_branchless` generalises the pattern.** Casting `bool` to `i64` gives 0 or 1; negating gives 0 or -1. This is a portable, safe way to implement conditional selection without a branch — useful when the compiler fails to emit a CMOV on its own.

5. **When branchy code wins.** Branchless code always executes both computation paths. For data where one branch dominates (e.g., 99% positive inputs for abs), the CPU branch predictor wins and branchless is slower. For truly unpredictable data (random min/max of arbitrary inputs), branchless eliminates misprediction stalls and wins decisively.

## When to Use Each Style

**Use idiomatic Rust (`a.min(b)`, `x.clamp(lo, hi)`):** Always as the default — readable, safe, and LLVM produces CMOV. Profile before reaching for explicit bitmasks.

**Use explicit branchless bitmasks when:** Profiling confirms misprediction stalls, the target architecture lacks CMOV, or you need a `select` primitive that the compiler fails to optimise into a conditional move.
