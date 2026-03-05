📖 **[View on hightechmind.io →](https://hightechmind.io/rust/431-macro-count-pattern)**

---

# 431: Counting Elements at Compile Time

**Difficulty:** 3  **Level:** Advanced

Count the number of arguments passed to a variadic macro at compile time — enabling fixed-size arrays, static dispatch tables, and compile-time arity validation with no runtime overhead.

## The Problem This Solves

`macro_rules!` macros accept variadic input via `$($x:expr),*`, but there's no built-in `len` or `count`. When you need to create a fixed-size array from variadic arguments, or assert that exactly N items were passed, you need to count them yourself.

The naive recursive counting approach works but can hit the recursion limit for large inputs. The substitution trick — replacing each token with `()` and measuring the length of the resulting slice — is O(1) and the idiomatic solution for anything non-trivial.

Compile-time counting also enables patterns that would otherwise require runtime bookkeeping: a dispatch table whose size is known to the compiler, a `const N: usize` derived from a list of items, or a `compile_error!` if the wrong number of arguments is passed.

## The Intuition

The substitution trick is clever: replace every token in the variadic list with `()` (unit), put all those units into a slice literal `[(), (), ()]`, and call `.len()` on it. Since all the values are `()`, the compiler knows the length at compile time and optimises it to a constant. No recursion, no stack overflow, no iteration.

The recursive approach (`1 + count!($($tail)*)`) reads more naturally but recurses once per element. For short lists (< 64 items) it's fine. For longer lists or when you want a guaranteed `const`, prefer the substitution trick.

## How It Works in Rust

```rust
// ── Recursive count (clear, limited depth) ───────────────────────────────────
macro_rules! count {
    () => { 0usize };
    ($head:tt $($tail:tt)*) => { 1 + count!($($tail)*) };
}

// ── Substitution trick (preferred for large lists) ───────────────────────────
macro_rules! replace_with_unit { ($anything:tt) => { () }; }

macro_rules! count_tts {
    ($($tts:tt)*) => {
        // Replaces each token with () then measures slice length — O(1), const
        <[()]>::len(&[$(replace_with_unit!($tts)),*])
    };
}

// ── Count expressions ─────────────────────────────────────────────────────────
macro_rules! count_exprs {
    () => { 0usize };
    ($e:expr $(, $rest:expr)*) => { 1 + count_exprs!($($rest),*) };
}

// ── Build a fixed-size array — size inferred from argument count ──────────────
macro_rules! fixed_array {
    ($($val:expr),* $(,)?) => {{
        const N: usize = count_exprs!($($val),*);
        let arr: [i32; N] = [$($val,)*];
        arr
    }};
}

let arr = fixed_array![10, 20, 30, 40, 50];
// arr has type [i32; 5] — the 5 is a compile-time constant

// ── Static dispatch table with known size ────────────────────────────────────
macro_rules! dispatch_table {
    ($($name:ident : $fn:expr),* $(,)?) => {{
        const SIZE: usize = count_exprs!($($fn),*);
        let names: [&str; SIZE] = [$(stringify!($name),)*];
        let funcs: [fn(i32) -> i32; SIZE] = [$($fn,)*];
        (names, funcs)
    }};
}

let (names, funcs) = dispatch_table!(
    double: |x| x * 2,
    square: |x| x * x,
    negate: |x| -x,
);
// SIZE = 3 at compile time, no Vec allocation
```

## What This Unlocks

- **Fixed-size arrays from variadic macros** — generate `[T; N]` arrays where `N` is the argument count, unlocking stack allocation and `const` contexts.
- **Compile-time arity validation** — `assert!(count_exprs!(...) == expected)` inside a `const _` block fails the build if the wrong number of items is provided.
- **Plugin/dispatch tables** — build a `[fn(...); N]` array of handlers where the size is known to the compiler, enabling bounds-free indexing.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Variadic argument count | Not directly — use lists; `List.length` at runtime | Compile-time via substitution trick or recursive `macro_rules!` |
| Fixed-size array from variadic input | Not possible with standard syntax | `[T; N]` where `N = count_exprs!(...)` — zero runtime cost |
| Compile-time assertions | `[@@if]` guards in ppx | `const _: () = assert!(count == expected)` |
| Dispatch table sizing | `Array.make n` at runtime | `[fn; N]` with `N` as compile-time constant |
