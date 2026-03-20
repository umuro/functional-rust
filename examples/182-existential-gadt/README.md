📖 **[View on hightechmind.io →](https://hightechmind.io/rust/182-existential-gadt)**

---

# Existential Types via Box<dyn Trait>

## Problem Statement

An existential type packs a value together with proof that it satisfies an interface, erasing the concrete type. The consumer of an existential value can use the interface but cannot recover the original type. This enables heterogeneous collections (a `Vec` of different types all implementing the same trait), plugin architectures, and open-ended extension points. Rust's `Box<dyn Trait>` is the primary existential mechanism, paralleling OCaml's GADT-based existential encoding.

## Learning Outcomes

- Understand existential types as "there exists a type satisfying this interface"
- Build heterogeneous collections with `Box<dyn Display>` and custom traits
- Implement the closure-based existential pattern for packing values with their operations
- See how existentials enable extension without recompilation (open-world assumption)

## Rust Application

`Vec<Box<dyn Display>>` stores `42`, `"hello"`, and `3.14` together — each has a different concrete type, all erased behind `Display`. The custom `Showable` struct packs any value with its `show` function as a closure: `struct Showable { show: Box<dyn Fn() -> String> }`. This is the GADT existential: `exists T. (T, T -> String)` packed into one value. The closure captures both the value and the function, erasing `T` completely.

## OCaml Approach

OCaml encodes existentials via GADTs:
```ocaml
type showable = Show : 'a * ('a -> string) -> showable
let show (Show (x, f)) = f x
let showables = [Show (42, string_of_int); Show ("hello", Fun.id)]
```
The `Show` constructor packs the value `x: 'a` and its `show` function `f: 'a -> string`, erasing `'a` in `showable`. OCaml's encoding is more transparent than Rust's closure-based approach — the packed function is explicit.

## Key Differences

1. **GADT vs. closure**: OCaml's GADT explicitly packs `(value, function)`; Rust's closure-based approach bundles them in the captured environment — equivalent but differently structured.
2. **Vtable vs. closure**: Rust's `Box<dyn Trait>` uses a vtable; the closure approach creates an independent dispatch mechanism — useful when the "trait" is just one function.
3. **Type recovery**: Neither OCaml's `Show` existential nor Rust's `Box<dyn Any>` allows recovering the concrete type in the existential position; `Box<dyn Any>` is a different (universally typed) erasure.
4. **Collection use**: Both `Vec<showable>` in OCaml and `Vec<Box<dyn Display>>` in Rust are idiomatic for heterogeneous collections.

## Exercises

1. Create `Vec<Box<dyn std::error::Error>>` and store different error types, then iterate and print each.
2. Implement a `Callback` type using the closure existential: `struct Callback { call: Box<dyn Fn(Event) -> ()> }`.
3. Write a plugin registry that stores `Vec<Box<dyn Plugin>>` and calls `plugin.name()` and `plugin.run()` on each.
