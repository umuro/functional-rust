# OCaml vs Rust: Async Recursion

## Recursive Sum

**OCaml:**
```ocaml
let rec sum_tree = function
  | Leaf -> 0
  | Node { value; left; right } -> value + sum_tree left + sum_tree right
```

**Rust:**
```rust
fn async_sum(t: &Tree) -> BoxFuture<'_, i64> {
    Box::pin(async move {
        match t {
            Tree::Leaf => 0,
            Tree::Node{value,left,right} =>
                *value as i64 + async_sum(left).await + async_sum(right).await,
        }
    })
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Direct recursion | Yes | No — requires `Box::pin` |
| Async support | Lwt wraps naturally | Must heap-allocate |
| Return type | Simple `int` | `BoxFuture<'_, i64>` |
| Memory | GC handles closures | Explicit heap allocation |
| Crate helper | N/A | `#[async_recursion]` macro |
