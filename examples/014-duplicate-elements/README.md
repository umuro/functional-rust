📖 **[View on hightechmind.io →](https://hightechmind.io/rust/014-duplicate-elements)**

---

# Example 014: Duplicate Elements

**Difficulty:** ⭐
**Category:** Lists, Higher-Order Functions
**Concept:** Duplicate every element in a list. A simple but instructive problem showing one-to-many list transformations using flat_map/concat_map and basic recursion.
**OCaml → Rust key insight:** OCaml's `h :: h :: duplicate t` becomes Rust's `flat_map(|x| vec![x.clone(), x.clone()])` — the recursive cons vs iterator collect pattern.
