📖 **[View on hightechmind.io →](https://hightechmind.io/rust/370-small-vec-pattern)**

---

# 370: SmallVec Pattern

**Difficulty:** 3  **Level:** Advanced

Store small collections inline on the stack, only allocating when the collection grows beyond a threshold.

## The Problem This Solves

Vec always allocates on the heap. For small, short-lived collections (function arguments, return values, intermediate results), this heap allocation is overhead.

SmallVec stores up to N elements inline (no heap allocation). Only when you push the N+1th element does it allocate and move to the heap. For workloads where most collections are small, this eliminates most allocations.

## How It Works

```rust
enum SmallVec<T, const N: usize> {
    Inline { data: [T; N], len: usize },
    Heap(Vec<T>),
}
```

- `Inline`: Array on the stack, up to N elements
- `Heap`: Regular Vec when overflow occurs

## Use Cases

- Function arguments that are usually 1-3 items
- Temporary working sets
- Graph adjacency lists (most nodes have few neighbors)
- Parser tokens per line
