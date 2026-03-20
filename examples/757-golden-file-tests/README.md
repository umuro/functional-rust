📖 **[View on hightechmind.io →](https://hightechmind.io/rust/757-golden-file-tests)**

---

# 757-golden-file-tests — Golden File Tests
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Snapshot testing and golden file testing are closely related: both compare actual output against stored expected output. Golden files differ in that the expected output is stored as human-readable files you commit to version control, making output changes visible in code review. This pattern is standard in compilers (test output for each source file), documentation generators, and code formatters. It makes regressions visible as diffs rather than assertion failures.

## Learning Outcomes

- Store expected output in `tests/golden/` files committed to version control
- Compare rendered output against golden files with informative diff-like error messages
- Use an `UPDATE_GOLDEN=1` environment variable to regenerate golden files
- Test complex transformations (Markdown-to-HTML, JSON formatting) against stable golden output
- Understand the golden file review workflow: generate, review diff, commit

## Rust Application

`render_markdown` converts `#`, `##`, and `- ` prefixed lines to HTML elements. `pretty_json` adds indentation to a JSON string. Each function has a corresponding golden file in `tests/golden/`. The `assert_golden` helper reads the expected file, compares against actual output, and either fails with a message showing the first difference, or writes the new content if `UPDATE_GOLDEN=1`. Tests for both functions verify complete input-to-output transformations.

## OCaml Approach

OCaml's compiler test suite uses golden files extensively — each `.ml` test file has a corresponding `.expected` file. `dune runtest` and `dune promote` manage the workflow. The `expect_test` framework (Jane Street) is an inline variant. For standalone golden tests, OCaml uses `read_file`, runs the transformation, and calls `assert_equal ~pp_diff` with `Alcotest` for pretty diff output on failure.

## Key Differences

1. **Inline vs file**: OCaml's `expect_test` stores expected output inline; Rust's golden file approach uses separate files (closer to the compiler testing tradition).
2. **Review workflow**: Both use a "promote/update" workflow; Rust uses `UPDATE_GOLDEN=1` env var; OCaml uses `dune promote`.
3. **Diff output**: OCaml's `Alcotest.check string` provides colored line diffs; this example shows the first differing line; the `insta` crate provides full colored diffs.
4. **Granularity**: Golden files are per-transformation; `expect_test` is per-assertion within a test function.

## Exercises

1. Add a `render_csv_to_markdown_table` function and create a golden file for its output. Write a test that regenerates it with `UPDATE_GOLDEN=1 cargo test`.
2. Implement `assert_golden_json` that normalizes JSON whitespace before comparing, so formatting changes don't cause spurious golden file failures.
3. Write a script `scripts/update_golden.sh` that runs all tests with `UPDATE_GOLDEN=1` and prints the number of golden files updated.
