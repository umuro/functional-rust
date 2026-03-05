# OCaml vs Rust: Const Functions (Compile-Time Computation)

## Side-by-Side Code

### OCaml
```ocaml
(* Module-level bindings are evaluated at program startup, not compile time *)
let rec fibonacci n =
  match n with
  | 0 -> 0
  | 1 -> 1
  | n -> fibonacci (n - 1) + fibonacci (n - 2)

(* Computed when the module loads — runtime initialization *)
let fib_10 = fibonacci 10
let fib_20 = fibonacci 20

(* Binary exponentiation — pure, but still runtime *)
let pow_int base exp =
  let rec go acc b e =
    if e = 0 then acc
    else if e mod 2 = 1 then go (acc * b) (b * b) (e / 2)
    else go acc (b * b) (e / 2)
  in
  go 1 base exp

(* Lookup table built at startup *)
let square_table = Array.init 256 (fun i -> i * i)

let () =
  assert (fib_10 = 55);
  assert (fib_20 = 6765);
  assert (pow_int 2 16 = 65536);
  assert (square_table.(15) = 225);
  print_endline "ok"
```

### Rust (idiomatic — const fn)
```rust
pub const fn fibonacci(n: u64) -> u64 {
    let mut a = 0u64;
    let mut b = 1u64;
    let mut i = 0u64;
    while i < n {
        let temp = b;
        b = a + b;
        a = temp;
        i += 1;
    }
    a
}

// Evaluated at compile time — value baked into the binary
pub const FIB_10: u64 = fibonacci(10);
pub const FIB_20: u64 = fibonacci(20);
```

### Rust (functional/recursive — NOT const-compatible)
```rust
// Recursive fib works at runtime but cannot be used in const context
// because const fn recursion depth isn't statically bounded.
pub fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}
```

### Rust (compile-time lookup table)
```rust
pub const fn build_square_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        table[i] = (i * i) as u32;
        i += 1;
    }
    table
}

// Entire 256-entry table lives in read-only binary data — zero runtime init cost
pub const SQUARE_TABLE: [u32; 256] = build_square_table();
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Fibonacci function | `val fibonacci : int -> int` | `const fn fibonacci(n: u64) -> u64` |
| Compile-time constant | *(not possible without PPX)* | `const FIB_10: u64 = fibonacci(10)` |
| Mutable accumulator | `let rec go acc b e = ...` (immutable params) | `let mut a = 0u64; while ...` (loop required) |
| Lookup array | `Array.t` (heap-allocated) | `[u32; 256]` (stack / static memory) |
| Alignment helper | `let align_up size align = ...` | `pub const fn align_up(size: usize, align: usize) -> usize` |

## Key Insights

1. **True compile-time vs startup-time:** OCaml module-level `let` bindings are evaluated when the module loads at program startup — they are fast but still runtime. Rust `const` items evaluated in a `const fn` context are fully resolved by the compiler; the result is a literal in the binary with zero runtime cost.

2. **No recursion in const fn (in practice):** Rust's const evaluator supports recursion technically, but the compiler enforces a recursion limit. Idiomatic `const fn` uses `while` loops instead, which maps to OCaml's tail-recursive helper pattern — both avoid stack blowup, but for different reasons.

3. **Static arrays vs heap arrays:** OCaml's `Array.init` produces a heap-allocated array initialized at runtime. Rust's `const fn` can return `[T; N]` fixed-size arrays that live in the binary's read-only data segment — no heap, no initialization, ideal for embedded targets with no allocator.

4. **Dual-mode functions:** A Rust `const fn` works in both compile-time and runtime contexts with no code duplication. OCaml has no direct equivalent; you'd need a PPX preprocessor or a separate build script to achieve true compile-time constants from computed values.

5. **Embedded and `no_std` relevance:** `const fn` evaluation happens entirely inside the compiler, with no OS or allocator. This makes it the primary tool for `#![no_std]` embedded Rust — protocol buffer sizes, register masks, and CRC tables are all computable this way, replacing what C developers do with complex preprocessor macros or build-time code generators.

## When to Use Each Style

**Use `const fn` with loops when:** you need a value or table embedded in the binary with zero runtime overhead — Fibonacci indices, CRC polynomials, alignment masks, or any mathematically fixed constant derived from other constants.

**Use runtime functions when:** the computation depends on runtime inputs, involves heap allocation, or uses types/operations not yet supported in `const` contexts (e.g., floating-point math, trait object dispatch).
