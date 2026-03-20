📖 **[View on hightechmind.io →](https://hightechmind.io/rust/077-generic-bounds)**

---

# 077 — Generic Bounds
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Generic bounds constrain which types a generic function or struct can be used with. `fn print_list<T: Display>(items: &[T])` works for any T that implements Display. Without bounds, generic code cannot call any methods on T. With bounds, you unlock the full interface of the trait.

Bounds are Rust's mechanism for "bounded polymorphism" — the type theory concept underlying Java interfaces, Haskell typeclasses, and OCaml module signatures. They enable writing algorithms once that work across many types, with the compiler verifying type safety and monomorphizing for performance.

## Learning Outcomes

- Use single trait bounds: `T: Display`
- Use multiple bounds with `+`: `T: Display + Clone`
- Use bounds for arithmetic: `T: std::iter::Sum + Copy`
- Implement generic functions that call trait methods on their type parameters
- Understand the difference between bounds on type parameters and where clauses

## Rust Application

`print_item<T: Display>` formats any displayable value. `print_and_clone<T: Display + Clone>` requires both formatting and cloning. `find_max<T: PartialOrd + Clone>` uses `>=` (requires `PartialOrd`). `sum_items<T: Sum + Copy>` uses `copied().sum()`. `contains<T: PartialEq>` uses `==`. Each bound unlocks specific operations: `Display` → `{}`, `PartialOrd` → `>`, `Sum` → `.sum()`.

## OCaml Approach

OCaml's equivalent is module functors or type constraints. `let print_item (type a) (show: a -> string) (x: a) = print_string (show x)` — OCaml passes the "typeclass dictionary" explicitly as a function argument. With modular implicits or ppx_deriving, this becomes less verbose. OCaml 5 modules with functors: `module type PRINTABLE = sig type t val to_string : t -> string end`.

## Key Differences

1. **Implicit vs explicit**: Rust resolves trait implementations implicitly — `T: Display` finds the implementation automatically. OCaml passes trait dictionaries (module arguments) explicitly in the traditional style.
2. **Multiple bounds**: Rust's `T: A + B + C` is compact. OCaml's functor approach requires a module combining all required interfaces.
3. **`where` clause**: `where T: Display + Clone` is equivalent to the inline bound but more readable for complex constraints. Both compile to the same code.
4. **Monomorphization**: Rust monomorphizes generic functions per concrete type — `print_item::<i32>` and `print_item::<String>` are separate compiled functions. OCaml uses boxing for polymorphism (no monomorphization by default).

## Exercises

1. **Median function**: Write `median<T: PartialOrd + Clone>(mut v: Vec<T>) -> Option<T>` that sorts the vector and returns the middle element. What additional bound is needed for sorting?
2. **Min and max**: Write `min_max<T: PartialOrd + Clone>(v: &[T]) -> Option<(T, T)>` that returns both the minimum and maximum in a single pass. Use `fold` with a `(T, T)` accumulator.
3. **Generic statistics**: Write `Stats<T>` that stores count, sum, min, max with bounds `T: PartialOrd + Add + Copy + Default`. Implement `update(&mut self, x: T)` that incorporates a new value.
