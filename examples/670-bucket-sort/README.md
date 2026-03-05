# Bucket Sort

Distributes elements into buckets, sorts each bucket, concatenates.

## Complexity
- **Time**: O(n + k) average, O(n²) worst
- **Space**: O(n + k)
- **Stable**: Yes (with stable bucket sort)

## When to Use
- Uniformly distributed data
- Floating point numbers
- Known range of values
- Parallelizable (each bucket independent)
