# PDQsort

PDQsort is Rust's `sort_unstable`. It's a modern quicksort variant that:
- Detects common patterns (sorted, reverse, etc.)
- Uses block partitioning for better cache performance
- Falls back to heapsort when detecting adversarial patterns
