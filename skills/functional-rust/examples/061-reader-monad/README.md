# 061: Reader Monad

**Difficulty:** 3  **Level:** Advanced

Dependency injection without a framework — thread shared config through a call chain without passing it to every function.

## The Problem This Solves

You're building a service. It needs a database URL. It needs a log prefix. It needs a feature flag. So you create a `Config` struct and pass `&Config` to your top-level function. That function calls three helpers. Those helpers call more helpers. Now every function in your call stack has `config: &Config` as a parameter, even functions that only need *one field* of the config and only because their callees need the *rest*.

```rust
fn handle_request(req: Request, config: &Config) -> Response {
    let conn = get_connection(config);       // needs db_host, db_port
    let msg  = format_message(req, config);  // needs debug flag
    log_request(req, config);               // needs log_level
    Response { conn, msg }
}

fn get_connection(config: &Config) -> Connection { /* uses 2 fields */ }
fn format_message(req: Request, config: &Config) -> String { /* uses 1 field */ }
fn log_request(req: Request, config: &Config) { /* uses 1 field */ }
```

This is config threading pollution. The function signatures tell a lie: `get_connection` looks like it depends on the whole config, but it only cares about `db_host` and `db_port`. Refactoring the config means touching every function signature. Testing means constructing the full config even when the function only uses one field.

The Reader monad's answer: a function that needs the environment is `Env -> A`. You chain these functions together and only "inject" the environment once at the call site — like how `std::thread::local!` makes a value available everywhere in a thread without passing it explicitly, except composable and testable.

Rust's idiomatic answer: trait bounds. Instead of `&Config`, functions declare exactly what they need: `fn get_connection<E: HasDb>(env: &E)`. Both patterns solve the same problem. This exists to solve exactly that pain.

## The Intuition

Imagine every function in your call chain is a *recipe* that needs an oven. Instead of carrying the oven into every kitchen yourself, you hang the oven on the wall and declare "whoever runs this recipe, the oven is there." The Reader monad is the mechanism for hanging the oven on the wall — it threads the environment implicitly through all the composed functions.

More concretely: a Reader is just a wrapper around a function `|env: &Config| -> A`. When you chain two Readers with `and_then`, the resulting Reader is a new function that passes `env` to both. You only call `.run(&config)` once, at the outermost level.

```rust
// A Reader is just a boxed function: |config| -> result
// Instead of:
fn get_url(config: &Config) -> String { ... }
fn get_prefix(config: &Config) -> &str { ... }

// You combine them into one Reader:
let combined = asks(|c: &Config| c.db_port)  // Reader<Config, u16>
    .map(|port| format!("localhost:{}", port)); // Reader<Config, String>

// The environment is only provided at the end:
let url = combined.run(&config);
```

**Jargon decoded:**
- *Reader monad* — a wrapper around `fn(&Env) -> A` with a `bind` that threads `env` automatically
- *`ask`* — returns the whole environment as the result
- *`asks(f)`* — applies `f` to the environment and returns the result (like `ask().map(f)` but neater)
- *`run(env)`* — provide the environment and get the final result
- *Dependency injection* — making a dependency (config, DB, clock) available to code without passing it explicitly everywhere

## How It Works in Rust

```rust
// Approach 1: The Reader struct (explicit monad encoding)
struct Reader<'a, R, A> {
    run: Box<dyn FnOnce(&R) -> A + 'a>,
}

impl<'a, R: 'a, A: 'a> Reader<'a, R, A> {
    fn new(f: impl FnOnce(&R) -> A + 'a) -> Self {
        Reader { run: Box::new(f) }
    }

    fn run(self, env: &R) -> A {
        (self.run)(env)
    }

    // Transform the result — environment still threads through
    fn map<B: 'a>(self, f: impl FnOnce(A) -> B + 'a) -> Reader<'a, R, B> {
        Reader::new(move |env| f(self.run(env)))
    }
}

// asks: project a field from the environment
fn asks<'a, R: 'a, A: 'a>(f: impl FnOnce(&R) -> A + 'a) -> Reader<'a, R, A> {
    Reader::new(f)
}
```

```rust
// Usage:
struct Config { db_host: String, db_port: u16, debug: bool }

let reader = asks(|c: &Config| c.db_port)
    .map(|port| format!("localhost:{}", port));

let url = reader.run(&config); // "localhost:5432"
```

```rust
// Approach 2: Plain functions taking &Config (idiomatic Rust, no monad)
// This is what most Rust code does — and it's perfectly fine:
fn get_connection_string(config: &Config) -> String {
    format!("{}:{}", config.db_host, config.db_port)
}

fn get_log_prefix(config: &Config) -> &str {
    if config.debug { "[DEBUG] " } else { "[INFO] " }
}

fn format_message(msg: &str, config: &Config) -> String {
    format!(
        "{}{} (connected to {})",
        get_log_prefix(config),
        msg,
        get_connection_string(config),
    )
}
// Config is explicit — you can see exactly what each function needs.
```

```rust
// Approach 3: Trait-based DI — the most idiomatic Rust for production code
// Each function declares only what it actually needs:
trait HasDb {
    fn db_url(&self) -> String;
}

trait HasLogger {
    fn log_prefix(&self) -> &str;
}

impl HasDb for Config {
    fn db_url(&self) -> String { format!("{}:{}", self.db_host, self.db_port) }
}

impl HasLogger for Config {
    fn log_prefix(&self) -> &str { if self.debug { "[DEBUG] " } else { "[INFO] " } }
}

// This function only requires what it uses — honest signature
fn format_msg_generic<E: HasDb + HasLogger>(msg: &str, env: &E) -> String {
    format!("{}{} (connected to {})", env.log_prefix(), msg, env.db_url())
}

// Benefits:
// - Testable: mock just HasDb or just HasLogger in tests
// - Refactoring: change Config fields without touching function signatures
// - Zero-cost: trait dispatch can be monomorphized away at compile time
```

## What This Unlocks

- **Testable services:** Swap `Config` for a test struct that implements the same traits. No dependency injection framework, no mocking library — just different types.
- **Plugin systems:** Functions that need database access declare `T: HasDb`. You can compose them with anything that provides a DB, whether that's your production `Config`, a test double, or a multi-tenant router that picks a DB based on the request.
- **Async contexts:** `axum` and `actix-web` use extractors that are essentially Reader — the request handler declares what it needs (`State<Config>`, `Path<Id>`) and the framework injects them, which is the Reader pattern made ergonomic for HTTP.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Reader type | `type ('r, 'a) reader = Reader of ('r -> 'a)` | `struct Reader<'a, R, A> { run: Box<dyn FnOnce(&R) -> A + 'a> }` |
| Sharing env in `and_then` | Environment is shared freely (immutable, GC'd) | Requires unsafe pointer trick or `Arc<R>` — env can't be borrowed twice through `FnOnce` |
| Idiomatic for DI? | Reader monad is natural | Trait bounds (`T: HasDb + HasLogger`) are more idiomatic |
| Compile-time DI | No (functors/modules achieve it differently) | Yes — `impl Trait` bounds are resolved at compile time |
| Runtime DI | Yes via higher-order functions | Yes via `Box<dyn Trait>` — but prefer static dispatch |
| `ask` in stdlib | No | No — but `axum::extract::State` is the same idea |
