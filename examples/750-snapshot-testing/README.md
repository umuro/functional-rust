📖 **[View on hightechmind.io →](https://hightechmind.io/rust/750-snapshot-testing)**

---

# 750-snapshot-testing — Snapshot Testing
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Complex output — formatted reports, rendered templates, generated code, pretty-printed data structures — is hard to test with `assert_eq!` because the expected string is long and fragile. Snapshot testing stores the first run's output as a "golden file" and asserts subsequent runs produce identical output. When output legitimately changes, run with `UPDATE_SNAPSHOTS=1` to accept the new output. Used in `insta` (Rust), `jest` (JavaScript), and `expect_test` (OCaml/Reason).

## Learning Outcomes

- Implement a snapshot assertion that reads from a `.snap` file and compares against actual output
- Use an `UPDATE_SNAPSHOTS=1` environment variable to regenerate snapshot files
- Understand the commit workflow: generate snapshots, review the diff, commit them alongside code
- Recognize which types of output benefit from snapshot testing (complex, multi-line, structured)
- Build `render_report` and `render_json_like` as examples of complex output worth snapshotting

## Rust Application

`render_report` formats a sales report with aligned columns, totals, and a header/footer. `render_json_like` produces a multiline key-value structure. The `snapshot_assert` function reads the expected output from `tests/snapshots/{name}.snap`, and either asserts equality or writes the new file if `UPDATE_SNAPSHOTS=1` is set. The `should_update()` function reads the environment variable; `snapshot_path` constructs the file path.

## OCaml Approach

OCaml's `expect_test` (Jane Street) embeds expected output directly in source comments: `[%expect {| output here |}]`. Running `dune runtest` compares actual vs expected; `dune promote` accepts changes. The `mdx` tool serves a similar purpose for documentation examples. Unlike file-based snapshots, `expect_test` keeps expected output adjacent to the test code, making diffs easier to review.

## Key Differences

1. **Storage**: Rust's `insta` stores snapshots in separate `.snap` files; OCaml's `expect_test` stores them inline in source code.
2. **Review workflow**: Rust uses `cargo insta review` for interactive snapshot review; OCaml uses `dune promote` to accept all changes at once.
3. **Granularity**: Rust snapshots are per-assertion; OCaml `expect_test` captures all `print_*` output within a test block.
4. **Serialization**: Rust's `insta` can snapshot any `Debug`-printable value automatically; OCaml's `expect_test` requires explicit `Sexp.to_string` or `printf` calls.

## Exercises

1. Add a `render_html_table` function that renders `&[(&str, u32)]` as an HTML table, and create a snapshot test for it that is stored in `tests/snapshots/html_table.snap`.
2. Implement a `diff_snapshots` function that shows a colored line-by-line diff when a snapshot fails, making it easier to review what changed.
3. Write a snapshot test for a recursive JSON structure with nested objects and arrays — verify that indentation and comma placement are stable across runs.
