📖 **[View on hightechmind.io →](https://hightechmind.io/rust/179-gadt-safety)**

---

# GADT Preventing Runtime Errors — Safe Head
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

`head` on an empty list is undefined. Instead of returning `Option<T>` and forcing callers to handle `None`, the typestate approach encodes emptiness in the type: `SafeList<T, NonEmpty>` has a `head` method; `SafeList<T, Empty>` does not. This converts a potential runtime panic into a compile error, and callers never need to unwrap. The pattern generalizes to any operation that requires a precondition: safe division, safe array access, safe database reads.

## Learning Outcomes

- Encode list emptiness as a phantom type state (`Empty` vs. `NonEmpty`)
- Implement `head` only on `SafeList<T, NonEmpty>` — calling it on an empty list is a compile error
- See how `push` transitions from `SafeList<T, Empty>` to `SafeList<T, NonEmpty>`
- Understand this as a specialized typestate pattern (example 130) applied to data structures

## Rust Application

`Empty` and `NonEmpty` are zero-sized marker structs. `SafeList<T, S>` wraps `Vec<T>` with `PhantomData<S>`. `SafeList<T, Empty>::new() -> SafeList<T, Empty>` creates an empty list. `push(self, val: T) -> SafeList<T, NonEmpty>` transitions to non-empty. `head(&self) -> &T` is implemented only on `SafeList<T, NonEmpty>` — the compiler rejects calling `head` on a `SafeList<T, Empty>`. This is zero-overhead: `PhantomData` has no runtime size.

## OCaml Approach

OCaml's GADT approach is more elegant:
```ocaml
type empty = Empty
type nonempty = Nonempty
type ('a, 's) safe_list =
  | Nil  : ('a, empty) safe_list
  | Cons : 'a * ('a, _) safe_list -> ('a, nonempty) safe_list
let head : ('a, nonempty) safe_list -> 'a = function Cons (x, _) -> x
```
Pattern matching on `Nil` in `head` is rejected by the compiler — OCaml's GADT exhaustiveness checker knows `nonempty` lists are never `Nil`.

## Key Differences

1. **Exhaustiveness**: OCaml's GADT `head` on `nonempty` lists has no `Nil` case — it's impossible; Rust's `impl SafeList<T, NonEmpty>` block achieves the same by not defining the method on `Empty`.
2. **Transition type**: OCaml's `Cons` constructor changes the type index directly; Rust's `push` consumes the old value and returns a new type.
3. **Runtime cost**: Both are zero-cost — `PhantomData`/phantom types vanish at runtime.
4. **Practical use**: Rust's typestate is used in the `rusqlite` crate (transaction states), `tokio` (task states), and custom protocol implementations.

## Exercises

1. Add a `tail(self) -> (T, SafeList<T, ...>)` method that returns a `SafeList<T, NonEmpty>` if the original had 2+ elements, or `SafeList<T, Empty>` if exactly 1.
2. Implement `first_two(&self) -> (&T, &T)` only on lists with at least two elements — requiring a `TwoOrMore` phantom state.
3. Implement a `safe_dequeue` for a `SafeQueue` type with `Enqueue` and `Dequeue` operations that prevent dequeuing from an empty queue.
