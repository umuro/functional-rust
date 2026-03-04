# 333: Async Recursion

**Difficulty:** 4  **Level:** Expert

Recursive async functions need `Box::pin` — the future's size must be known at compile time.

## The Problem This Solves

When you write a recursive function in Rust, the compiler calculates the function's stack frame size. For a plain function that's fine. But an `async fn` is secretly transformed into a state machine struct — and a recursive async function would produce a struct that *contains itself*, making its size infinite. The compiler refuses this outright.

The fix is heap-allocation: instead of returning `impl Future<Output=T>` (unknown/infinite size), you return `Pin<Box<dyn Future<Output=T>>>` — a fat pointer of known size that points to the heap-allocated future. The `async-recursion` crate (`#[async_recursion]`) generates this boilerplate automatically. Without the crate, you write `Box::pin(async move { ... })` by hand.

This pattern shows up any time you traverse a recursive data structure asynchronously: JSON tree parsing, directory scanning, graph traversal, or — as in this example — computing properties of a binary tree.

## The Intuition

In Python asyncio:
```python
async def async_sum(tree):
    if tree is None: return 0
    return tree.value + await async_sum(tree.left) + await async_sum(tree.right)
```
Python hides the complexity — every coroutine is already heap-allocated. Rust exposes it because it normally stores futures on the stack for performance. The `Box::pin` is you saying "ok, put this one on the heap."

Think of `Pin<Box<dyn Future>>` as Rust's equivalent of a manually heap-allocated coroutine.

## How It Works in Rust

```rust
// Type alias for brevity — a heap-pinned future with lifetime 'a
type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

fn async_sum(t: &Tree) -> BoxFuture<'_, i64> {
    // Box::pin(...) wraps the async block and pins it to the heap
    Box::pin(async move {
        match t {
            Tree::Leaf => 0,
            Tree::Node { value, left, right } =>
                // .await works normally inside the box
                *value as i64 + async_sum(left).await + async_sum(right).await,
        }
    })
}
```

Why `async move`? The closure captures `t` by value (actually by reference, bounded by `'_`). The `move` ensures ownership semantics are explicit even when borrowing.

The minimal `block_on` executor at the bottom drives the futures to completion — in a real project you'd use `tokio::main` or `tokio::runtime::Runtime::block_on`.

## What This Unlocks

- **Async tree traversal** — traverse ASTs, JSON trees, or DOM structures with async lookups at each node.
- **Async directory scanning** — `async fn scan(dir: &Path) -> BoxFuture<Vec<File>>` that recurses into subdirectories.
- **Graph algorithms** — async DFS / BFS that fetches node data from a database or remote API at each step.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive async | Direct recursion with Lwt (`let rec f x = ... >>= f y`) | Must use `Box::pin(async { ... })` |
| Stack vs heap | Lwt always heap-allocates continuations | Rust normally stack-allocates; `Box` opts into heap |
| Return type | `'a Lwt.t` (always a pointer) | `Pin<Box<dyn Future<Output=T>>>` (explicit) |
| `async-recursion` crate | N/A | `#[async_recursion]` generates Box::pin automatically |
