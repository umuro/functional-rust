# 750: Snapshot Testing: Expect Files Pattern

**Difficulty:** 2  **Level:** Intermediate

Save expected output to a file on first run; compare on every subsequent run — catches output regressions automatically.

## The Problem This Solves

Some functions produce complex, multi-line output that's hard to `assert_eq!` by hand: formatted reports, JSON, serialized structures, rendered templates. Writing all those expected strings inline in test code is tedious and brittle — they break on cosmetic changes and require manual updates. Snapshot testing flips the workflow: run the test once to *capture* the expected output, then every future run *compares* against that snapshot. If the output changes, the test fails and shows a diff.

This is the pattern behind popular testing tools like Jest's `.toMatchSnapshot()`, Insta (a Rust crate), and `expect_test` (used in rust-analyzer). It's especially valuable for refactoring: after verifying that your refactored code produces the same output as before, you have strong evidence of behavioral equivalence.

The key workflow is: first run creates `tests/snapshots/name.snap`; subsequent runs compare against it; `UPDATE_SNAPSHOTS=1 cargo test` updates snapshots when intentional changes are made.

## The Intuition

The snapshot infrastructure is just three operations: **create** (write actual output to a `.snap` file if it doesn't exist), **compare** (read the `.snap` file and diff against current output), and **update** (overwrite the `.snap` file when output changes intentionally). The diff is shown on failure so you can see exactly what changed. Line ending normalization (`\r\n` → `\n`) prevents false failures on Windows/Linux cross-platform runs.

## How It Works in Rust

```rust
const SNAPSHOT_DIR: &str = "tests/snapshots";

fn should_update() -> bool {
    std::env::var("UPDATE_SNAPSHOTS").map(|v| v == "1").unwrap_or(false)
}

pub fn assert_snapshot(name: &str, actual: &str) {
    let path = Path::new(SNAPSHOT_DIR).join(format!("{}.snap", name));

    if !path.exists() || should_update() {
        // First run: create/update the snapshot
        fs::create_dir_all(SNAPSHOT_DIR).expect("create snapshot dir");
        fs::write(&path, actual).expect("write snapshot");
        eprintln!("[snapshot:{}] {}", name,
            if should_update() { "Updated" } else { "Created" });
        return;
    }

    let expected = fs::read_to_string(&path).expect("read snapshot");
    // Normalize line endings for cross-platform stability
    let actual_norm   = actual.replace("\r\n", "\n");
    let expected_norm = expected.replace("\r\n", "\n");

    if actual_norm != expected_norm {
        panic!("Snapshot '{}' mismatch!\nTo update: UPDATE_SNAPSHOTS=1 cargo test\n\nDiff:\n{}",
               name, compute_diff(&expected_norm, &actual_norm));
    }
}

#[test]
fn snapshot_sales_report() {
    let data = &[("Apples", 42u32), ("Bananas", 17), ("Cherries", 99)];
    assert_snapshot("sales_report", &render_report(data));
    // First run: creates tests/snapshots/sales_report.snap
    // Subsequent runs: compares against it
}
```

Commit the `.snap` files to version control — they're the ground truth. Code review shows diffs in snapshot files alongside code diffs, making output changes visible and reviewable.

## What This Unlocks

- **Regression detection without manual expected values** — any output change (intended or not) shows up as a test failure with a precise diff; no more "did this change the output?" uncertainty after refactoring.
- **Review-friendly output changes** — when you intentionally change output format, `UPDATE_SNAPSHOTS=1 cargo test` updates all snapshots atomically; the git diff of `.snap` files documents exactly what changed.
- **Complement to unit tests** — unit tests verify logic; snapshot tests verify output shape and formatting; use both for complete coverage of complex rendering functions.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Snapshot infrastructure | `ppx_expect` (Jane Street) | Hand-rolled (this example) or `insta` crate |
| Update mechanism | `EXPECT_TEST_UPDATE=1` / review workflow | `UPDATE_SNAPSHOTS=1 cargo test` (env var pattern) |
| File I/O in tests | `open_in` / `read_line` | `fs::read_to_string` / `fs::write` — stdlib |
| Cross-platform line endings | Usually not an issue | Normalize `\r\n` → `\n` for portability |
