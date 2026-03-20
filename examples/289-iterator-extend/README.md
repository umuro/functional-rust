📖 **[View on hightechmind.io →](https://hightechmind.io/rust/289-iterator-extend)**

---

# 289: Extending Collections with extend()

## Problem Statement

Building up a collection incrementally from multiple sources — appending new items to an existing `Vec`, merging two `HashMap`s, adding elements from a computation to an existing set — is a fundamental operation in accumulative algorithms. The `extend()` method is the mutable counterpart to `collect()`: it appends elements from any `IntoIterator` to an existing collection in place, avoiding the need to create intermediate temporary collections.

## Learning Outcomes

- Understand `extend()` as the in-place append operation for any `Extend<T>` collection
- Use `extend()` to merge multiple sources into a single pre-existing collection
- Recognize that `extend` on a `Vec` is equivalent to `append` but accepts any iterator
- Combine `extend()` with filtered or transformed iterators for selective merging

## Rust Application

`Extend::extend(iter)` appends elements from `iter` to `self`. It is implemented by `Vec`, `String`, `HashMap`, `HashSet`, and other standard collections:

```rust
let mut base = vec![1, 2, 3];
base.extend([4, 5, 6]);
base.extend(7..=9);
// base = [1, 2, 3, 4, 5, 6, 7, 8, 9]

// String extend from chars
let mut s = String::from("hello");
s.extend(" world".chars());

// HashMap extend merges entries (later values overwrite earlier)
let mut map: HashMap<&str, i32> = HashMap::new();
map.extend([("a", 1), ("b", 2)]);
map.extend([("b", 99), ("c", 3)]); // "b" -> 99 now
```

## OCaml Approach

OCaml's `List.rev_append` and `@` operator combine lists, but these create new lists rather than mutating in place. For mutable structures, `Buffer.add_string` (for strings) and `Hashtbl.add_seq` provide equivalent in-place extension:

```ocaml
(* Functional: create new list combining both *)
let combined = base @ [4; 5; 6]

(* Mutable: Hashtbl extend from sequence *)
Hashtbl.add_seq tbl (List.to_seq [("b", 2); ("c", 3)])
```

## Key Differences

1. **Mutability**: Rust's `extend()` mutates the collection in place, consuming the iterator; OCaml's `@` creates a new list.
2. **Conflict resolution**: HashMap `extend` silently overwrites on key collision; explicit merge logic is needed if different behavior is desired.
3. **Allocation efficiency**: `extend()` on a `Vec` reserves space using `size_hint()` to minimize reallocations; OCaml's `@` always allocates.
4. **Build pattern**: The common Rust pattern builds a `Vec` with `with_capacity()` then uses `extend()` in a loop — one allocation, multiple appends.

## Exercises

1. Build a word frequency map by `extend()`-ing a `HashMap<String, usize>` from multiple document iterators, combining counts correctly.
2. Use `extend()` to merge sorted chunks back into a single sorted `Vec` (k-way merge via repeated extend + sort, demonstrating the efficiency concern).
3. Implement a `StringBuilder` wrapper around `String` that provides an `append(iter)` method using `extend()`, chaining multiple sources.
