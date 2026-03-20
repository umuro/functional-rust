📖 **[View on hightechmind.io →](https://hightechmind.io/rust/122-higher-order-fn)**

---

# Higher-Order Functions with Lifetime Constraints

## Problem Statement

Higher-order functions (HOFs) — functions that take or return other functions — are the backbone of functional programming. In Rust, HOFs that deal with references must explicitly declare lifetimes to tell the borrow checker how long returned references live relative to their inputs. Without this, the compiler cannot guarantee memory safety. This example shows the patterns for safe HOFs: finding elements, composing functions, and filtering collections.

## Learning Outcomes

- Understand how lifetime annotations on HOFs constrain the relationship between input and output references
- Learn function composition (`compose`) and how it preserves types through two transformations
- See how `find_first` and `filter_refs` safely return references into the input collection
- Practice writing generic HOFs with multiple type parameters and lifetime bounds

## Rust Application

`find_first<'a, F>(items: &'a [&'a str], pred: F) -> Option<&'a str>` ties the lifetime of the returned string slice to the input slice — the compiler rejects any use after the slice is dropped. `compose<A, B, C, F, G>` works on owned types, requiring no lifetime annotation. `apply_to_str<'a, F>(s: &'a str, f: F) -> &'a str` ensures the function `f` cannot return a reference to something with a shorter lifetime than `s`.

## OCaml Approach

OCaml's type system handles most HOF patterns automatically. `List.find`, `List.filter`, and function composition (`>>`) work without lifetime annotations because the GC ensures referenced values stay alive. OCaml's higher-order functions use polymorphic types like `('a -> bool) -> 'a list -> 'a list`, with the type checker inferring all instantiations.

## Key Differences

1. **Lifetime annotations**: Rust HOFs operating on references require explicit `'a` annotations; OCaml infers everything with no annotation burden.
2. **Monomorphization**: Rust HOFs with `impl Fn` or `F: Fn` bounds generate a concrete copy per call site; OCaml uses a single polymorphic function with no code duplication.
3. **Composition types**: Rust `compose` requires three type parameters plus two function bounds; OCaml's `(f >> g)` uses a single polymorphic `('a -> 'b) -> ('b -> 'c) -> 'a -> 'c`.
4. **First-class functions**: Both treat functions as first-class values; Rust stores them as zero-sized types (no heap allocation for `impl Fn`); OCaml stores closures as heap records.

## Exercises

1. Write `apply_n<T, F: Fn(T) -> T>(f: F, n: usize, init: T) -> T` and use it to apply a doubling function 10 times.
2. Add explicit lifetime annotations to `filter_refs` and explain why the returned `Vec<&T>` cannot outlive the input slice.
3. Implement `pipe<A, B, C>(g: impl Fn(A) -> B, f: impl Fn(B) -> C) -> impl Fn(A) -> C` (same as `compose` but argument order reversed) and demonstrate it with string processing.
