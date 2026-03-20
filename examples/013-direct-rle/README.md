📖 **[View on hightechmind.io →](https://hightechmind.io/rust/013-direct-rle)**

---

# 013 — Direct Run-Length Encoding

## Problem Statement

OCaml 99 Problems #13 challenges you to implement run-length encoding directly — without first packing runs into sublists (as in example 010) and then counting them. This single-pass approach is more efficient because it avoids intermediate allocations and processes each element exactly once.

The direct approach teaches an important programming pattern: maintaining a small amount of state (current element, current count) as you scan through a sequence, emitting output at each run boundary. This is the basis for streaming compression algorithms, tokenizers, lexers, and parser front-ends. Anything that groups consecutive similar items uses this pattern.

## Learning Outcomes

- Implement a stateful single-pass scan using a while loop and index tracking
- Use the "current run" pattern: track `(current_element, count)` and emit at boundaries
- Compare single-pass direct encoding with two-pass (pack then count) approaches
- Understand when single-pass algorithms are preferable to multi-pass
- Apply the `RleItem` enum from example 011 as a shared output type

## Rust Application

`encode_direct` uses a while loop with two pointers: `start` marks the beginning of each run, `i` advances until it finds an element different from `list[start]`. The run length is `i - start`. This is O(n) time, O(1) extra space (beyond the output). The recursive `encode_direct_recursive` uses pattern matching on `split_first()` and an accumulator, showing how the same algorithm can be expressed recursively. Both produce identical output to the modified encoder from example 011.

## OCaml Approach

The direct OCaml version is: `let rec encode lst = match lst with | [] -> [] | x :: rest -> let rec count_run acc = function | y :: t when y = x -> count_run (acc + 1) t | rest -> (acc, rest) in let (n, remaining) = count_run 1 rest in let item = if n = 1 then One x else Many (n, x) in item :: encode remaining`. This uses a nested helper `count_run` that advances through matching elements, making the structure explicitly recursive.

## Key Differences

1. **Two-pointer vs nested helper**: Rust uses an index-based two-pointer approach (idiomatic for `Vec`/slice). OCaml uses a nested recursive `count_run` function (idiomatic for linked lists).
2. **Slice vs list navigation**: Rust random-accesses `list[i]` and `list[start]` in O(1). OCaml's list traversal is sequential — random access is O(n), so index-based approaches are avoided.
3. **Boundary detection**: Rust checks `list[i] != list[start]` to detect run end. OCaml's `when y = x` guard in the match arm advances while equal.
4. **Output accumulation**: Rust pushes to a `Vec` (amortized O(1)). OCaml builds a cons list in reverse and reverses at the end, or builds forward using a tail-recursive accumulator.

## Exercises

1. **Encode bytes**: Adapt `encode_direct` to work on `&[u8]` for binary data encoding. Benchmark it against the version from example 011 on a large repeated-byte input.
2. **Maximum run**: Write `longest_run<T: PartialEq>(list: &[T]) -> Option<(usize, &T)>` that returns the count and value of the longest consecutive run in a single pass.
3. **RLE streaming encoder**: Implement `RleEncoder<T>` as a struct that accepts elements one at a time via `push(&mut self, x: T)` and emits complete `RleItem<T>` values when a run ends (like a streaming codec).
