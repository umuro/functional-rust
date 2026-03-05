📖 **[View on hightechmind.io →](https://hightechmind.io/rust/782-const-eval-patterns)**

---

# 782: const eval: Limitations and Workarounds

**Difficulty:** 4  **Level:** Advanced

Compute values at compile time with `const fn` — and know exactly which operations are allowed, which are forbidden, and how to work around the restrictions.

## The Problem This Solves

Moving computation from runtime to compile time has real benefits: zero runtime cost, no initialization order issues, and values that are baked directly into the binary. But `const fn` in Rust is not arbitrary computation — it is a restricted subset of Rust that the compiler can evaluate deterministically at compile time.

Knowing the boundaries saves you from cryptic "this expression is not allowed in a constant" errors. More importantly, knowing the workarounds lets you achieve what you want within the rules: use fixed-size arrays instead of `Vec`, work with byte arrays instead of `String`, use integer approximations instead of floating-point where needed.

This also covers `const {}` blocks (Rust 1.79+) — inline const evaluation in function bodies.

## The Intuition

The const evaluator is a Rust interpreter that runs at compile time. It's conservative: it only allows operations it can fully evaluate with no side effects and no indirection. Heap allocation (`Vec::new()`, `String::new()`) is forbidden because the heap doesn't exist at compile time. Dynamic dispatch is forbidden. Floating-point arithmetic was forbidden until Rust 1.82, when basic float ops became stable in `const fn`.

What *is* allowed: integer arithmetic, bitwise ops, loops (`while` and `for` on ranges), conditionals, fixed-size arrays, `&str` literals, pattern matching, and calling other `const fn`s.

## How It Works in Rust

**Allowed — integer algorithms with loops:**
```rust
const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

const GCD_48_18: u64 = gcd(48, 18);  // evaluated at compile time: 6
```

**Allowed — fixed-size array generation:**
```rust
const fn squares<const N: usize>() -> [u64; N] {
    let mut out = [0u64; N];
    let mut i = 0;
    while i < N { out[i] = (i * i) as u64; i += 1; }
    out
}

const SQUARES: [u64; 10] = squares::<10>();  // [0, 1, 4, 9, 16, 25, ...]
```
Use `while` not `for i in 0..N` — range iteration isn't yet stable in `const fn`.

**Allowed — byte-level string operations:**
```rust
const fn starts_with_prefix(s: &[u8], prefix: &[u8]) -> bool {
    if s.len() < prefix.len() { return false; }
    let mut i = 0;
    while i < prefix.len() {
        if s[i] != prefix[i] { return false; }
        i += 1;
    }
    true
}
```
`&str` operations aren't fully available in `const`; work with `&[u8]` instead.

**Forbidden — workarounds:**
```rust
// ✗ const fn build_vec() -> Vec<i32> { ... }   // Vec is heap — not allowed
// ✓ Use [T; N] instead:
const fn squares<const N: usize>() -> [u64; N] { ... }

// ✗ const fn greet() -> String { ... }          // String is heap — not allowed
// ✓ Use &'static str:
const GREETING: &str = "hello";
```

**Allowed since Rust 1.79 — inline `const {}` block:**
```rust
fn demonstrate_const_block() {
    let n = const { gcd(48, 18) };  // evaluated at compile time, result inlined
    println!("n = {n}");
}
```
The `const { ... }` expression evaluates at compile time and the result is used as a literal at that point in the function.

**Dual use — same `const fn` at compile time and runtime:**
```rust
let runtime_gcd = gcd(1071, 462);  // same function, runtime call
```
`const fn` can be called at runtime too — it's a regular function that *also* happens to be evaluable at compile time.

## What This Unlocks

- **Zero-cost lookup tables** — compute `[u64; 256]` CRC tables, sine approximations, or prefix sums entirely at compile time.
- **Compile-time validation** — use `const fn` assertions (`panic!` in const is allowed) to fail compilation if invariants are violated.
- **Const generics + const fn** — combine `const N: usize` with `const fn` to generate differently-sized tables for different callers.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Compile-time value | `let () = assert ...` or PPX | `const X: T = expr;` |
| Compile-time function | Meta-programming / PPX | `const fn` — restricted Rust subset |
| Inline compile-time eval | N/A | `const { expr }` block (Rust 1.79+) |
| Heap in const | N/A (GC lang, heap always available) | Forbidden — use `[T; N]` instead of `Vec` |
