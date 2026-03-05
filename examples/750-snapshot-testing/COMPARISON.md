# OCaml vs Rust: Snapshot Testing

## Basic Snapshot Pattern

### Rust (std-only)
```rust
pub fn assert_snapshot(name: &str, actual: &str) {
    let path = snapshot_path(name);
    
    if !path.exists() || should_update() {
        fs::write(&path, actual)?;
        return;
    }
    
    let expected = fs::read_to_string(&path)?;
    if actual != expected {
        panic!("Snapshot mismatch!\n{}", compute_diff(&expected, &actual));
    }
}
```

### OCaml (expect_test)
```ocaml
let%expect_test "render report" =
  let report = render_report [("Widget", 10); ("Gadget", 20)] in
  print_string report;
  [%expect {|
    === Sales Report ===
      1. Widget               10
      2. Gadget               20
    ====================
    Total items: 2
    Total qty:   30
  |}]
```

## Updating Snapshots

### Rust
```bash
UPDATE_SNAPSHOTS=1 cargo test
```

### OCaml
```bash
dune runtest --auto-promote
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Library | expect_test (inline) | insta, or std-only |
| Storage | Inline in source | Separate .snap files |
| Update mode | `--auto-promote` | `UPDATE_SNAPSHOTS=1` |
| Inline vs external | Inline in test | External files |
| Diff output | Built-in | Custom or library |

## When to Use Snapshots

- Complex output (reports, formatted strings)
- Serialized data (JSON, YAML)
- UI rendering output
- Any output where manual assertions are tedious
