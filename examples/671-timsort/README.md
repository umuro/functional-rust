# Timsort

Hybrid stable sort used by Python and Java.

## Algorithm
1. Divide into runs of MIN_RUN size
2. Sort each run with insertion sort
3. Merge runs with merge sort

## Complexity
- **Time**: O(n log n) worst, O(n) best
- **Space**: O(n)
- **Stable**: Yes

## Optimizations
- Natural run detection
- Galloping merge for skewed distributions
