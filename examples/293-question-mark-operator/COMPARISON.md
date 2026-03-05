# OCaml vs Rust: The ? Operator

## Pattern 1: Early Return on Error

### OCaml
```ocaml
let ( let* ) = Result.bind

let parse_and_add s1 s2 =
  let* a = int_of_string_opt s1 |> Option.to_result ~none:"bad" in
  let* b = int_of_string_opt s2 |> Option.to_result ~none:"bad" in
  Ok (a + b)
```

### Rust
```rust
fn parse_and_add(s1: &str, s2: &str) -> Result<i32, ParseError> {
    let a = s1.parse::<i32>()?;
    let b = s2.parse::<i32>()?;
    Ok(a + b)
}
```

## Pattern 2: Option Early Return

### OCaml
```ocaml
let ( let* ) = Option.bind

let lookup env key =
  let* value = List.assoc_opt key env in
  let* n = int_of_string_opt value in
  Some (n * 2)
```

### Rust
```rust
fn lookup(map: &HashMap<&str, &str>, key: &str) -> Option<i32> {
    let value = map.get(key)?;
    let n = value.parse::<i32>().ok()?;
    Some(n * 2)
}
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Syntax | `let* x = expr in` | `let x = expr?;` |
| Desugars to | `Result.bind` / `Option.bind` | `match + return Err(e.into())` |
| Error conversion | Manual | Automatic via `From` trait |
| Works on Option | Yes (with binding ops) | Yes, returns `None` early |
| In closures | Yes | Limited (must return Result/Option) |
