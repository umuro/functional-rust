📖 **[View on hightechmind.io →](https://hightechmind.io/rust/149-extension-traits)**

---

# Extension Traits

## Problem Statement

Rust's orphan rule prevents implementing an external trait on an external type — you cannot add `Display` to `Vec<T>` if both come from other crates. Extension traits solve this by defining a new trait with the desired methods and implementing it for the external type — since the new trait is yours, the orphan rule permits it. This pattern is widely used in Rust libraries to add methods to standard types (`IteratorExt`, `ReadExt`, `FutureExt`).

## Learning Outcomes

- Understand the orphan rule and why it exists (coherence guarantee)
- Learn the extension trait pattern: a new trait adding methods to an existing type
- See how `use MyExt` in scope brings the new methods into visibility
- Recognize extension traits in the Rust ecosystem: `futures::StreamExt`, `itertools::Itertools`

## Rust Application

To add methods to `Iterator` beyond those in `std`:
```rust
pub trait IteratorExt: Iterator {
    fn collect_string(self) -> String where Self::Item: Display;
    fn sum_or_default(self) -> Self::Item where Self::Item: Default + Add<Output = Self::Item>;
}
impl<I: Iterator> IteratorExt for I { ... }
```
The `I: Iterator` bound means every iterator automatically gets the extension methods. Callers import `use my_crate::IteratorExt` to bring the methods into scope. This is identical in spirit to Kotlin's extension functions and C#'s extension methods.

## OCaml Approach

OCaml achieves method extension through modules. To add functions to a library type:
```ocaml
module ListExt = struct
  include List  (* bring existing functions into scope *)
  let sum = List.fold_left (+) 0
  let chunk n lst = ...
end
```
There is no concept of method dispatch in OCaml's standard library (no `obj.method()` syntax), so extensions are just functions in a module that call the original library functions. Open dispatch with polymorphic variants provides more dynamic alternatives.

## Key Differences

1. **Method dispatch**: Rust's extension traits enable `iter.my_method()` dot-notation; OCaml's extensions are plain functions (`ListExt.my_function lst`).
2. **Orphan rule**: Rust's orphan rule requires the trait or the type to be local; OCaml modules have no such restriction — you can add functions to any type.
3. **Visibility**: Rust extension methods require importing the trait (`use MyExt`); OCaml module functions are always accessible via `Module.f`.
4. **Ecosystem pattern**: `itertools`, `futures`, `tokio` heavily use extension traits; OCaml's `Base`, `Core` add functions to `List`, `String` etc. via module inclusion.

## Exercises

1. Write `StringExt` with methods `truncate_at(n: usize) -> String` and `word_count() -> usize` for `&str`.
2. Add `tally<K: Hash + Eq>(self) -> HashMap<K, usize>` to `IteratorExt` that counts occurrences of each item.
3. Create an extension trait for `Option<T>` that adds `or_error(msg: &str) -> Result<T, String>`.
