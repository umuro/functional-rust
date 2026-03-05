# Insertion Sort

Builds sorted array one element at a time by inserting each element in its correct position.

## Complexity

- **Time**: O(n²) average/worst, O(n) best
- **Space**: O(1)
- **Stable**: Yes

## Variants

1. **Standard**: Swap-based insertion
2. **Binary**: Binary search for position
3. **Shell**: Gap-based generalization

## When to Use

- Small arrays (< 50 elements)
- Nearly sorted data
- Online sorting (streaming data)
- As the base case for hybrid sorts
