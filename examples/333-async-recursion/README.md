📖 **[View on hightechmind.io →](https://hightechmind.io/rust/333-async-recursion)**

---

# 333: Async Recursion
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Recursive async functions have a fundamental problem: the compiler needs to know the future's size at compile time to allocate it on the stack. A recursive async function has an infinitely-nested type (each recursive call adds another layer of future type). The solution is `Box::pin()` — heap-allocating the future and using a type alias `BoxFuture<'a, T>` to break the recursive type definition. This is the standard pattern for tree traversals and parser combinators in async Rust.

## Learning Outcomes

- Understand why recursive `async fn` requires `Box::pin` — infinite type recursion
- Use `BoxFuture<'a, T>` = `Pin<Box<dyn Future<Output = T> + 'a>>` as the return type
- Implement recursive async tree operations using `Box::pin(async move { ... })`
- Recognize the performance cost: each recursive call allocates a heap-pinned future

## Rust Application

Recursive tree operations wrapped in `Box::pin`:

```rust
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

pub fn sum_tree<'a>(tree: &'a Tree<i32>) -> BoxFuture<'a, i32> {
    Box::pin(async move {
        match tree {
            Tree::Leaf => 0,
            Tree::Node { value, left, right } => {
                let left_sum = sum_tree(left).await;
                let right_sum = sum_tree(right).await;
                value + left_sum + right_sum
            }
        }
    })
}
```

## OCaml Approach

OCaml's Lwt recursive functions don't require boxing — closures are always heap-allocated:

```ocaml
let rec sum_tree = function
  | Leaf -> Lwt.return 0
  | Node { value; left; right } ->
    let* l = sum_tree left in
    let* r = sum_tree right in
    Lwt.return (value + l + r)
```

OCaml closures naturally handle recursive types without explicit boxing.

## Key Differences

1. **Explicit boxing**: Rust requires explicit `Box::pin` for recursive futures; OCaml's closures are always on the heap.
2. **async-recursion crate**: The `async-recursion` crate provides `#[async_recursion]` macro that generates the `Box::pin` boilerplate automatically.
3. **Allocation cost**: Each recursive level allocates a heap future; for very deep recursion, this adds GC pressure (Rust) or risk of stack overflow (OCaml).
4. **Pattern**: `BoxFuture<'a, T>` is the standard type alias for any dynamically-dispatched future — used throughout `async-trait` and similar crates.

## Exercises

1. Implement recursive async JSON traversal: count all leaf nodes in a nested `JsonValue` structure using `BoxFuture`.
2. Use the `async-recursion` crate's `#[async_recursion]` macro and compare the generated code to the manual `BoxFuture` approach.
3. Implement a recursive descent parser using async functions, where each production rule is a separate async function.
