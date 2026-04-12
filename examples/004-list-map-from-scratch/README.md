# Example 004: List Map From Scratch
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement the `map` function—a higher-order function that applies a transformation function to each element of a list. Demonstrate how abstracting this common pattern (apply f to each element) enables partial application: binding `map` with specific functions creates specialized transformers.

Building `map` from scratch is the canonical first exercise in functional programming because it forces you to understand the abstraction you are hiding. Every senior developer should know that `for (item of items) result.push(f(item))` is just `items.map(f)`. Making this abstraction a first-class value — a function that takes other functions — is what enables predicate factories, transformer pipelines, and the entire functional style.

## Learning Outcomes

- How to abstract repeated functional patterns into reusable higher-order functions
- How Rust's closure trait bounds (`Fn`) parallel OCaml's function types
- The difference between idiomatic Rust (iterator chains) and functional/recursive style
- How partial application in Rust (via closures) differs from OCaml's implicit currying
- Why `Copy` is required when working with borrowed slices, and when to use `&[T]` vs `Vec<T>`

## OCaml Approach

OCaml's `map` is recursive and explicit: match on the list structure (empty or head::tail), apply the function to the head, and recursively process the tail. OCaml's automatic currying makes partial application effortless—`let add1 = map (fun x -> x + 1)` creates a specialized transformer function without extra syntax.

## Rust Approach

Idiomatic Rust leverages iterators: `.iter().map(f).collect()` is the standard, efficient way to apply a transformation. For teaching the abstraction principle, we also show the explicit recursive version. Rust closures capture their environment, making partial application natural: `|x| x + 1` is a closure, and binding `map(|x| x + 1, items)` creates the transformation inline.

## Key Differences

1. **List vs Slice representation:** OCaml uses a linked list `'a list`; Rust uses slices `&[T]` for borrowed views of contiguous data. Slices are cheaper to work with but require `Copy` bounds for element operations.

2. **Recursion style:** OCaml pattern matches on the list structure directly; Rust's recursive version uses slice pattern matching `[head, rest @ ..]` to deconstruct and recurse.

3. **Partial application:** OCaml's implicit currying means `let add1 = map (fun x -> x + 1)` automatically returns a function. Rust requires explicit closure syntax: `|x| x + 1`, and capturing the function in a closure returned by a wrapper function.

4. **Standard library idiom:** OCaml learners use `List.map`; Rust learners typically use the iterator `map` method. The explicit implementation teaches the abstraction, but production code uses iterators.

5. **Function composition:** OCaml easily composes `map` with other list functions via piping; Rust chains methods or uses closure composition, reflecting different language paradigms.

1. **List vs Slice representation:** OCaml uses a linked list `'a list`; Rust uses slices `&[T]` for borrowed views of contiguous data. Slices are cheaper to work with but require `Copy` bounds for element operations.
2. **Automatic currying:** OCaml's `map f` partially applies to create a transformer. Rust needs an explicit closure: `|items| map_idiomatic(f, items)`.
3. **Allocation per call:** The recursive Rust version allocates an intermediate `Vec` per recursive frame. The iterator version collects once at the end.
4. **Trait bounds:** Rust requires `T: Copy` when extracting values from a borrowed slice by value. OCaml's GC handles ownership transparently — no bounds needed.
5. **Function type syntax:** OCaml: `'a -> 'b`. Rust: `impl Fn(T) -> U` for closures or `fn(T) -> U` for function pointers.

## Exercises

1. Implement `map_index` — a variant of `map` that passes both the element and its index to the mapping function.
2. Write `flat_map` from scratch: given a list and a function `f: T -> Vec<U>`, return a flattened `Vec<U>` without using `.flat_map()`.
3. Implement `map_result` that applies a fallible function `f: T -> Result<U, E>` to each element, returning `Ok(Vec<U>)` if all succeed or the first `Err`.
