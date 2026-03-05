# OCaml vs Rust: Radix Sort

Both implementations use similar LSD (least significant digit) approach.

## Key Steps
1. Find maximum value for digit count
2. For each digit position (1, 10, 100, ...):
   - Count occurrences
   - Build cumulative count
   - Place elements in output
3. Copy back

## Complexity
Both achieve O(d * n) where d is the number of digits.

## Key Difference
Rust uses slices and copy_from_slice.
OCaml uses Array.blit for in-place update.
