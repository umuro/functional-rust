# Radix Sort

Non-comparison sort processing digits/characters.

## Variants
- **LSD**: Least significant digit first (bottom-up)
- **MSD**: Most significant digit first (top-down)

## Complexity
- **Time**: O(d * (n + k)) 
- **Space**: O(n + k)
- **Stable**: Yes (LSD)

## When to Use
- Fixed-length integers
- Strings with common prefixes
- Large datasets with small key range
