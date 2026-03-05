# 340: Async Trait Pattern

**Difficulty:** 4  **Level:** Expert

Async methods in traits require boxing — `async fn` in traits isn't directly supported in stable Rust without the `async-trait` crate.

## The Problem This Solves

You want `trait Storage { async fn get(&self, key: &str) -> Option<String>; }` to swap implementations. But the return type of async functions is opaque, making the trait non-object-safe. Solution: return `Pin<Box<dyn Future>>` explicitly.

## The Intuition

In TypeScript:
```ts
interface Storage {
  get(key: string): Promise<string | null>;
}
```
`Promise<T>` is always heap-allocated. Rust's `Pin<Box<dyn Future>>` is the explicit equivalent.

## How It Works in Rust

```rust
type AsyncResult<T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + Send>>;

trait AsyncStore: Send + Sync {
    fn get(&self, key: &str) -> AsyncResult<Option<String>, String>;
    fn set(&self, key: String, val: String) -> AsyncResult<(), String>;
}

impl AsyncStore for MemStore {
    fn get(&self, key: &str) -> AsyncResult<Option<String>, String> {
        let result = self.data.lock().unwrap().get(key).cloned();
        Box::pin(async move { Ok(result) })
    }
}

// Both implementations usable through trait object
let stores: Vec<Box<dyn AsyncStore>> = vec![
    Box::new(MemStore::new()),
    Box::new(FailStore),
];
```

With `#[async_trait]`, this simplifies to `async fn get(...)`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Async trait | Module type with `Lwt.t` | Explicit `Pin<Box<dyn Future>>` |
| Dynamic dispatch | First-class module | `dyn Trait` |
| Boxing overhead | Always boxed (GC) | Explicit `Box::pin` |
