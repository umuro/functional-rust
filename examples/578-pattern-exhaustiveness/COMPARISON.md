# OCaml vs Rust: Pattern Exhaustiveness

## Exhaustive Match

### OCaml
```ocaml
type dir = N | S | E | W

let describe = function
  | N -> "north"
  | S -> "south" 
  | E -> "east"
  | W -> "west"
(* Compiler warns if any case is missing *)
```

### Rust
```rust
enum Dir { N, S, E, W }

fn describe(d: Dir) -> &'static str {
    match d {
        Dir::N => "north",
        Dir::S => "south",
        Dir::E => "east",
        Dir::W => "west",
    }
}
// Compiler errors if any case is missing
```

## Non-Exhaustive Enums

### OCaml
```ocaml
(* No built-in mechanism - use wildcard by convention *)
let status_text = function
  | OK -> "OK"
  | NotFound -> "Not Found"
  | _ -> "Unknown"  (* by convention for library types *)
```

### Rust
```rust
#[non_exhaustive]
enum StatusCode { Ok, NotFound, /* ... */ }

fn status_text(c: StatusCode) -> &'static str {
    match c {
        StatusCode::Ok => "OK",
        StatusCode::NotFound => "Not Found",
        _ => "Unknown",  // Required by #[non_exhaustive]
    }
}
```

## Range Patterns

### OCaml
```ocaml
let classify n =
  if n < 0 then "negative"
  else if n = 0 then "zero"
  else "positive"
(* No range patterns in match *)
```

### Rust
```rust
fn classify(n: i32) -> &'static str {
    match n {
        i32::MIN..=-1 => "negative",
        0 => "zero",
        1..=i32::MAX => "positive",
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Missing case** | Warning (can be ignored) | Compile error |
| **Non-exhaustive** | Convention only | `#[non_exhaustive]` attribute |
| **Range matching** | Not supported in match | `start..=end` patterns |
| **Wildcard** | `_` | `_` (same) |
