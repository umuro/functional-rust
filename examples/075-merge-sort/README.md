# Example 075: Merge Sort — Functional Divide and Conquer

**Difficulty:** ⭐⭐
**Category:** Sorting Algorithms
**Concept:** Pure functional merge sort: split the input, recursively sort each half, merge the sorted halves. A classic divide-and-conquer algorithm that's naturally expressed with recursion and immutable data.
**OCaml → Rust insight:** OCaml's list cons (`h1 :: merge ...`) builds the result incrementally with structural sharing; Rust's `Vec::push` and `extend_from_slice` allocate contiguously for cache-friendly performance.
