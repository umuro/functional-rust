# OCaml vs Rust: anyhow Pattern

## Pattern: Universal Error Container

### Rust
```rust
type AnyResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

fn parse_port(s: &str) -> AnyResult<u16> {
    let n: u16 = s.parse()?;  // auto-boxed
    if n == 0 { return Err("port cannot be zero".into()); }
    Ok(n)
}
```

### OCaml
```ocaml
(* Use exceptions for untyped errors *)
exception AppError of string

let parse_port s =
  match int_of_string_opt s with
  | None -> raise (AppError "invalid port")
  | Some n when n = 0 -> raise (AppError "port cannot be zero")
  | Some n -> n
```

## Pattern: Adding Context

### Rust
```rust
let port = parse_port(port_str).context("invalid port")?;
```

### OCaml
```ocaml
try parse_port s with
| AppError msg -> raise (AppError ("invalid port: " ^ msg))
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Untyped error | `exn` (exceptions) | `Box<dyn Error>` |
| String as error | `Failure "msg"` | `"msg".into()` |
| Context | Catch and re-raise | `.context()` extension |
| Library vs app | Same mechanism | Library: typed; App: boxed |
| Thread safety | N/A | `+ Send + Sync` bounds |
