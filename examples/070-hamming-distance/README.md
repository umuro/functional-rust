📖 **[View on hightechmind.io →](https://hightechmind.io/rust/070-hamming-distance)**

---

# Example 070: Hamming Distance

**Difficulty:** ⭐
**Category:** String Processing
**Concept:** Counting positional differences between two equal-length strings using zip and filter. A clean demonstration of pairwise character comparison as a higher-order function pipeline.
**OCaml → Rust insight:** OCaml needs `List.combine` to zip two lists; Rust's `Iterator::zip` works lazily on any iterator — including `chars()` on strings — without intermediate allocation.
