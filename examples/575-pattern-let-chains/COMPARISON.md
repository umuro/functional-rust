# OCaml vs Rust: Let Chains

## Chained Validation

### OCaml (using let* monadic binding)
```ocaml
let (let*) = Option.bind

let process s =
  let* n = (try Some(int_of_string s) with _ -> None) in
  let* _ = (if n > 0 then Some () else None) in
  let* _ = (if n mod 2 = 0 then Some () else None) in
  Some (n * 2)
```

### Rust (let chains - Rust 1.88+)
```rust
fn process(s: &str) -> Option<i32> {
    if let Ok(n) = s.parse::<i32>()
        && n > 0
        && n % 2 == 0
    {
        Some(n * 2)
    } else {
        None
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Syntax** | `let* x = expr in ...` | `if let pattern = expr && cond` |
| **Mechanism** | Monadic binding (requires `let*` definition) | Built-in syntax |
| **Boolean conditions** | Require wrapping in `Some()`/`None` | Direct `&& condition` |
| **Multiple bindings** | Each needs `let* x = ...` | `&& let pattern = expr` |
| **Scope** | Inside the monadic chain | Body of the `if` block |
| **Fallback** | Implicitly returns `None` | Explicit `else` branch |

## Multiple Pattern Bindings

### OCaml
```ocaml
let make_addr cfg =
  let* host = cfg.host in
  let* port = cfg.port in
  let* _ = if String.length host > 0 then Some () else None in
  let* _ = if port > 0 then Some () else None in
  Some (host ^ ":" ^ string_of_int port)
```

### Rust
```rust
fn make_addr(cfg: &Config) -> Option<String> {
    if let Some(ref host) = cfg.host
        && let Some(port) = cfg.port
        && !host.is_empty()
        && port > 0
    {
        Some(format!("{}:{}", host, port))
    } else {
        None
    }
}
```

## When to Use Each

**Rust let chains are ideal when:**
- Combining pattern matching with boolean guards
- Avoiding nested if-let pyramids
- Working with multiple Option/Result unpacking in conditions

**OCaml let* is ideal when:**
- You already have monadic infrastructure
- Building longer computation chains
- Want early return semantics built into the monad
