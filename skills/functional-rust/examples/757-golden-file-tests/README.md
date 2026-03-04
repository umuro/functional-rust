# 757: Golden File Testing Pattern

**Difficulty:** 2  **Level:** Intermediate

Capture the output of a function to a `.txt` file on first run; on subsequent runs, compare against the saved "golden" output and fail if it changes.

## The Problem This Solves

Some outputs are too complex to write expected values for by hand. Pretty-printed ASTs, formatted reports, rendered templates, compiler diagnostic text — you want to ensure these outputs don't *change* accidentally, but writing `assert_eq!(output, "Add\n  Mul\n    Num(2)\n    Var(x)\n  Num(3)\n")` is painful and brittle.

Golden file tests (also called snapshot tests) solve this: run the function once, save the output as the "expected" file, commit it to version control. Every subsequent run compares against that file. If the output changes — intentionally or by accident — the test fails, and you review the diff. If the change is intentional, update with `UPDATE_GOLDEN=1 cargo test`.

This pattern is widely used in compilers (LLVM's `FileCheck`), CLI tools, formatters, and any code with complex human-readable output.

## The Intuition

The golden file *is* the test assertion. You write the code, run it once to generate the golden files, commit them. Now your test is "does the current output match what it looked like when I last said it was correct?"

The `UPDATE_GOLDEN=1` environment variable acts as an explicit "accept this output" signal — you must deliberately choose to update, which prevents accidental regressions being silently accepted.

## How It Works in Rust

**The infrastructure** — compare or update based on `UPDATE_GOLDEN`:
```rust
pub fn assert_golden(name: &str, actual: &str) {
    let path = PathBuf::from("tests/golden").join(format!("{}.txt", name));
    let update = std::env::var("UPDATE_GOLDEN").map(|v| v == "1").unwrap_or(false);

    if !path.exists() || update {
        std::fs::create_dir_all("tests/golden").unwrap();
        std::fs::write(&path, actual).unwrap();
        return;
    }

    let expected = std::fs::read_to_string(&path).unwrap();
    // Normalize line endings (Windows ↔ Unix)
    assert_eq!(
        actual.replace("\r\n", "\n"),
        expected.replace("\r\n", "\n"),
        "Golden file mismatch for '{}'. Run with UPDATE_GOLDEN=1 to update.", name
    );
}
```

**Using it in tests:**
```rust
#[test]
fn golden_tree_render() {
    let expr = make_expr();
    assert_golden("expr_tree", &render_tree(&expr, 0));
}
```

**Generating the golden file** (first run or after intentional change):
```bash
UPDATE_GOLDEN=1 cargo test
```
The `tests/golden/expr_tree.txt` file is created or updated.

**Committing golden files** — add `tests/golden/` to version control. The diff in your PR shows exactly what output changed, making code review of rendering logic much easier.

**Normalizing line endings** — always strip `\r\n` to `\n` before comparison to avoid false failures on Windows CI runners.

## What This Unlocks

- **Complex output regression prevention** — catch accidental changes to AST rendering, report formatting, or diagnostic text.
- **Readable test maintenance** — update golden files intentionally with a single env var, then `git diff` shows exactly what changed.
- **Documentation by example** — golden files in version control are living examples of what the function produces.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Snapshot testing | `ppx_expect` (Jane Street) | Manual golden-file infra, or `insta` crate |
| File I/O in tests | `open_in` / `output_string` | `std::fs::read_to_string` / `std::fs::write` |
| Environment variable | `Sys.getenv` | `std::env::var("KEY")` → `Result<String, VarError>` |
| Test update workflow | `PPXEXPECT_UPDATE=true` | `UPDATE_GOLDEN=1 cargo test` |
