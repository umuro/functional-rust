# OCaml vs Rust: Functional Builder Pattern

## Builder via Method Chaining

### OCaml (Pipeline with Record Update)
```ocaml
type config = { host: string; port: int; tls: bool; (* ... *) }

let default_config = { host = "localhost"; port = 80; tls = false }

let with_host h c = { c with host = h }
let with_port p c = { c with port = p }
let with_tls  b c = { c with tls = b }

let cfg =
  default_config
  |> with_host "api.example.com"
  |> with_port 443
  |> with_tls true
```

### Rust (Consuming Builder)
```rust
impl Config {
    fn host(mut self, h: impl Into<String>) -> Self {
        self.host = h.into();
        self
    }
    fn port(mut self, p: u16) -> Self {
        self.port = p;
        self
    }
    fn tls(mut self, b: bool) -> Self {
        self.tls = b;
        self
    }
}

let cfg = Config::default()
    .host("api.example.com")
    .port(443)
    .tls(true)
    .build()?;
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Mutation** | Immutable update `{ c with }` | `mut self` consumed |
| **Chaining** | Pipeline `\|>` | Dot `.method()` |
| **Ownership** | GC copies | Move semantics |
| **Clone for reuse** | Automatic | Explicit `.clone()` |
| **Validation** | Separate function | `.build()` returns `Result` |

## Benefits

1. **Fluent API** - Readable configuration
2. **Type safety** - Compiler catches missing fields
3. **Immutability** - Each step produces new value
4. **Validation** - Build step can verify constraints
