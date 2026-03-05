# Comparison: Example 180 — PhantomData for API Safety

## Type-State Connection

### OCaml
```ocaml
type _ connection =
  | Closed : string -> closed_state connection
  | Open   : string * int -> open_state connection

let connect (Closed host) : open_state connection = Open (host, 42)
let query (Open (host, _)) sql = "result: " ^ sql
let close (Open (host, _)) : closed_state connection = Closed host
```

### Rust
```rust
struct Connection<State> { host: String, _state: PhantomData<State> }

impl Connection<Closed> {
    fn open(self) -> Connection<Open> { /* ... */ }
}
impl Connection<Open> {
    fn query(&self, sql: &str) -> String { /* ... */ }
    fn close(self) -> Connection<Closed> { /* ... */ }
}
```

## Abstract Module vs Trait

### OCaml
```ocaml
module SafeConn : sig
  type 'a conn
  type opened
  type closed
  val open_conn : closed conn -> opened conn
  val query : opened conn -> string -> string
  val close : opened conn -> closed conn
end
```

### Rust
```rust
// No need for module abstraction — PhantomData + separate impls
// achieves the same: methods only exist on the right state type
impl Connection<Open> {
    fn query(&self, sql: &str) -> String { /* ... */ }
}
// Connection<Closed> simply has no query method
```
