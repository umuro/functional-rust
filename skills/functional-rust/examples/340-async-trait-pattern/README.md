# 340: Async Trait Pattern

**Difficulty:** 4  **Level:** Expert

Async methods in traits require boxing — `async fn` in traits isn't directly supported in stable Rust without the `async-trait` crate.

## The Problem This Solves

You want to define a trait with async methods — `trait Storage { async fn get(&self, key: &str) -> Option<String>; }` — so you can swap implementations (in-memory, Redis, PostgreSQL) behind a common interface. But Rust traits can't directly express `async fn` in a stable, object-safe way: the return type of an async function is an opaque `impl Future`, and different implementations may return different future types. That makes the trait non-object-safe — you can't use `dyn Storage`.

The solution: return `Pin<Box<dyn Future<Output=Result<T,E>> + Send>>` explicitly. Each implementation boxes its async block. The `async-trait` crate (`#[async_trait]`) generates this boxing automatically, making the ergonomics identical to writing `async fn` in an impl block.

This pattern is essential for writing testable, pluggable async services: mock stores in tests, real stores in production, both behind the same trait.

## The Intuition

In TypeScript, you'd write:
```ts
interface Storage {
  get(key: string): Promise<string | null>;
  set(key: string, val: string): Promise<void>;
}
```
`Promise<T>` is always heap-allocated — there's no equivalent to Rust's stack futures. Rust's `Pin<Box<dyn Future>>` is the explicit version of the same thing: heap-allocated, type-erased, sendable.

## How It Works in Rust

```rust
// Type alias for readability
type AsyncResult<T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + Send>>;

trait AsyncStore: Send + Sync {
    fn get(&self, key: &str) -> AsyncResult<Option<String>, String>;
    fn set(&self, key: String, val: String) -> AsyncResult<(), String>;
}

// In-memory implementation
impl AsyncStore for MemStore {
    fn get(&self, key: &str) -> AsyncResult<Option<String>, String> {
        let result = self.data.lock().unwrap().get(key).cloned();
        // Box::pin wraps the async block and pins it to the heap
        Box::pin(async move { Ok(result) })
    }
    fn set(&self, key: String, val: String) -> AsyncResult<(), String> {
        self.data.lock().unwrap().insert(key, val);
        Box::pin(async { Ok(()) })
    }
}

// Both can be used through the same trait object
let stores: Vec<Box<dyn AsyncStore>> = vec![
    Box::new(MemStore::new()),
    Box::new(FailStore),
];
```

With the `async-trait` crate, this simplifies to:
```rust
#[async_trait]
trait AsyncStore: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<String>, String>;
    async fn set(&self, key: String, val: String) -> Result<(), String>;
}
```
The crate generates the identical `Pin<Box<dyn Future>>` code behind the scenes.

## What This Unlocks

- **Pluggable storage backends** — swap in-memory, Redis, or PostgreSQL implementations behind one trait.
- **Testable async services** — inject a `MockStore` in tests, a real store in production.
- **Middleware / decorator pattern** — wrap a `dyn AsyncStore` with logging, caching, or retry logic.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Async trait methods | Module type with `'a Lwt.t` return | Explicit `Pin<Box<dyn Future>>` or `#[async_trait]` |
| Dynamic dispatch | First-class module / polymorphism | `dyn Trait` (requires object-safe return types) |
| Trait object | `(module Store)` pattern | `Box<dyn AsyncStore>` |
| Boxing overhead | Always boxed (GC managed) | Explicit `Box::pin(async { ... })` per call |
