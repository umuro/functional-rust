📖 **[View on hightechmind.io →](https://hightechmind.io/rust/201-lens-intro)**

---

# 201: The Nested Update Problem — Why Lenses Exist

**Difficulty:** ⭐⭐⭐  **Level:** Intermediate

A Lens is a composable getter+setter that makes deep immutable updates painless.

## The Problem This Solves

You have a deeply nested struct. You need to change one field three levels down. In Rust, because all values are immutable by default (or because you want to preserve the original), you have to rebuild every layer by hand:

```rust
let updated = AppConfig {
    server: ServerConfig {
        db: DbConfig {
            port: 5433,
            ..config.server.db.clone()
        },
        ..config.server.clone()
    },
    ..config.clone()
};
```

That's 8 lines to change **one field**. Add a fourth nesting level and it becomes 11. The signal-to-noise ratio is awful — the actual intent (change the port to 5433) is buried under structural noise.

You might try writing helper functions like `map_server` and `map_db` — that helps, but now you have a proliferation of one-off helpers that don't compose. You can't combine `map_server` with `map_db` in a generic way; you have to write a new function for every path through the struct.

A Lens solves this by giving you a reusable, **composable** get+set pair. Two Lenses snap together like LEGO: a Lens from `AppConfig → ServerConfig` composed with a Lens from `ServerConfig → DbConfig` gives you a Lens from `AppConfig → DbConfig`. This example exists to solve exactly that pain.

## The Intuition

Think of a Lens like a **labelled drill bit**. You have a drill press (the Lens), and you press it into a struct. It knows exactly where to go and can either *read* what's there or *replace* it — without disturbing anything else.

The key insight: a Lens is just two functions bundled together:
- `get`: `(&S) -> A` — read the focused value
- `set`: `(A, &S) -> S` — produce a new `S` with the focused value replaced

```rust
struct Lens<S, A> {
    get: Box<dyn Fn(&S) -> A>,
    set: Box<dyn Fn(A, &S) -> S>,
}
```

Here `S` is the "source" (the outer struct you hold) and `A` is the "focus" (the field you care about). The Lens doesn't store the struct — it stores the *knowledge of how to reach into it*.

## How It Works in Rust

**Step 1 — Define a Lens for each field:**

```rust
fn server_lens() -> Lens<AppConfig, ServerConfig> {
    Lens::new(
        |c| c.server.clone(),                           // get: extract server
        |s, c| AppConfig { server: s, ..c.clone() },   // set: rebuild with new server
    )
}

fn db_lens() -> Lens<ServerConfig, DbConfig> {
    Lens::new(
        |s| s.db.clone(),
        |d, s| ServerConfig { db: d, ..s.clone() },
    )
}

fn port_lens() -> Lens<DbConfig, u16> {
    Lens::new(
        |d| d.port,
        |p, d| DbConfig { port: p, ..d.clone() },
    )
}
```

**Step 2 — Compose them to reach any depth:**

```rust
// This composed lens goes all the way from AppConfig to u16 (the port)
let app_db_port = server_lens()
    .compose(db_lens())
    .compose(port_lens());
```

Composition works by threading the get and set functions: the composed `get` calls the outer `get`, then the inner `get`. The composed `set` calls the outer `get` to extract the intermediate value, calls the inner `set` to build the updated intermediate, then calls the outer `set` to rebuild the top level.

**Step 3 — Use it:**

```rust
// Read
let port = (app_db_port.get)(&config); // 5432

// Write — one line, any depth
let updated = (app_db_port.set)(5433, &config);
```

The struct update boilerplate is now inside the Lens definitions, written **once**. All call sites are a single line.

## What This Unlocks

- **Zero-boilerplate deep updates**: change any nested field in one line regardless of nesting depth.
- **Reusable paths**: define `server_lens` once, compose it with anything — no new helpers needed per call site.
- **Composable structure**: two Lenses snapped together give a new Lens; the composition law means you can build paths incrementally and they always work as expected.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Struct update syntax | `{ x with field = v }` — no clone needed | `Struct { field: v, ..x.clone() }` — requires `Clone` |
| Lens type | Record of two functions, no allocation concern | `Box<dyn Fn>` closures — heap allocated |
| Composition | Natural function composition, infix `\|>>` | `.compose()` method chain |
| Cost | GC handles memory; no explicit clone | Every `set` clones the struct at each level |
| Ergonomics | Very concise | More verbose but fully type-safe |
