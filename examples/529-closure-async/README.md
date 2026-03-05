📖 **[View on hightechmind.io →](https://hightechmind.io/rust/529-closure-async)**

---

# 529: Async Closures

**Difficulty:** 4  **Level:** Advanced

Closures that return `Future`s — the standard pattern for async callbacks, retry logic, and concurrent pipelines.

## The Problem This Solves

You want to pass an async operation as a callback — `retry(3, || fetch(url))` where each attempt is async. But `fetch(url)` returns a `Future`, and closures returning `Future`s have complex type requirements. The first attempt usually produces compiler errors about `Fn` bounds not being satisfied or lifetimes being wrong.

True `async ||` closures are nightly-only in Rust. On stable, you write `|| async { ... }` — a regular closure that returns an anonymous `async` block. The types get intricate: you need `Fn() -> impl Future<Output = T>` or, for dynamic dispatch, `Box<dyn Fn() -> BoxFuture<'a, T>>`.

Without understanding this pattern, async callbacks become a wall of incomprehensible errors. With it, you can build retry logic, concurrent processing, and async event handlers cleanly.

## The Intuition

An async closure is a closure that returns a future instead of a value. Calling it starts a computation; awaiting the result runs it. Think of it like a factory that produces `Promise`s in JavaScript: `() => fetch(url)` — each call creates a new pending promise.

In JavaScript: `const withRetry = (fn, n) => { for(let i=0;i<n;i++) try { return await fn() } catch {} }` — `fn` is an async callback. Rust's equivalent requires spelling out the `Future` type explicitly, because Rust's type system doesn't have implicit promise boxing.

The workhorse pattern: `|x| async move { use_x(x).await }`. The `move` transfers `x` into the async block, making the resulting `Future` `'static` (owns its data, not borrowing from the closure's scope).

## How It Works in Rust

```rust
use std::future::Future;
use std::pin::Pin;

// Type alias for boxed futures — common in async callback APIs
type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

// Retry pattern: async closure called up to N times
async fn retry<T, E, F, Fut>(max_attempts: usize, f: F) -> Result<T, E>
where
    F: Fn() -> Fut,                // closure takes no args, returns a Future
    Fut: Future<Output = Result<T, E>>,
{
    for attempt in 0..max_attempts {
        match f().await {           // create and await a new future each call
            Ok(val) => return Ok(val),
            Err(e)  => {
                eprintln!("Attempt {} failed", attempt + 1);
                if attempt + 1 == max_attempts { return Err(e); }
            }
        }
    }
    unreachable!()
}

// Pattern 1: closure returning async block (stable Rust)
let async_double = |x: i32| async move { x * 2 };
// Type: impl Fn(i32) -> impl Future<Output = i32>

// Pattern 2: async map — process items sequentially
async fn async_map<T, U, F, Fut>(items: Vec<T>, f: F) -> Vec<U>
where F: Fn(T) -> Fut, Fut: Future<Output = U> {
    let mut results = Vec::new();
    for item in items {
        results.push(f(item).await);   // sequential: one at a time
    }
    results
}

// Pattern 3: boxed async closure for dynamic dispatch
fn make_processor(prefix: String) -> impl Fn(i32) -> BoxFuture<'static, String> {
    move |x: i32| {
        let prefix = prefix.clone();     // clone for each future (owned)
        Box::pin(async move {            // Box::pin for heap-allocated Future
            format!("{}: {}", prefix, x * 2)
        })
    }
}

// Pattern 4: async with captured reference — use move to own it
async fn process_all(urls: Vec<String>) -> Vec<Result<String, String>> {
    let mut results = Vec::new();
    for url in urls {
        // move url into the async block — Future owns it
        let result = (|| async move { fake_fetch(&url).await })().await;
        results.push(result);
    }
    results
}
```

The key rule: **if the closure's `Future` must be `'static`** (sent to a thread or stored), use `async move { ... }` to transfer ownership into the future. If the future only lives within the closure's scope, you can borrow.

## What This Unlocks

- **Retry and timeout wrappers** — `retry(3, || async { fetch(url).await })` wraps any async operation with retry logic without modifying the operation.
- **Async middleware pipelines** — pass async transformation functions through a processing chain where each step may await I/O.
- **Concurrent async event handlers** — store `Box<dyn Fn(Event) -> BoxFuture<'static, ()>>` handlers that spawn async work when events arrive.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Async function | External libraries (Lwt, Async) | `async fn` / `impl Future` — built-in |
| Async closure | `fun x -> Lwt.return x` | `\|x\| async move { x }` (stable) or `async \|x\| { x }` (nightly) |
| Await | `Lwt.bind` / `let*` | `.await` — postfix operator |
| Boxed future | `Lwt.t` (always boxed) | `Pin<Box<dyn Future<...>>>` — explicit |
| `'static` future | GC handles | `async move` — closure owns all captures |
