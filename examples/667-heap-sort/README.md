# Heap Sort

Uses binary heap for guaranteed O(n log n) performance with O(1) extra space.

## Complexity
- **Time**: O(n log n) all cases
- **Space**: O(1)
- **Stable**: No

## Properties
- In-place sorting
- Not cache-friendly (poor locality)
- Useful for priority queue operations

## When to Use
- Need guaranteed O(n log n)
- Memory constrained
- Finding k largest/smallest elements
