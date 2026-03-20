📖 **[View on hightechmind.io →](https://hightechmind.io/rust/914-iterator-min-max)**

---

# 914-iterator-min-max — Iterator min and max

## Problem Statement

Finding the extreme value in a sequence is ubiquitous: highest score, earliest date, longest string, coldest temperature. The standard approach traverses the sequence once while tracking the running extreme. Rust's `Iterator::min()` and `Iterator::max()` encapsulate this idiom, requiring `Ord` for direct comparison. For types with partial ordering (like `f64`, which has NaN), `min_by` and `max_by` accept a custom comparator. `min_by_key` and `max_by_key` extract a sort key from each element. All return `Option<T>` — returning `None` for empty iterators rather than panicking.

## Learning Outcomes

- Use `.min()` and `.max()` for total-order types (`Ord`)
- Use `.min_by_key()` and `.max_by_key()` to find extremes by a derived key
- Handle `f64` using `.reduce(f64::min)` since `f64` is not `Ord`
- Apply min/max to structs by key extraction (top student, shortest word)
- Compare with OCaml's `List.fold_left max` and `Base.List.min_elt`

## Rust Application

`slice_min` and `slice_max` use `nums.iter().copied().min()` / `.max()`. `shortest` and `longest` use `words.iter().copied().min_by_key(|w| w.len())`. `top_student` and `bottom_student` use `students.iter().max_by_key(|s| s.score)` on a struct slice. For `f64`: `.iter().copied().reduce(f64::min)` avoids the `Ord` requirement since `f64::min` handles NaN by returning the other value. The `Student` struct examples show practical use in domain modeling.

## OCaml Approach

OCaml uses `List.fold_left max x xs` where `max` is the built-in polymorphic comparison. `Base.List.min_elt ~compare` and `Base.List.max_elt ~compare` are the idiomatic equivalents with an explicit comparator. Standard OCaml: `let max_student students = List.fold_left (fun acc s -> if s.score > acc.score then s else acc) (List.hd students) (List.tl students)`. OCaml's built-in `compare` works on all types but is untyped — `Ord` in Rust provides a type-safe alternative.

## Key Differences

1. **Type safety**: Rust requires `Ord` for min/max — provides a compile-time guarantee; OCaml's polymorphic `max` works on any type but may compare incorrectly.
2. **f64 handling**: Rust `f64` doesn't implement `Ord` (due to NaN); OCaml's polymorphic compare handles NaN with platform-defined behavior.
3. **Option return**: Rust min/max return `Option<T>` for empty input; OCaml `List.fold_left` requires a non-empty list or explicit check.
4. **key extraction**: `min_by_key` / `max_by_key` are first-class; OCaml requires `min_elt ~compare:(fun a b -> compare (key a) (key b))`.

## Exercises

1. Write `top_n<T: Ord + Clone>(data: &[T], n: usize) -> Vec<T>` that returns the n largest elements without sorting the whole slice.
2. Implement `argmin<T: PartialOrd>(data: &[T]) -> Option<usize>` using `enumerate().min_by(...)` that returns the index of the minimum.
3. Find the word with the most distinct characters in a list of strings using `max_by_key`.
