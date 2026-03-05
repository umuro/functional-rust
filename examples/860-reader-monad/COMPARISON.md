# Comparison: Reader Monad

## Reader Type

**OCaml:**
```ocaml
type ('r, 'a) reader = Reader of ('r -> 'a)
let ask = Reader (fun env -> env)
let asks f = Reader (fun env -> f env)
```

**Rust:**
```rust
struct Reader<R, A> { run: Box<dyn FnOnce(&R) -> A> }
fn ask<R: Clone>() -> Reader<R, R> { Reader::new(|env| env.clone()) }
fn asks<R, A>(f: impl FnOnce(&R) -> A) -> Reader<R, A> { Reader::new(f) }
```

## Environment-Based Computation

**OCaml:**
```ocaml
let format_message msg =
  asks (fun c -> if c.debug then "[DEBUG] " else "[INFO] ") >>= fun prefix ->
  asks (fun c -> c.db_host ^ ":" ^ string_of_int c.db_port) >>= fun conn ->
  return_ (prefix ^ msg ^ " (" ^ conn ^ ")")
```

**Rust (idiomatic — just pass &Config):**
```rust
fn format_message(msg: &str, config: &Config) -> String {
    format!("{}{} (connected to {}:{})",
        if config.debug { "[DEBUG] " } else { "[INFO] " },
        msg, config.db_host, config.db_port)
}
```

**Rust (trait-based DI):**
```rust
trait HasDb { fn db_url(&self) -> String; }
trait HasLogger { fn log_prefix(&self) -> &str; }

fn format_msg<E: HasDb + HasLogger>(msg: &str, env: &E) -> String {
    format!("{}{} (connected to {})", env.log_prefix(), msg, env.db_url())
}
```
