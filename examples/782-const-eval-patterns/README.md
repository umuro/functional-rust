📖 **[View on hightechmind.io →](https://hightechmind.io/rust/782-const-eval-patterns)**

---

# 782-const-eval-patterns — Const Eval Patterns

## Problem Statement

Compile-time evaluation is not limited to simple arithmetic. Complex computations — string hashing for perfect hash maps, computing bit-reversal permutations, generating code tables — can run during compilation. This example collects practical `const fn` patterns: FNV hash of a string, log2 of a size for bit-counting, compile-time min/max/clamp, and ANSI escape code generation for terminal colors. These patterns eliminate entire categories of runtime initialization.

## Learning Outcomes

- Implement `const fn const_hash(s: &str) -> u64` using the FNV-1a algorithm
- Write `const fn const_log2(n: usize) -> usize` for bit-shift computations
- Use `const fn const_min/max/clamp` for bounded configuration values
- Understand which operations are available in `const fn` (bitops, arithmetic, loops, if)
- See how `const fn` can generate switch tables: `const ESCAPE_CODES: [&str; 8]`

## Rust Application

`const_hash` implements FNV-1a: XOR each byte then multiply by the FNV prime. Used for compile-time `match`-like dispatch tables. `const_max` and `const_min` use `if` expressions. `const_clamp` combines them. `const_log2` uses a `while` loop to count right-shift steps. All are declared `pub const fn` and used to initialize `const` values at module level. Tests verify against expected values.

## OCaml Approach

OCaml lacks compile-time evaluation for complex patterns. The `cppo` preprocessor handles conditional compilation. For hash maps, `phf` (Rust) has no OCaml equivalent — OCaml uses runtime `Hashtbl`. Code generation via Dune rules or `ocamlopt` plugins can produce pre-computed tables, but this is more complex than Rust's `const fn`. Jane Street's `ppx_hash` generates efficient hash functions but evaluates them at runtime.

## Key Differences

1. **FNV at compile time**: Rust computes FNV hashes at compile time for switch dispatch; OCaml computes at runtime (module initialization), which is still fast but not embedded in the binary.
2. **Log2 for arrays**: Rust uses `const_log2(N)` to compute shift amounts for power-of-two rings at compile time; OCaml computes this at runtime.
3. **String patterns**: Rust's `const fn const_hash(s: &str)` enables compile-time string→u64 mapping; OCaml has no equivalent.
4. **Restrictions**: `const fn` cannot call `println!`, allocate, or use `dyn`; OCaml module-level code can call any function.

## Exercises

1. Use `const_hash` to implement a compile-time dispatch table: `const HANDLERS: [(u64, &str); 4] = [...]` mapping hashed command names to handler names.
2. Implement `const fn is_ascii_uppercase(c: u8) -> bool` and `const fn to_lowercase(c: u8) -> u8` and use them in a `const fn to_lowercase_str` that operates on a fixed-size array.
3. Write `const fn encode_color(r: u8, g: u8, b: u8) -> u32` that packs RGB into a 24-bit constant and a complementary `const fn decode_color(c: u32) -> (u8, u8, u8)`.
