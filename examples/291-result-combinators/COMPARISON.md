# OCaml vs Rust: Result Combinators

## Pattern 1: Map Ok Value

### OCaml
```ocaml
let ok = Ok 5 in
let mapped = Result.map (fun x -> x * 2) ok
(* Ok 10 *)
```

### Rust
```rust
let doubled: Result<i32, String> = Ok(5).map(|x| x * 2);
// Ok(10)
```

## Pattern 2: Chain Fallible Operations

### OCaml
```ocaml
let result = 
  parse "10" 
  |> Result.bind (fun n -> divide n 2)
```

### Rust
```rust
let result = parse_int("10").and_then(|n| divide(n, 2));
```

## Pattern 3: Transform Error

### OCaml
```ocaml
let rich_error = 
  Result.map_error (fun e -> "Parse failed: " ^ e) (parse "abc")
```

### Rust
```rust
let rich = "bad".parse::<i32>()
    .map_err(|e| format!("Validation failed: {}", e));
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Map Ok | `Result.map f r` | `r.map(f)` |
| Chain fallible | `Result.bind r f` | `r.and_then(f)` |
| Map error | `Result.map_error f r` | `r.map_err(f)` |
| Fallback | Custom match | `r.or_else(f)` |
| Default value | `Result.value ~default r` | `r.unwrap_or(default)` |
