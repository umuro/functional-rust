📖 **[View on hightechmind.io →](https://hightechmind.io/rust/746-doc-test-patterns)**

---

# 746-doc-test-patterns — Doc Test Patterns
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Documentation that diverges from reality is worse than no documentation: it misleads users and erodes trust. Rust's doc tests solve this by compiling and running every `///` code example as a test. If you change the function signature or behavior, the doc test fails — documentation can never silently go out of date. This is used pervasively in the Rust standard library, the `serde` crate, and virtually every published Rust crate.

## Learning Outcomes

- Write code examples in `///` doc comments using fenced ` ``` ` blocks
- Understand that `cargo test` runs all doc examples as real test cases
- Use `#` prefix to hide boilerplate lines in rendered docs while keeping them in the test
- Mark examples with `no_run` (compiles but skips execution) or `ignore` (skips entirely)
- Write doc tests that cover both success cases and documented error conditions

## Rust Application

`clamp(lo, hi, x)` has examples for below-range, in-range, and above-range inputs. `repeat(s, n)` has examples for typical use, zero repetitions, and empty strings. `split_once_char` has examples for found, not-found, and multiple-delimiter cases. Each example uses `use example_746_doc_test_patterns::clamp;` to import from the crate, exactly as a user would. `cargo test --doc` runs only the doc examples.

## OCaml Approach

OCaml does not have a built-in doc-test runner. The `mdx` (Markdown eXtended) tool runs OCaml code blocks in markdown documentation. `ocamldoc` generates HTML from `(** ... *)` comments but does not execute examples. The `doctests` opam package provides pytest-style doc testing. Jane Street uses `expect_test` for inline tests with expected output embedded in source comments.

## Key Differences

1. **First-class support**: Rust's `cargo test --doc` runs doc examples with no additional tooling; OCaml requires `mdx` or external tooling.
2. **Compilation**: Rust doc examples are fully compiled against the crate's API; OCaml's `mdx` interprets examples via the REPL without full type checking.
3. **Visibility control**: Rust's `#`-prefix hides boilerplate while keeping it executable; OCaml's `mdx` has no equivalent line-hiding syntax.
4. **Integration**: Rust doc tests appear in `cargo test` output alongside unit tests; OCaml doc tests require a separate `mdx` invocation.

## Exercises

1. Add a `///` doc example to `split_once_char` that shows the `?` operator usage in a function that returns `Option`, including the hidden `fn main() -> Option<()>` wrapper.
2. Write a `parse_ip` function with doc examples for IPv4, IPv6, and malformed inputs, using `no_run` for any example that requires network access.
3. Add an `Examples` section to a `Config::from_str` function with a multi-step example that shows parsing, validation, and accessing fields — verify it compiles and runs with `cargo test --doc`.
