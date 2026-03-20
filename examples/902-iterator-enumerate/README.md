📖 **[View on hightechmind.io →](https://hightechmind.io/rust/902-iterator-enumerate)**

---

# 902-iterator-enumerate — Iterator Enumerate
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Algorithms that process both element values and their positions — numbering output lines, finding the index of a match, filtering by even/odd positions — need both the index and the value simultaneously. The naive approach uses a separate mutable counter variable, which is error-prone and verbose. Python's `enumerate()` and OCaml's `List.mapi` solve this idiomatically. Rust's `.enumerate()` wraps any iterator to yield `(usize, T)` pairs, composing cleanly with `.filter()`, `.find()`, and `.map()` without mutable counter variables.

## Learning Outcomes

- Use `.enumerate()` to add zero-based indices to any iterator
- Filter by position using `.enumerate().filter(|(i, _)| ...)`
- Find the index of the first matching element using `.enumerate().find(...)`
- Format numbered lists using `.enumerate().map(|(i, s)| format!("{}. {}", i+1, s))`
- Compare with OCaml's `List.mapi` and `Array.iteri`

## Rust Application

`even_indexed` uses `.enumerate().filter(|(i, _)| i % 2 == 0).map(|(_, v)| v)`. `number_items` uses `.enumerate().map(|(i, s)| format!("{}. {}", i+1, s))` for 1-based display. `find_index` uses `.enumerate().find(|(_, v)| pred(v)).map(|(i, _)| i)`. `indexed_filter` combines both index and value in the output: `.enumerate().filter(|(_, v)| pred(v)).collect()`. All are zero-allocation intermediate pipelines — enumerate is a lazy adapter.

## OCaml Approach

`List.mapi: (int -> 'a -> 'b) -> 'a list -> 'b list` maps with index. `List.iteri: (int -> 'a -> unit) -> 'a list -> unit` iterates with index. Finding an index requires `List.find_index` (OCaml 5.1) or manual implementation: `let find_index pred xs = let rec go i = function | [] -> None | x :: rest -> if pred x then Some i else go (i+1) rest in go 0 xs`. Array: `Array.iteri f arr` provides direct indexed access. `List.filteri: (int -> 'a -> bool) -> 'a list -> 'a list` is the filter-by-index equivalent.

## Key Differences

1. **Composition**: Rust `enumerate()` composes with all other adapters; OCaml requires separate `mapi`/`filteri`/`find_index` functions per operation.
2. **Laziness**: Rust `.enumerate()` is lazy; OCaml `List.mapi` is eager and allocates.
3. **find_index**: Rust `.enumerate().find().map(|(i, _)| i)` is idiomatic; OCaml lacked `find_index` until 5.1 — required manual recursion.
4. **1-based display**: Both require `i+1` for human-readable 1-based numbering.

## Exercises

1. Use `.enumerate()` to implement `rotate_left(data: &[i32], n: usize) -> Vec<i32>` that shifts elements left by n positions.
2. Write `positions_of<T: PartialEq>(data: &[T], target: &T) -> Vec<usize>` using `enumerate().filter_map()`.
3. Implement `annotate_changes<T: PartialEq + Clone>(data: &[T]) -> Vec<(usize, &T)>` that returns `(index, value)` only for positions where the value changed from the previous element.
