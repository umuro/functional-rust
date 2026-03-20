📖 **[View on hightechmind.io →](https://hightechmind.io/rust/145-gat-collections)**

---

# GAT Collections
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A truly generic collection trait in Rust — one where `map` changes the element type — requires GATs. Without them, you cannot express "a `Vec<T>` mapped by `T -> U` produces `Vec<U>`" in a trait that also applies to `HashMap<K, V>`, `BTreeSet<T>`, and custom containers. GAT collections resolve this by making the container's output type generic over the element type, enabling unified generic algorithms over diverse collection types.

## Learning Outcomes

- Learn how GATs enable `type Mapped<U>` in collection traits, changing element types through map
- Understand the difference between a homogeneous collection trait and a GAT-based one
- See how `Functor` for collections is expressed using GATs in Rust
- Practice implementing GAT-based traits for `Vec<T>` and `Option<T>`

## Rust Application

```rust
trait Collection {
    type Item;
    type Mapped<U>;
    fn map_col<U>(self, f: impl Fn(Self::Item) -> U) -> Self::Mapped<U>;
    fn filter_col(self, f: impl Fn(&Self::Item) -> bool) -> Self;
}
```
Implementing this for `Vec<T>`: `type Item = T; type Mapped<U> = Vec<U>; fn map_col<U>(self, f: ...) -> Vec<U> { self.into_iter().map(f).collect() }`. Without the GAT `type Mapped<U>`, the return type of `map_col` could not change the element type while remaining associated with `Self`.

## OCaml Approach

OCaml expresses this naturally:
```ocaml
module type COLLECTION = sig
  type 'a t
  val map : ('a -> 'b) -> 'a t -> 'b t
  val filter : ('a -> bool) -> 'a t -> 'a t
end
module ListCollection : COLLECTION with type 'a t = 'a list = struct ... end
```
The parameterized type `'a t` is OCaml's built-in equivalent of Rust's GAT `type Mapped<U>`. This pattern is fundamental to OCaml's standard library design.

## Key Differences

1. **Native vs. GAT**: OCaml's `'a t` is a native parameterized type constructor; Rust requires the GAT feature to express the same concept in traits.
2. **Verbosity**: OCaml's `COLLECTION` signature is terse; Rust's equivalent requires explicit `type Item`, `type Mapped<U>`, and `where` bounds.
3. **Stability**: OCaml's pattern is decades old; Rust's GATs stabilized in 1.65 and still have some rough edges around lifetime inference.
4. **Key-value containers**: Extending to `HashMap<K, V>` requires handling two type parameters; GATs handle this with `type Mapped<K2, V2>`.

## Exercises

1. Implement `Collection` for `Option<T>` with `type Mapped<U> = Option<U>`.
2. Add a `flat_map_col<U>(self, f: impl Fn(Self::Item) -> Self::Mapped<U>) -> Self::Mapped<U>` method to the trait.
3. Write a generic `double_all<C: Collection<Item = i32>>(c: C) -> C::Mapped<i32>` that doubles every element.
