# Example 015: Replicate Elements N Times

**Difficulty:** ⭐
**Category:** Lists, Iteration
**Concept:** Generalization of duplicate: replicate each element n times. Shows the power of `repeat`/`take` combinators and how a simple recursive helper in OCaml becomes an iterator chain in Rust.
**OCaml → Rust key insight:** OCaml's `List.init n (fun _ -> x)` becomes Rust's `std::iter::repeat(x).take(n)` — both create n copies of a value lazily.
