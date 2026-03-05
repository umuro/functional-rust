📖 **[View on hightechmind.io →](https://hightechmind.io/rust/367-fenwick-tree)**

---

# 367: Fenwick Tree (Binary Indexed Tree)

**Difficulty:** 4  **Level:** Expert

O(log n) prefix sums and point updates with minimal memory overhead.

## The Problem This Solves

Like segment trees, Fenwick trees support prefix sum queries and point updates in O(log n). But they use much less memory (n elements vs 4n for segment trees) and have lower constant factors.

The tradeoff: Fenwick trees naturally support prefix sums, not arbitrary range queries. Range [l, r] requires two prefix queries: sum(r) - sum(l-1).

## How It Works

The key insight is using binary representation of indices. Each position i stores the sum of elements in a range determined by the lowest set bit of i.

- Position i stores sum of range [i - lowbit(i) + 1, i]
- lowbit(i) = i & (-i)

## Key Operations

| Operation | Complexity |
|-----------|------------|
| Prefix sum | O(log n) |
| Point update | O(log n) |
| Range sum | O(log n) |
| Memory | O(n) |
