📖 **[View on hightechmind.io →](https://hightechmind.io/rust/428-macro-hygiene)**

---

# 428: Macro Hygiene

## Problem Statement

C preprocessor macros are famously dangerous because they operate via text substitution: `#define DOUBLE(x) x * 2` expands `DOUBLE(a + b)` to `a + b * 2` — not `(a + b) * 2`. Variable name collisions are another hazard: a macro using `int result = ...` conflicts with any `result` variable in the expansion scope. Rust's `macro_rules!` is hygienic: identifiers introduced inside a macro expansion live in a separate scope from the call site. `let result = $val` inside a macro doesn't shadow `result` outside it.

Hygiene is what makes Rust's macros safe to use in large codebases without name collision nightmares — it's a fundamental property that distinguishes `macro_rules!` from C preprocessor macros.

## Learning Outcomes

- Understand what macro hygiene means: each macro expansion gets its own identifier scope
- Learn how `let result = $val` inside a macro doesn't capture the caller's `result`
- See the `with_counter!(|c| { c += 1; })` pattern for intentional hygiene-breaking
- Understand when hygiene can be intentionally bypassed (passing identifier arguments)
- Learn the difference between hygienic `macro_rules!` and non-hygienic proc macros

## Rust Application

In `src/lib.rs`, `hygienic_example!` creates a local `result` variable inside its block. The caller also has a `result` variable. The test in `test_hygiene` verifies that `result + doubled == 20` — the macro's `result = 5` doesn't affect the outer `result = 10`. The `with_counter!` macro takes `|$c:ident|` as an argument, letting the caller name the counter variable — explicit hygiene bypass.

## OCaml Approach

OCaml's PPX extensions are not hygienic in the way Rust's `macro_rules!` is. Generated code identifiers can conflict with surrounding code. PPX authors must use `Ast_builder.gen_symbol` or `fresh_var` utilities to generate unique names. This is the same problem Rust's `macro_rules!` solves automatically. OCaml's `let open Module in` scoping provides some protection, but not systematic hygiene.

## Key Differences

1. **Automatic hygiene**: Rust `macro_rules!` is automatically hygienic for introduced variables; OCaml PPX requires explicit fresh identifier generation.
2. **Span-based**: Rust's hygiene is based on `Span` — identifiers have a "context" indicating which expansion created them; OCaml has no equivalent span-based hygiene.
3. **Proc macro hygiene**: Rust's proc macros are not hygienic by default (they use `Span::call_site()`) but can opt into hygiene with `Span::def_site()`.
4. **Fragment capture**: Rust's `$expr:expr` captured fragments maintain their own hygiene context; OCaml's AST captures are transparent.

## Exercises

1. **Hygiene test**: Write a macro `swap!(a, b)` that swaps two variables using a temporary. Verify that the temporary doesn't conflict with variables named `tmp` or `temp` in the calling scope.
2. **Intentional capture**: Implement `with_err!(|e| { ... })` where `e` is a user-named error variable bound inside the macro. Show that the caller controls the name and can use `with_err!(|my_err| { handle(my_err) })`.
3. **Demonstrate non-hygiene**: Write a C-style macro alternative using string manipulation (as a thought exercise) and explain how three different variable naming conflicts would occur in C but not in Rust's hygienic `macro_rules!`.
