📖 **[View on hightechmind.io →](https://hightechmind.io/rust/340-async-trait-pattern)**

---

# 340: Async Trait Pattern
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Rust traits with `async fn` methods have a fundamental limitation: the returned `Future` type differs per implementation, making traits with async methods not object-safe. The workaround is to return `Pin<Box<dyn Future<Output = T> + Send>>` — a heap-allocated, type-erased future. The `async-trait` crate automates this boxing. Understanding the manual pattern illuminates what the macro generates and when to use each approach.

## Learning Outcomes

- Understand why `async fn` in traits is not directly object-safe
- Implement async traits using `Pin<Box<dyn Future<...>>>` return types manually
- Use `AsyncResult<T, E>` type alias for cleaner signatures
- Recognize when to use the `async-trait` crate vs manual boxing

## Rust Application

Manual async trait with boxed futures:

```rust
pub type AsyncResult<T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + Send>>;

pub trait AsyncStore: Send + Sync {
    fn get(&self, key: &str) -> AsyncResult<Option<String>, String>;
    fn set(&self, key: String, value: String) -> AsyncResult<(), String>;
}

impl AsyncStore for MemStore {
    fn get(&self, key: &str) -> AsyncResult<Option<String>, String> {
        let data = self.data.lock().unwrap().get(key).cloned();
        Box::pin(async move { Ok(data) })  // wrap sync result in future
    }
}
```

## OCaml Approach

OCaml's module types with Lwt functions are the idiomatic equivalent — each module implementing the signature provides its own `Lwt.t`-returning functions:

```ocaml
module type STORE = sig
  val get : string -> string option Lwt.t
  val set : string -> string -> unit Lwt.t
end
```

Module types are inherently polymorphic — no boxing is required.

## Key Differences

1. **Stable Rust (1.75+)**: Return-position `impl Trait` in traits (`RPITIT`) was stabilized — `async fn` in traits now works on stable Rust without the `async-trait` crate in many cases.
2. **Object safety**: For `dyn AsyncStore`, boxing is still required; for monomorphic dispatch, stable `async fn` in traits now works.
3. **async-trait crate**: `#[async_trait]` macro transforms `async fn` methods to return `Pin<Box<dyn Future>>` automatically — reducing boilerplate.
4. **Performance**: Boxed futures allocate per-call; unboxed `async fn` in traits (stable Rust 1.75+) avoids this for concrete types.

## Exercises

1. Implement two different `AsyncStore` backends (in-memory and a mock filesystem), and swap them in a function that takes `&dyn AsyncStore`.
2. Use the `async-trait` crate and compare its generated code to the manual boxing approach.
3. Benchmark the performance difference between a boxed async trait method and a non-boxed concrete `async fn` call.
