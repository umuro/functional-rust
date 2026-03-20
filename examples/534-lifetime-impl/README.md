📖 **[View on hightechmind.io →](https://hightechmind.io/rust/534-lifetime-impl)**

---

# Lifetimes in impl Blocks
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

When a struct has a lifetime parameter, every `impl` block for that struct must repeat the lifetime parameter and can use it in method signatures. The critical subtlety is that methods can return references with either the struct's lifetime (`'a`) or the lifetime of `&self` — and these are different. A method returning `&'a T` can return data that outlives the method call; a method returning `&T` tied to `&self` is only valid for the duration of the method borrow. Understanding this distinction prevents common confusion when implementing view-type APIs.

## Learning Outcomes

- How `impl<'a, T> View<'a, T>` propagates the struct's lifetime into method signatures
- Why `fn get(&self, index: usize) -> Option<&'a T>` returns data with the stored lifetime, not `self`'s
- How `fn slice(&self, ...) -> Option<View<'a, T>>` creates a sub-view with the same parent lifetime
- The difference between returning `&'a T` vs `&T` (where `T` is tied to `self`)
- Where this pattern is used: slice wrappers, buffer views, database row references

## Rust Application

`View<'a, T>` stores `data: &'a [T]`. The `get` method returns `Option<&'a T>` — not `Option<&T>` — because the data outlives the `View` struct itself; the reference is valid as long as the original slice is valid. `slice` similarly returns `Option<View<'a, T>>` — a sub-view with the same `'a` lifetime. This is the key insight: `self.data.get(index)` returns `Option<&'a T>` because `self.data` is already `&'a [T]`.

Key patterns:
- `impl<'a, T> View<'a, T>` — repeating `'a` in the `impl` header
- `fn get(&self, i: usize) -> Option<&'a T>` — returning the stored lifetime, not `&self`'s
- `fn slice(&self, start, end) -> Option<View<'a, T>>` — sub-view preserving the parent lifetime

## OCaml Approach

OCaml modules implementing view-like abstractions use plain records and return values without lifetime annotations. The GC ensures the referenced data remains alive:

```ocaml
type 'a view = { data: 'a array; start: int; len: int }
let get v i = if i < v.len then Some v.data.(v.start + i) else None
let slice v s e = if s <= e && e <= v.len then Some { v with start = v.start + s; len = e - s } else None
```

## Key Differences

1. **Return lifetime source**: Rust methods must distinguish whether a returned reference comes from `self` or from the stored `'a` data — different lifetimes with different scopes; OCaml has no such distinction.
2. **Sub-view lifetime**: Rust `slice` returns `View<'a, T>` with the same lifetime as the original — no copy made; OCaml slice creates a new record pointing into the same array, relying on GC for safety.
3. **Method signature verbosity**: Rust `impl<'a, T>` methods often repeat `'a` in return types; OCaml methods on parameterized types (`'a view`) are simpler to write.
4. **Safety model**: Rust's `View<'a, T>` statically prevents accessing data after the source slice is freed; OCaml's GC prevents it dynamically — the array is kept alive as long as any view references it.

## Exercises

1. **Mutable view**: Implement `struct ViewMut<'a, T> { data: &'a mut [T] }` with `fn get_mut(&mut self, i: usize) -> Option<&mut T>` — note the lifetime difference from the immutable version.
2. **Chained slices**: Write a method `fn chunks(&self, size: usize) -> Vec<View<'a, T>>` that splits the view into equal-sized sub-views without copying data.
3. **View iterator**: Implement `struct ViewIter<'a, T> { view: &'a View<'a, T>, pos: usize }` as an `Iterator<Item = &'a T>` that yields elements from the view.
