# Merge Sort — Functional Divide and Conquer: OCaml vs Rust

## The Core Insight
Merge sort is the quintessential functional sorting algorithm because it's naturally recursive and doesn't require mutation. The comparison reveals how OCaml's linked lists and Rust's contiguous Vecs lead to different implementation strategies with the same O(n log n) complexity but different constant factors.

## OCaml Approach
OCaml's merge sort is textbook elegant. `split` alternates elements into two lists using pattern matching on pairs (`a :: b :: rest`). `merge` recursively cons-es the smaller head onto the merged tail. The `compare` function is passed as a first-class value for custom ordering. All intermediate lists share structure through the GC — no copying needed. The `([] | [_]) as l -> l` or-pattern cleanly handles base cases.

## Rust Approach
Rust uses slices (`&[T]`) for input and `Vec<T>` for output. Splitting is trivial with slice indexing: `&list[..mid]` and `&list[mid..]`. Merging uses index-based iteration with `push` and `extend_from_slice` for efficient bulk copying. `Vec::with_capacity` pre-allocates the exact needed size, avoiding reallocation. The `Ord` trait bound replaces OCaml's `compare` function parameter. A peekable iterator variant shows the more functional style.

## Side-by-Side
| Concept | OCaml | Rust |
|---------|-------|------|
| Split | Pattern match `a :: b :: rest` | Slice indexing `&list[..mid]` |
| Merge | Recursive cons `h1 :: merge ...` | `Vec::push` + `extend_from_slice` |
| Base case | `([] \| [_]) as l -> l` | `if list.len() <= 1` |
| Comparator | `cmp` function parameter | `Ord` trait bound or closure |
| Memory | GC, structural sharing | `Vec` allocation, contiguous |
| Stability | Stable (preserves order) | Stable (preserves order) |

## What Rust Learners Should Notice
- `Vec::with_capacity(n)` is a crucial optimization — it avoids O(log n) reallocations during the merge phase
- Slice references (`&[T]`) give you zero-cost views into data without copying — similar to how OCaml lists share tails
- `extend_from_slice` copies a contiguous block efficiently, much faster than pushing elements one by one
- The `Ord` trait bound means any type that implements comparison can be sorted — no need to pass a comparator function (though `merge_sort_by` with a closure also works)
- In production Rust, use `slice::sort()` or `slice::sort_unstable()` — they use Timsort / pdqsort respectively, both highly optimized

## Further Reading
- [The Rust Book — Generics and Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [OCaml List Sorting](https://cs3110.github.io/textbook/chapters/ds/bst.html)
