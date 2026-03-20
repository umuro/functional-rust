📖 **[View on hightechmind.io →](https://hightechmind.io/rust/221-apomorphism)**

---

# Apomorphism — Ana that Can Short-Circuit
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

An anamorphism unfolds a seed step by step until the base case. But some unfolding algorithms know partway through that the rest of the structure is already built — they want to "embed" an existing structure rather than continuing to unfold. Apomorphisms extend anamorphisms with this short-circuit capability: the coalgebra can return either a new seed (continue unfolding) or an existing `Fix<F>` value (embed directly). This is the dual of a paramorphism.

## Learning Outcomes

- Understand apomorphisms as anamorphisms with early termination (embedding existing structures)
- Learn how `Either<Fix<F>, S>` in the coalgebra return type enables short-circuiting
- See list insertion as the canonical example: insert an element, then embed the tail
- Understand the duality: `apo` is to `ana` as `para` is to `cata`

## Rust Application

`apo<S>(coalg: impl Fn(S) -> ListF<Either<FixList, S>>) -> impl Fn(S) -> FixList`. The coalgebra returns `Either<FixList, S>` for children: `Left(existing_list)` embeds the existing structure directly, `Right(new_seed)` continues unfolding. Insertion into a sorted list: `insert(x, list)` — if `x <= head`, return `ConsF(x, Left(list))` (prepend `x` and embed the rest); if `x > head`, return `ConsF(head, Right((x, tail)))` (keep `head`, continue inserting into tail).

## OCaml Approach

OCaml's apomorphism:
```ocaml
let rec apo coalg seed =
  Fix (map_list_f (function
    | Left existing -> existing
    | Right new_seed -> apo coalg new_seed)
  (coalg seed))
```
The `Either` (called `result` or a custom type in OCaml) is the mechanism for short-circuiting. OCaml's pattern matching on `Left`/`Right` is direct and readable.

## Key Differences

1. **Short-circuit power**: `apo` terminates early by embedding existing structures; `ana` must unfold everything from scratch — `apo` is strictly more expressive.
2. **Duality with para**: `apo : Seed -> Fix` mirrors `para : Fix -> Result` dually; the "embed" vs. "expose" distinction is the symmetric difference.
3. **Practical use**: Sorted list insertion, search tree insertion with path copying, and "insert and replace" operations in functional structures are natural apomorphisms.
4. **Either in coalgebra**: The `Either<Fix<F>, S>` return type is the key innovation; `Right(seed)` continues, `Left(fix_value)` short-circuits.

## Exercises

1. Implement sorted insertion for a `Vec<i64>` using `apo` and verify the result is sorted.
2. Write `apo_take(n, seed)` that generates at most n elements from an infinite coalgebra by short-circuiting after n steps.
3. Implement `replace_first(pred, new_val, list)` using `apo`: replace the first element satisfying `pred`, then embed the rest unchanged.
