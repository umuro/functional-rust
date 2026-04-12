📖 **[View on hightechmind.io →](https://hightechmind.io/rust/116-box-heap)**

---

# 116-box-heap — Box<T>: Heap Allocation
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Rust values live on the stack by default. Two situations require heap allocation: when a value is too large for the stack, and when a type is recursive (its size would be infinite). `Box<T>` is Rust's simplest heap allocation — a single owned pointer to a heap-allocated `T` with no reference counting overhead.

`Box<T>` is also the mechanism for trait objects (`Box<dyn Trait>`) enabling runtime polymorphism — the Rust equivalent of OOP's virtual dispatch.

## Learning Outcomes

- Use `Box<T>` to heap-allocate large data with single ownership
- Understand why recursive types require `Box` (breaks the infinite size recursion)
- Use `Box<dyn Trait>` for dynamic dispatch and runtime polymorphism
- Know the deref coercion: `Box<T>` automatically derefs to `&T`
- Understand that `Box<T>` drops the inner value when it goes out of scope

## Rust Application

`src/lib.rs` demonstrates three patterns. `sum_boxed_squares` boxes a `Vec` to heap-allocate it — rarely needed in practice but illustrates the mechanic. The `Expr` enum uses `Box<Expr>` to make the recursive type have a known size: each variant stores an 8-byte pointer rather than an infinitely-sized sub-tree. `eval` recursively evaluates `Expr` trees. `Box<dyn Fn(i32) -> i32>` stores closures of any type for heterogeneous collections.

Recursive types (`Expr`, `List<T>`, `Tree<T>`) always require `Box` in Rust — this is the most common real-world use case.

## OCaml Approach

OCaml heap-allocates everything except integers and booleans — all ADT values, structs, and closures live on the heap implicitly:

```ocaml
type expr =
  | Num of int
  | Add of expr * expr  (* no Box needed — GC handles recursion *)
  | Mul of expr * expr

let rec eval = function
  | Num n -> n
  | Add (a, b) -> eval a + eval b
  | Mul (a, b) -> eval a * eval b
```

OCaml's GC manages the recursive heap allocations automatically. There is no equivalent to `Box` — all variant values are heap-allocated by the runtime.

## Key Differences

1. **Automatic vs explicit**: OCaml heap-allocates all non-trivial values automatically; Rust requires `Box<T>` to explicitly move to the heap.
2. **Recursive types**: OCaml recursive ADTs work without any annotation; Rust requires `Box<T>` to break the size cycle.
3. **`dyn Trait`**: Rust's `Box<dyn Trait>` enables dynamic dispatch; OCaml uses polymorphic variants or first-class modules for equivalent runtime polymorphism.
4. **Drop semantics**: Rust's `Box<T>` drops the inner value when the `Box` goes out of scope; OCaml's GC collects values when reference count drops to zero.

## Exercises

1. Add `Neg(Box<Expr>)` and `Div(Box<Expr>, Box<Expr>)` variants to `Expr` and extend `eval` to handle them.
2. Implement a `Display` for `Expr` that prints expressions with correct parenthesization.
3. Write a list of heterogeneous closures `Vec<Box<dyn Fn(i32) -> i32>>` and apply them all in sequence to an initial value.
