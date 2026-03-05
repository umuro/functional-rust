# Timsort Comparison

## Standard Library Sorts

- **Python**: Timsort
- **Java**: Timsort (Arrays.sort for objects)
- **Rust**: pdqsort (slice::sort), merge sort variant (slice::sort_stable)
- **OCaml**: Merge sort (List.sort)

## Key Insight
Timsort exploits pre-existing order in data, making it O(n) for nearly sorted input.
