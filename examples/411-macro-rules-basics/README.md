📖 **[View on hightechmind.io →](https://hightechmind.io/rust/411-macro-rules-basics)**

---

# 411: `macro_rules!` Basics
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Some code patterns cannot be abstracted with functions. A function that takes a variable number of arguments of different types, generates different code branches depending on the caller's syntax, or captures the source location of its invocation requires metaprogramming. Rust's `macro_rules!` is the declarative macro system: pattern-match on the input token stream, transform it, and emit generated code. Unlike C preprocessor macros, `macro_rules!` is hygienic (generated identifiers don't leak), syntactically aware, and integrated into the module system.

`macro_rules!` powers `vec!`, `println!`, `assert_eq!`, `format!`, `todo!`, `unimplemented!`, and hundreds of third-party macros.

## Learning Outcomes

- Understand how `macro_rules!` pattern-matches on token trees to generate code
- Learn the basic fragment designators: `expr`, `ident`, `ty`, `block`, `tt`
- See how multiple arms enable function-like overloading on syntax
- Understand hygiene: macro-generated variables don't leak into the call site scope
- Learn when macros are appropriate vs. functions (variadic args, syntax capture, codegen)

## Rust Application

In `src/lib.rs`, `check_eq!` has two arms: one without a message and one with. Both expand to an `if` with a `panic!`. The `repeat!` macro takes a count `$n:expr` and a `$body:block`, expanding to a `for` loop. `min_of!` is recursive — the single-argument arm is the base case, the multi-argument arm delegates to itself. `#[macro_export]` makes macros available to crate users.

## OCaml Approach

OCaml uses PPX (preprocessor extensions) and camlp5 for syntactic metaprogramming. PPX attributes (`[@attr]`) and extensions (`[%ext ...]`) trigger compile-time code transformation via external plugins. `ppx_deriving` generates trait implementations; `ppx_sexp_conv` generates S-expression serialization. Unlike Rust's built-in `macro_rules!`, OCaml's PPX requires external libraries and a build system plugin.

## Key Differences

1. **Built-in vs. external**: Rust's `macro_rules!` is in the language; OCaml requires PPX dependencies and `dune` plugin configuration.
2. **Hygiene**: Rust macros are hygienic — generated identifiers don't conflict with caller-scope names; OCaml PPX extensions have no hygienic scope.
3. **Pattern matching**: Rust matches on token streams with named fragments; OCaml PPX transforms the AST directly using OCaml pattern matching on `Parsetree` types.
4. **Error messages**: Rust macro expansion errors show the expansion point; OCaml PPX errors appear at the expansion site but can be harder to trace.

## Exercises

1. **Hash map macro**: Implement `hashmap!{ "key" => value, ... }` that creates a `HashMap` from key-value pairs. Handle the empty case `hashmap!{}` returning an empty map.
2. **Assert with context**: Create `assert_between!(val, lo, hi)` that panics with a message showing the value and the expected range, using `file!()` and `line!()` in the panic message.
3. **Timing macro**: Implement `time_it!(label, expr)` that evaluates `expr`, prints `"{label} took {elapsed:?}"` using `std::time::Instant`, and returns the expression value.
