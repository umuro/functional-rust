# OCaml vs Rust: String Pattern Matching

## Basic String Matching

### OCaml
```ocaml
let cmd s = match s with
  | "quit" | "exit" | "q" -> "quit"
  | "help" | "?" | "h"    -> "help"
  | ""                     -> "empty"
  | _                      -> "unknown"
```

### Rust
```rust
fn classify_cmd(s: &str) -> &'static str {
    match s {
        "quit" | "exit" | "q" => "quit",
        "help" | "?" | "h" => "help",
        "" => "empty",
        _ => "unknown",
    }
}
```

## String Matching with Guards

### OCaml
```ocaml
let cmd s = match s with
  | s when String.length s > 0 && s.[0] = '/' -> "command"
  | _ -> "unknown"
```

### Rust
```rust
fn classify_cmd(s: &str) -> &'static str {
    match s {
        s if s.starts_with('/') => "command",
        _ => "unknown",
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Syntax** | `match s with` | `match s { }` |
| **OR patterns** | `"a" \| "b" -> ...` | `"a" \| "b" => ...` |
| **Guards** | `when condition` | `if condition` |
| **Binding in guard** | `s when f(s)` | `s if f(s)` |
| **String type** | `string` (owned) | `&str` (borrowed) |
| **Case insensitive** | Manual with `String.lowercase` | `eq_ignore_ascii_case` |

## Working with Owned Strings

### Rust - Matching &String
```rust
let owned = String::from("Monday");
// &String derefs to &str automatically
match owned.as_str() {
    "Monday" => "weekday",
    _ => "unknown",
}
// Or simply: match &owned[..] { ... }
// Or: match &*owned { ... }
```

String patterns in Rust work with `&str`. When you have a `String`, use `.as_str()` or deref coercion.
