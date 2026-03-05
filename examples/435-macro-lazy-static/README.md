📖 **[View on hightechmind.io →](https://hightechmind.io/rust/435-macro-lazy-static)**

---

# 435: lazy_static! / OnceLock Pattern

**Difficulty:** 3  **Level:** Advanced

Initialise a global value exactly once on first access and reuse it safely across threads — the modern way with `OnceLock` / `LazyLock`, and what the old `lazy_static!` macro was doing underneath.

## The Problem This Solves

Global constants in Rust must be computable at compile time. That rules out anything that requires heap allocation (`Vec`, `HashMap`, `String`), system calls, or complex computation. Yet programs often need process-wide singletons: a compiled regex, a connection pool, a config map loaded from environment variables, a prime sieve.

The two wrong approaches: compute it every call (wasteful), or initialise it in `main` and thread it through every function as a parameter (ergonomic nightmare). What you want is a global that initialises itself the first time it's needed and is then shared — zero cost on subsequent accesses, thread-safe, no manual synchronisation.

`OnceLock<T>` (stable since Rust 1.70) and `LazyLock<T>` (stable since Rust 1.80) are the standard library answer. `lazy_static!` from the eponymous crate was the community solution before these were stabilised; understanding `OnceLock` demystifies what that macro was generating.

## The Intuition

`OnceLock<T>` is a cell that transitions from "empty" to "full" exactly once. The first caller of `get_or_init(|| ...)` runs the closure and stores the result; all subsequent callers get a reference to the stored value. The transition is atomic — safe across multiple threads racing to initialise the same global.

`LazyLock<T>` wraps this into a static that evaluates its initialiser on first deref, so you don't even need a wrapper function.

The old `lazy_static!` macro generated essentially the same structure: a static `OnceLock`-like wrapper, a function to get the inner reference, and a `Deref` impl to make it transparent. Now the standard library provides this directly.

## How It Works in Rust

```rust
use std::sync::{OnceLock, Mutex};
use std::collections::HashMap;

// ── OnceLock: initialise on first call, reuse forever ────────────────────────
static GLOBAL_CONFIG: OnceLock<HashMap<String, String>> = OnceLock::new();

fn get_config() -> &'static HashMap<String, String> {
    GLOBAL_CONFIG.get_or_init(|| {
        println!("Initializing config (runs ONCE)...");
        let mut m = HashMap::new();
        m.insert("host".to_string(), "localhost".to_string());
        m.insert("port".to_string(), "8080".to_string());
        m
    })
}
// Second call: closure doesn't run; returns cached reference immediately.

// ── Thread-safe mutable singleton ────────────────────────────────────────────
static COUNTER: OnceLock<Mutex<u64>> = OnceLock::new();

fn increment() -> u64 {
    let mut c = COUNTER.get_or_init(|| Mutex::new(0)).lock().unwrap();
    *c += 1;
    *c
}

// ── LazyLock (Rust 1.80+) — closure in the static itself ─────────────────────
// use std::sync::LazyLock;
// static PRIMES: LazyLock<Vec<u32>> = LazyLock::new(|| sieve(100));
// Access: &*PRIMES  or  just  PRIMES[i]  (Deref)

// ── What lazy_static! was generating (simplified) ────────────────────────────
// macro_rules! lazy_static_sim {
//     (static ref $name:ident : $ty:ty = $init:expr ;) => {
//         static $name: OnceLock<$ty> = OnceLock::new();
//         fn get() -> &'static $ty { $name.get_or_init(|| $init) }
//     };
// }
```

**When to use which:**
- `OnceLock<T>` — when you want explicit control over when initialisation happens, or need `set()` separately from `get_or_init()`
- `LazyLock<T>` — when you want the simplest possible "global that initialises itself"
- `lazy_static!` crate — for pre-1.80 compatibility, or in ecosystems that prefer explicit crate usage

## What This Unlocks

- **Compiled regexes as globals** — `static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap())` — compiled once, used everywhere, no per-call overhead.
- **Config loaded at startup** — read environment variables and build a config map once; all code gets a `&'static Config` with no Arc or RefCell.
- **Computed lookup tables** — sieve of Eratosthenes, trigonometric tables, hash seeds — initialised once, accessible from any thread.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Global mutable state | `ref` values at module level; not thread-safe by default | `OnceLock<Mutex<T>>` — thread-safe, initialised once |
| Lazy initialisation | `lazy_t` (3rd party); or `let x = lazy (fun () -> ...)` | `OnceLock::get_or_init` / `LazyLock::new` (std) |
| Thread safety | Not guaranteed; explicit locking | `OnceLock` is `Sync`; initialisation is atomic |
| Equivalent of `lazy_static!` | No standard equivalent | `LazyLock<T>` (Rust 1.80+) |
