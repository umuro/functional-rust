# OCaml vs Rust: Golden File Tests

## Golden File Assertion

### Rust
```rust
pub fn assert_golden(name: &str, actual: &str, golden_dir: &Path) {
    let golden_path = golden_dir.join(format!("{}.golden", name));
    
    if !golden_path.exists() {
        fs::write(&golden_path, actual)?;
        return;
    }
    
    let expected = fs::read_to_string(&golden_path)?;
    assert_eq!(actual, expected, "Golden test '{}' failed!", name);
}
```

### OCaml (expect_test)
```ocaml
let%expect_test "markdown rendering" =
  let output = render_markdown "# Title" in
  print_string output;
  [%expect {| <h1>Title</h1> |}]
```

## Updating Golden Files

### Rust
```bash
# Typically delete .golden files and re-run tests
rm tests/golden/*.golden
cargo test
```

### OCaml
```bash
dune runtest --auto-promote
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Storage | Inline in source | External .golden files |
| Update | `--auto-promote` | Delete and regenerate |
| Library | expect_test (ppx) | Manual or insta crate |
| Diff display | Built-in | Manual |
