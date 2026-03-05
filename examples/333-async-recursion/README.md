# 333: Async Recursion

**Difficulty:** 4  **Level:** Expert

Recursive async functions need `Box::pin` — the future's size must be known at compile time.

## The Problem This Solves

A recursive `async fn` produces a state machine struct that contains itself, making its size infinite. The compiler refuses this. The fix is heap-allocation via `Pin<Box<dyn Future<Output=T>>>`.

This pattern shows up for recursive data structure traversal: JSON trees, directory scanning, graph traversal.

## The Intuition

In Python, coroutines are already heap-allocated. Rust stores futures on the stack by default. `Box::pin` opts into heap allocation.

```python
async def async_sum(tree):
    if tree is None: return 0
    return tree.value + await async_sum(tree.left) + await async_sum(tree.right)
```

## How It Works in Rust

```rust
type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

fn async_sum(t: &Tree) -> BoxFuture<'_, i64> {
    Box::pin(async move {
        match t {
            Tree::Leaf => 0,
            Tree::Node { value, left, right } =>
                *value as i64 + async_sum(left).await + async_sum(right).await,
        }
    })
}
```

Why `async move`? The closure captures by reference, bounded by `'_`. The `move` makes ownership explicit.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive async | Direct recursion | Must use `Box::pin(async { ... })` |
| Stack vs heap | Always heap-allocated | Stack by default, `Box` for heap |
| Return type | `'a Lwt.t` | `Pin<Box<dyn Future>>` |
