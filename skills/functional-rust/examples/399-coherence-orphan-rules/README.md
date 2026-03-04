# 399: Coherence and Orphan Rules

**Difficulty:** 3  **Level:** Advanced

You can only implement a trait for a type if you own the trait OR the type — and you can only implement it once.

## The Problem This Solves

Imagine two crates both implement `Display` for `i64`. Your program depends on both. Which implementation wins? The compiler can't know. This ambiguity would make Rust programs fragile — swapping a dependency could silently change behavior, and conflicting crates couldn't coexist.

The orphan rule prevents this: an implementation of trait `T` for type `S` is only allowed if `T` or `S` (or both) is defined in the current crate. You can't implement a foreign trait on a foreign type. This guarantees that every (trait, type) pair has at most one implementation globally, which the compiler calls **coherence**.

This constraint is real and sometimes frustrating — you can't implement `serde::Serialize` for `chrono::DateTime` in your binary crate. But it's also why Rust can link crates together without ambiguity, and why changing a dependency never silently breaks trait resolution.

## The Intuition

Three combinations are possible:

1. **Your trait + your type** — always OK
2. **Your trait + foreign type** — OK (you own the trait)
3. **Foreign trait + your type** — OK (you own the type)
4. **Foreign trait + foreign type** — **FORBIDDEN** (orphan rule)

The newtype pattern is the standard workaround for case 4: wrap the foreign type in a new struct you own. Now it's your type, and you can implement any foreign trait on it.

Coherence is enforced globally: even case 1–3 allows only ONE implementation per (trait, type) pair. Duplicate impls are a compile error, regardless of where they live.

## How It Works in Rust

```rust
// Your own trait
trait Summarize {
    fn summarize(&self) -> String;
}

struct Article { title: String, content: String }

// VALID: your trait + your type
impl Summarize for Article {
    fn summarize(&self) -> String {
        format!("{}: {}...", self.title, &self.content[..50.min(self.content.len())])
    }
}

// VALID: your trait + foreign type (String is from std, but you own Summarize)
impl Summarize for String {
    fn summarize(&self) -> String { format!("\"{}\"", &self[..20.min(self.len())]) }
}

// VALID: your trait + foreign type
impl Summarize for Vec<i32> {
    fn summarize(&self) -> String { format!("{} integers", self.len()) }
}

// FORBIDDEN: foreign trait (Display) + foreign type (i64)
// impl std::fmt::Display for i64 {}  // ERROR: E0117 — orphan rule violation

// WORKAROUND: newtype wraps the foreign type — now it's YOUR type
struct Wrapper(Vec<i32>);

// VALID: foreign trait + your type (Wrapper)
impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]",
            self.0.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
    }
}

fn main() {
    let article = Article {
        title: "Rust Rocks".to_string(),
        content: "Rust is a systems programming language focused on safety.".to_string(),
    };
    println!("{}", article.summarize());
    println!("{}", "Hello World".to_string().summarize());
    println!("{}", Wrapper(vec![10, 20, 30]));
}
```

The newtype costs nothing at runtime — the compiler optimizes it away. The only price is manually delegating methods you want from the inner type (`self.0.len()`, etc.) or implementing `Deref` to forward them automatically.

## What This Unlocks

- **Stable trait resolution across crates** — no ambiguity, no silent behavior changes when dependencies change.
- **Newtype pattern** — wrapping foreign types to add custom behavior without forking; used throughout the ecosystem (`Wrapper(Vec<_>)`, `Meters(f64)`, `UserId(u64)`).
- **Predictable library composition** — two crates that both use `serde` will never conflict on how `String` is serialized.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Orphan rule | No global coherence — modules are namespaced, no conflict enforcement | Strict global coherence — one impl per (trait, type) pair, enforced by compiler |
| Foreign type workaround | Wrap in a new module with its own types | Newtype pattern: `struct Wrapper(ForeignType)` |
| Duplicate impls | Different modules can define `Printable` for `int` independently | Compile error anywhere in the crate graph |
| Trade-off | Flexible but can silently conflict | Restrictive but guarantees unambiguous dispatch |
