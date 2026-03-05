# Introsort

Hybrid sort: quicksort + heapsort + insertion sort.

## Algorithm
- Start with quicksort
- Switch to heapsort if depth > 2*log(n)
- Use insertion sort for small arrays (<16)

## Complexity
- **Time**: O(n log n) guaranteed
- **Space**: O(log n)
- **Stable**: No

Used by C++ STL std::sort.
