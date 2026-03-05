# OCaml vs Rust: Builder with Closures

## OCaml
```ocaml
type server_config = {
  host: string;
  port: int;
  on_connect: string -> unit;
}

let default_config = {
  host = "localhost";
  port = 8080;
  on_connect = fun _ -> ();
}

let with_host host cfg = { cfg with host }
let with_on_connect f cfg = { cfg with on_connect = f }
```

## Rust
```rust
pub struct ServerBuilder {
    config: ServerConfig,
}

impl ServerBuilder {
    pub fn on_connect(mut self, f: impl Fn(&str) + 'static) -> Self {
        self.config.on_connect = Box::new(f);
        self
    }

    pub fn build(self) -> ServerConfig { self.config }
}
```

## Key Differences

1. **OCaml**: Record update syntax `{ cfg with field = value }`
2. **Rust**: Fluent builder with method chaining
3. Both: Closures as configuration callbacks
4. **Rust**: Box<dyn Fn> for stored callbacks
5. Both support configurable behavior via closures
