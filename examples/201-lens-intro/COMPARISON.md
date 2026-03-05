# Comparison: Example 201 — The Nested Update Problem

## The Pain: Manual Nested Update

### OCaml
```ocaml
let update_db_port config new_port =
  { config with
    server = { config.server with
      db = { config.server.db with
        port = new_port } } }
```

### Rust
```rust
fn update_db_port(config: &AppConfig, new_port: u16) -> AppConfig {
    AppConfig {
        server: ServerConfig {
            db: DbConfig { port: new_port, ..config.server.db.clone() },
            ..config.server.clone()
        },
        ..config.clone()
    }
}
```

## The Solution: Lens Type

### OCaml
```ocaml
type ('s, 'a) lens = {
  get : 's -> 'a;
  set : 'a -> 's -> 's;
}

let compose outer inner = {
  get = (fun s -> inner.get (outer.get s));
  set = (fun a s -> outer.set (inner.set a (outer.get s)) s);
}
```

### Rust
```rust
struct Lens<S, A> {
    get: Box<dyn Fn(&S) -> A>,
    set: Box<dyn Fn(A, &S) -> S>,
}

impl<S: 'static, A: 'static> Lens<S, A> {
    fn compose<B: 'static>(self, inner: Lens<A, B>) -> Lens<S, B>
    where A: Clone, S: Clone {
        // ... chains get/set through both levels
    }
}
```

## Usage Comparison

### OCaml
```ocaml
let app_db_port = compose (compose server_lens db_lens) port_lens
let new_config = app_db_port.set 5433 config
```

### Rust
```rust
let app_db_port = server_lens().compose(db_lens()).compose(port_lens());
let new_config = (app_db_port.set)(5433, &config);
```
