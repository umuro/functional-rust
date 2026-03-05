📖 **[View on hightechmind.io →](https://hightechmind.io/rust/430-macro-debugging)**

---

# 430: Debugging Macros with cargo expand

**Difficulty:** 3  **Level:** Advanced

Use `cargo expand` to see exactly what your macros generate, and `compile_error!` to emit helpful diagnostics — the two essential tools when a macro misbehaves.

## The Problem This Solves

Macro errors are notoriously cryptic. The error points to the call site but the bug is in the expansion. You stare at `error[E0308]: mismatched types` in your template code and have no idea which arm matched, what tokens were captured, or what the expanded output looks like. Without tooling, debugging macros means adding `println!` calls you can't even see compile, or mentally tracing recursive expansions by hand.

The other failure mode is silent: the macro *runs* but does the wrong thing. Maybe it matched the wrong arm, maybe a repetition produced one more or fewer item than expected, maybe `$a + sum!($($tail),*)` has a comma issue in the base case. These are impossible to diagnose without seeing the expansion.

`cargo expand` solves this: it runs the macro expander and prints the fully-expanded Rust source. What you see is what the compiler compiles. You can read it, copy it, and compile it manually to isolate issues.

## The Intuition

`cargo expand` is a thin wrapper around the compiler's internal `-Z unpretty=expanded` flag. It produces valid Rust source with all macros expanded — `vec![1,2,3]` becomes its full `Vec` construction, your `sum!(1,2,3)` becomes `1 + (2 + 3)`, your derive impls become fully written trait implementations.

For interactive debugging, `compile_error!` is your `panic!` — it emits a hard error with your message during expansion. `stringify!($x)` shows you exactly what tokens were captured in a fragment. Together, these let you instrument macros like you'd instrument runtime code.

## How It Works in Rust

```rust
// ── Tool 1: cargo expand ─────────────────────────────────────────────────────
// Install: cargo install cargo-expand
// Run:     cargo expand          (entire crate)
//          cargo expand my_mod   (one module)
// What you see: every macro call replaced with its full expansion.

// ── Tool 2: stringify! to inspect captured tokens ────────────────────────────
macro_rules! trace_input {
    ($($x:tt)*) => {{
        // Shows exactly what tokens were matched — indispensable for debugging
        println!("Macro received: {}", stringify!($($x)*));
    }};
}
// trace_input!(1 + 2 * "foo") → prints: Macro received: 1 + 2 * "foo"

// ── Tool 3: compile_error! to diagnose wrong-arm matches ─────────────────────
macro_rules! exactly_two {
    ($a:expr, $b:expr) => { ($a, $b) };
    ($($other:tt)*) => {
        // Shows the offending tokens in the error message
        compile_error!(concat!(
            "exactly_two! needs exactly 2 args, got: ",
            stringify!($($other)*)
        ))
    };
}

// ── Mentally tracing recursive expansion ─────────────────────────────────────
// sum!(1, 2, 3):
//   → 1 + sum!(2, 3)       matches ($head, $($tail),*)
//   → 1 + (2 + sum!(3))    matches ($head, $($tail),*)
//   → 1 + (2 + 3)          matches ($head)
//   = 6

macro_rules! sum {
    () => { 0 };
    ($head:expr $(, $tail:expr)*) => {
        $head + sum!($($tail),*)
    };
}

// ── Nightly: trace_macros! prints each expansion step ────────────────────────
// #![feature(trace_macros)]
// trace_macros!(true);
// sum!(1, 2, 3);  // prints each recursive step to stderr
// trace_macros!(false);
```

**Debugging checklist when a macro fails:**
1. `cargo expand` — see the full expansion
2. `println!("arm: {}", stringify!($x))` — check what was matched
3. `compile_error!` in unexpected arms — force a diagnostic with context
4. Simplify the call site to the minimum failing case

## What This Unlocks

- **Confident macro development** — see exactly what you're generating before the compiler rejects it for a reason you can't interpret.
- **Learning from others' macros** — `cargo expand` on any crate shows you what `#[derive(serde::Serialize)]` actually generates for your type.
- **Better error messages** — `compile_error!` + `stringify!` lets your macros report *why* input was wrong, not just that it was.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| See macro expansion | `ocamlfind ocamlopt -ppx ... -dsource` or `ppxlib` dump | `cargo expand` (cargo-expand crate) |
| Runtime trace | N/A for ppx | `trace_macros!(true)` (nightly) |
| Compile-time error from macro | `Location.raise_errorf` in ppx | `compile_error!("msg")` |
| Inspect matched tokens | N/A | `stringify!($x)` prints token tree as string |
