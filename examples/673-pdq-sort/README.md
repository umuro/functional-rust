# PDQsort

Pattern-Defeating Quicksort - Rust's default unstable sort.

## Features
- Detects sorted/reverse patterns
- Block partitioning for cache efficiency
- Falls back to heapsort for bad cases

## Complexity
- **Time**: O(n log n) average
- **Space**: O(log n)
- **Stable**: No

## Usage in Rust
```rust
arr.sort_unstable(); // Uses pdqsort
```
