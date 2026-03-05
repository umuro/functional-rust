# OCaml vs Rust: From Trait for Errors

## Pattern 1: Automatic Conversion

### OCaml
```ocaml
(* Must manually convert at every call site *)
let process s =
  let* n = parse_number s |> Result.map_error (fun e -> ParseErr e) in
  let* v = validate n in
  Ok v
```

### Rust
```rust
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::Parse(e) }
}

fn process(s: &str) -> Result<i32, AppError> {
    let n = s.parse::<i32>()?;  // auto-converts via From
    Ok(n)
}
```

## Pattern 2: Manual vs Implicit

### OCaml
```ocaml
Result.map_error (fun e -> wrap e) result
```

### Rust
```rust
result?  // Calls From::from() automatically
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Conversion | `Result.map_error` at each site | `impl From<E>` once |
| Triggering | Explicit | Implicit via `?` |
| Type inference | N/A | Compiler selects `From` impl |
| Boilerplate | Scattered | Centralized |
