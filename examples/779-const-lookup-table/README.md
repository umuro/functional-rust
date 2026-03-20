📖 **[View on hightechmind.io →](https://hightechmind.io/rust/779-const-lookup-table)**

---

# 779-const-lookup-table — Const Lookup Table

## Problem Statement

Lookup tables trade memory for computation: instead of calculating `sin(x)` at runtime, you pre-compute 256 values and index into the table with a single array access. Pre-computing these tables at compile time with `const fn` means zero startup overhead — the table is embedded in the binary's read-only data segment. This technique is used in audio DSP (wavetable synthesis), graphics (gamma correction curves), and cryptography (S-boxes in AES, CRC32 tables).

## Learning Outcomes

- Generate a 256-entry sine lookup table at compile time using `const fn`
- Generate a 256-entry CRC32 lookup table using the standard polynomial `0xEDB88320`
- Use Taylor series approximations for `sin` in `const fn` (standard `f64::sin` is not `const`)
- Access the tables at runtime with O(1) array indexing
- Understand how to validate compile-time generated tables against runtime computations

## Rust Application

`generate_sin_table()` is a `const fn` that fills `[i16; 256]` using a Taylor series approximation of `sin`. `generate_crc32_table()` fills `[u32; 256]` using the CRC32 polynomial via a shift-register loop. Both are stored as `const` arrays: `const SIN_TABLE: [i16; 256] = generate_sin_table()`. `crc32` uses the table for O(n) CRC computation. Tests verify selected table entries against expected values.

## OCaml Approach

OCaml generates lookup tables at module initialization using `Array.init` with a computation function. For CRC32, `let crc32_table = Array.init 256 compute_entry`. Since OCaml doesn't have compile-time evaluation, these initialize on first module load (fast but not truly compile-time). For embedded targets, OCaml uses code generation scripts to produce hardcoded array literals.

## Key Differences

1. **Compile vs runtime**: Rust embeds the table in the binary at compile time (zero startup cost); OCaml computes the table during module initialization (milliseconds on first use).
2. **Float restrictions**: `const fn` in Rust cannot use `f64::sin` (not stable const); Taylor series approximation is required; OCaml uses `Float.sin` at runtime initialization freely.
3. **Binary size**: Rust's compile-time tables are in the `.rodata` segment, counted in binary size; OCaml's are heap-allocated at runtime.
4. **Validation**: Both can test generated tables against known values; Rust does this via `const_assert!` or tests; OCaml uses `Alcotest`.

## Exercises

1. Add a `generate_cos_table() -> [i16; 256]` and implement `fast_cos(angle_256: u8) -> f32` using the sine table with a 64-entry phase offset.
2. Implement `crc32_bytes(data: &[u8]) -> u32` that uses the generated table for CRC32/ISO-HDLC as used in Ethernet, ZIP, and PNG.
3. Generate a gamma correction lookup table for 8-bit image processing: `const GAMMA_TABLE: [u8; 256]` where `table[i] = (255 * (i/255)^2.2) as u8`.
