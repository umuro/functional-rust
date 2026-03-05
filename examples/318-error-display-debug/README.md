📖 **[View on hightechmind.io →](https://hightechmind.io/rust/318-error-display-debug)**

---

# 318: Display vs Debug for Errors

**Difficulty:** 2  **Level:** Intermediate

Two audiences, two formats — know which one to use where.

## The Problem This Solves

You implement an error type, derive `Debug`, and ship it. Then a user reports that your error message is `DbError::ConnectionFailed("db.prod")` — the raw Debug format leaking internal struct names. Meanwhile, your logging framework is printing beautiful human messages for other errors. The difference is whether `Display` is implemented.

`Debug` and `Display` serve different audiences. `Debug` is for developers: it should show the full internal structure, variant names, and any details useful for diagnosing a bug. It's what appears in `{:?}` format strings, in test failure output, and in `dbg!()` calls. `Display` is for users: it should be a clean, natural-language sentence with no Rust syntax — the message that might appear in a UI or be spoken aloud.

The `Error` trait requires both: you need `Debug` (usually derived for free) and `Display` (always implemented manually). If you only derive `Debug`, your error messages look like Rust code. If you implement only `Display`, you can't print the full debug representation. You need both, serving different purposes.

## The Intuition

`Debug` is for developers (`{:?}` in logs and tests), `Display` is for users (`{}` in messages) — every error type needs both, and they should say different things.

## How It Works in Rust

```rust
#[derive(Debug)]  // generates: DbError::ConnectionFailed("db.prod") — useful for devs
enum DbError {
    ConnectionFailed(String),
    QueryTimeout(f64),
    NotFound(String),
}

impl fmt::Display for DbError {  // implement manually — this is what users see
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionFailed(h) => write!(f, "Cannot connect to {h}"),
            Self::QueryTimeout(s)     => write!(f, "Query timed out after {s:.1}s"),
            Self::NotFound(k)         => write!(f, "Record not found: {k}"),
        }
    }
}

impl std::error::Error for DbError {}  // requires Display + Debug

// In practice:
let e = DbError::ConnectionFailed("db.prod".into());
println!("{e}");    // Display: "Cannot connect to db.prod"
println!("{e:?}");  // Debug:   "ConnectionFailed(\"db.prod\")"

// Log structured errors with both:
eprintln!("error: {e}");          // user-facing
log::error!("db error: {e:?}");   // developer log with full debug info

// Box<dyn Error> preserves both:
let boxed: Box<dyn std::error::Error> = Box::new(e);
println!("{boxed}");  // calls Display
```

The Display message should work as part of a sentence: "failed to connect: {e}" should read naturally. Avoid trailing periods (callers may add context after it), and avoid leading capital letters in library errors (callers control the framing).

## What This Unlocks

- **User-friendly error messages** — `{}` gives clean natural language; no Rust syntax leaks to end users
- **Developer diagnostics** — `{:?}` exposes full internal state for debugging; the derive covers 90% of cases
- **Error ecosystem integration** — `Box<dyn Error>`, `anyhow`, and every logging framework use `Display` — you must implement it

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Human-readable message | `Format.pp` or `to_string` | `impl fmt::Display` — required by `Error` |
| Debug representation | `[@@deriving show]` / `Printexc` | `#[derive(Debug)]` — structural output |
| Required by Error | No standard error trait | Both `Display` and `Debug` required |
| Format specifier | N/A | `{}` = Display; `{:?}` = Debug; `{:#?}` = pretty Debug |
