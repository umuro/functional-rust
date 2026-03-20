📖 **[View on hightechmind.io →](https://hightechmind.io/rust/777-const-assert-patterns)**

---

# 777-const-assert-patterns — Const Assert Patterns

## Problem Statement

Runtime assertions (`assert!`) check invariants at program execution time — too late for configuration errors that could be caught earlier. Compile-time assertions (`const_assert!`) catch impossible configurations, wrong struct sizes, invalid constant values, and violated mathematical invariants during compilation. Used in embedded systems (verify that a packet type fits in a buffer), cryptography (check key size assumptions), and any code with compile-time-knowable invariants.

## Learning Outcomes

- Implement `const_assert!(cond)` as a macro that evaluates a boolean constant at compile time
- Use `const_assert_eq!(a, b)` for equality checks on compile-time constants
- Validate struct sizes with `const_assert_size!(Type, expected_bytes)` 
- Enforce configuration invariants: `BUFFER_SIZE` between `MIN` and `MAX`, power-of-two alignment
- Understand that failing `const_assert!` produces a compile error, not a runtime panic

## Rust Application

The `const_assert!` macro expands to `const _: () = assert!(cond)`, evaluated at compile time. `const_assert_eq!` checks `a == b`. `const_assert_size!` uses `std::mem::size_of::<T>()`. Constants `MIN_BUFFER_SIZE = 64`, `MAX_BUFFER_SIZE = 4096`, `BUFFER_SIZE = 256` are validated against each other. A `Header` struct with `magic: [u8; 4]` and `version: u32` is asserted to be exactly 8 bytes. A power-of-two assertion uses `.is_power_of_two()`.

## OCaml Approach

OCaml's module system allows some compile-time verification via module signatures. However, OCaml has no direct equivalent of `const_assert`. The closest is `let () = assert (constant_expression)` which evaluates at module initialization time, catching errors on first load. `ppx_const` provides conditional compilation. In practice, OCaml relies on type-level encoding (like phantom types with specific sizes) for compile-time invariants.

## Key Differences

1. **Error timing**: Rust's `const_assert!` fails at compile time with a clear message; OCaml's `assert` fails at module initialization (still early, but not compile time).
2. **Size assertions**: `std::mem::size_of::<T>()` is a `const fn` in Rust; OCaml's `Obj.size` is runtime-only.
3. **Macro system**: Rust's `macro_rules!` can generate compile-time assertions; OCaml's ppx macros can generate code but not arbitrary compile-time checks.
4. **Use in embedded**: Rust's `const_assert!` is used in `no_std` firmware to verify buffer sizes without any runtime overhead; OCaml cannot target such environments.

## Exercises

1. Write a `const_assert_alignment!(Type, alignment)` macro that checks `std::mem::align_of::<Type>() == alignment` and test it on types with known alignment requirements.
2. Use `const_assert!` to enforce that a `PacketHeader` struct fits in a 64-byte cache line (`size_of::<PacketHeader>() <= 64`).
3. Create a `const_assert_range!(val, lo, hi)` macro and use it to validate that all timeout constants in a configuration module are within acceptable bounds.
