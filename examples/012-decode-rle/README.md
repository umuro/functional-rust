📖 **[View on hightechmind.io →](https://hightechmind.io/rust/012-decode-rle)**

---

# Example 012: Decode Run-Length Encoding

**Difficulty:** ⭐⭐
**Category:** Lists, Pattern Matching
**Concept:** The inverse of RLE encoding: expand a compressed representation back into the original list. Demonstrates flat_map/concat_map as the natural tool for one-to-many transformations.
**OCaml → Rust key insight:** OCaml's `List.concat_map` is Rust's `.flat_map()` — both expand each element into zero or more elements and flatten the result.
