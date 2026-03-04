# 213: Practical Lens — Deeply Nested Config Update

**Difficulty:** 3  **Level:** Advanced

Immutably update deeply nested structs using composed lenses, avoiding boilerplate clone chains.

## The Problem This Solves

Real-world configs are deeply nested. A typical app has `App → Server → DB → Pool → max_size`. Without lenses, updating one field requires rebuilding every level manually: clone the pool, update the one field, clone the db with the new pool, clone the server with the new db, clone the app with the new server. Four levels of boilerplate for one field change.

This gets worse at scale. A `configure_for_production` function that adjusts six settings across four levels produces pages of identical clone-and-update code. Every level is an opportunity for a typo that compiles but silently uses the wrong field.

Lenses solve this by separating "where to look" from "what to do". A `Lens<App, u32>` focused on pool size composes from four smaller lenses — one per nesting level. Once composed, `lens.over(|n| n * 2, &config)` handles all the rebuilding. The structure stays immutable, the code stays concise, and adding a new transformation is one line.

## The Intuition

Think of each lens as a pointer into a nested structure. `app_server` points to the `server` field. `server_db` points to the `db` field inside that. Composing them gives you a pointer directly to `db` inside `app`, skipping the intermediate level.

When you call `set` or `over` on a composed lens, it rebuilds every intermediate level automatically, cloning only what changes. You describe the path; the lens handles the plumbing.

Rust's struct update syntax — `Config { field: new_value, ..existing }` — is the FP idiom that lenses generalise. Instead of writing `..existing` manually at every level, the lens does it for you.

## How It Works in Rust

```rust
// Lens<S, A>: focuses on field of type A inside structure S
struct Lens<S, A> {
    get: Box<dyn Fn(&S) -> A>,
    set: Box<dyn Fn(A, &S) -> S>,  // returns new S with A replaced
}

impl<S: 'static, A: 'static> Lens<S, A> {
    // Compose: focus on B inside A inside S → Lens<S, B>
    fn compose<B: 'static>(self, inner: Lens<A, B>) -> Lens<S, B> where A: Clone {
        // get: reach A in S, then reach B in A
        // set: reach A in S, set B in A, then set new A in S
    }

    fn over(&self, f: impl FnOnce(A) -> A, s: &S) -> S {
        (self.set)(f((self.get)(s)), s)
    }
}

// Build individual lenses for each level
fn app_server() -> Lens<AppConfig, ServerConfig> { /* focuses on .server field */ }
fn server_db()  -> Lens<ServerConfig, DbConfig>  { /* focuses on .db field */ }
fn db_pool()    -> Lens<DbConfig, PoolConfig>    { /* focuses on .pool field */ }
fn pool_max()   -> Lens<PoolConfig, u32>         { /* focuses on .max_size field */ }

// Compose: direct path from AppConfig to pool.max_size
let app_pool_max = app_server().compose(server_db()).compose(db_pool()).compose(pool_max());

// Update: one line, all levels rebuilt automatically
let prod_config = app_pool_max.over(|_| 50, &dev_config);
```

## What This Unlocks

- **Config pipelines** — chain multiple lens operations for `configure_for_production`, `configure_for_test`, etc. Each step is one `over` call.
- **Immutable domain models** — update nested order structs, user records, or game state without mutation or accidental aliasing.
- **Refactoring safety** — when a field moves to a deeper struct, only the lens definition changes; all callers stay the same.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Record update | `{ config with field = v }` | `Config { field: v, ..config.clone() }` |
| Pipeline | `config \|> app_pool_max %~ (fun _ -> 50)` | `let c = app_pool_max.over(\|_\| 50, &c)` |
| Clone cost | None (GC shares structure) | Explicit `.clone()` at each rebuild level |
| Lens creation | Record literal | Function returning `Lens` struct |
| Composition | Infix operator `>>` or `%` | `.compose(inner)` method |
