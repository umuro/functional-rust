📖 **[View on hightechmind.io →](https://hightechmind.io/rust/054-list-map-from-scratch)**

---

# Example 054: List Map from Scratch

**Difficulty:** ⭐
**Category:** Higher-Order Functions
**OCaml Source:** CS3110

## Problem Statement

Derive `List.map` from scratch to illustrate the **Abstraction Principle**: when two functions share the same shape, factor out the difference into a parameter. `add1`, `double`, and `to_string` all walk a list and transform each element — `map` captures that common pattern.

## Learning Outcomes

- Understand `map` as the canonical higher-order list transformer
- See three equivalent implementations: iterator adapter, head-cons recursion, fold
- Recognise that partially-applied functions (`add1 = map (fun x -> x + 1)`) are first-class in both OCaml and Rust (via closures)
- Contrast OCaml's structural pattern matching on cons cells with Rust's slice patterns

## OCaml Approach

Recursive definition with a cons-cell pattern match. Partial application creates specialised helpers:

```ocaml
let rec map f = function
  | [] -> []
  | h :: t -> let h' = f h in h' :: map f t

let add1 = map (fun x -> x + 1)
```

## Rust Approach

1. **Iterator** (`map`): `list.iter().map(f).collect()` — idiomatic; leverages the standard iterator adapter
2. **Recursive** (`map_recursive`): slice pattern `[head, tail @ ..]` mirrors OCaml's `h :: t`
3. **Fold** (`map_fold`): `fold` accumulates transformed elements, showing `map` as a fold specialisation

## Key Differences

1. **Pattern matching on slices**: Rust uses `[head, tail @ ..]` where OCaml uses `h :: t`; both destructure the head and tail of a list
2. **Partial application**: OCaml's `let add1 = map (fun x -> x + 1)` curries naturally; Rust wraps in a named function or closure instead
3. **Ownership**: Rust's `map` takes `&[A]` (a slice reference) and produces an owned `Vec<B>`, avoiding allocation until `collect()`
4. **Generics vs polymorphism**: OCaml infers `'a list -> 'b list`; Rust requires explicit `<A, B, F: Fn(&A) -> B>`

## Exercises

1. Implement `map_rev` that applies a mapping function to each element and returns the result in reverse order, using a single pass.
2. Write `map_while` from scratch: apply `f: T -> Option<U>` to elements until it returns `None`, then stop — collect only the `Some` values.
3. Implement `map_accumulate` (a scan): like `map` but threads an accumulator through each step, returning both the transformed list and the final accumulator value.
