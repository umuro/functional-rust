# Quick Sort

Efficient divide-and-conquer sort with excellent cache performance.

## Complexity
- **Time**: O(n log n) average, O(n²) worst
- **Space**: O(log n) stack
- **Stable**: No

## Variants
1. **Lomuto**: Simple partition
2. **Hoare**: Original, fewer swaps
3. **Median-of-3**: Better pivot selection
4. **3-way**: Handles duplicates well

## When to Use
- General purpose sorting
- Cache-friendly (in-place)
- When stability not required
