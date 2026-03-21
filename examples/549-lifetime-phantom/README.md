📖 **[View on hightechmind.io →](https://hightechmind.io/rust/549-lifetime-phantom)**

---

# PhantomData for Lifetime Markers
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Sometimes a struct logically borrows from an external lifetime but does not store any reference — perhaps it holds a raw pointer, a numeric ID, or an opaque handle. Without `PhantomData`, the compiler has no way to know the struct's relationship to that lifetime, leading to incorrect variance and missing lifetime checks. `PhantomData<&'a T>` is a zero-size type that carries lifetime and variance information without storing any data. It is essential for safe wrappers around raw pointers, arena allocators, typed indices, and foreign-function handles.

## Learning Outcomes

- Why `PhantomData<&'a T>` is needed when a struct logically borrows `'a` but has no reference field
- How `Handle<'a, T>` with `PhantomData<&'a T>` prevents handles from outliving their source
- How `Index<T>` uses `PhantomData<T>` for type-safety without storing a `T`
- How `PhantomData` affects variance (covariant, contravariant, invariant)
- Where `PhantomData` appears: arena allocators, foreign-function handles, typed indices, raw pointer wrappers

## Rust Application

`Handle<'a, T>` stores a `usize` id and `PhantomData<&'a T>` — the phantom data ensures the handle cannot outlive the data it refers to, even though no reference is stored. `Index<T>` uses `PhantomData<T>` for type-level tagging: an `Index<User>` and `Index<Post>` are distinct types even though both store a `usize`, preventing accidental index misuse. The `new` constructors take no reference — `PhantomData` is constructed as `PhantomData` (zero-size, zero-cost).

Key patterns:
- `PhantomData<&'a T>` — covariant in `'a` and `T`, carries lifetime without storing data
- `PhantomData<T>` — type marker for typed indices, zero-size at runtime
- `_marker: PhantomData` — the field name `_marker` is conventional

## OCaml Approach

OCaml achieves typed index safety through phantom types using abstract module signatures or the `Ppx_phantom` approach:

```ocaml
type 'a index = Index of int
let make_index n : 'a index = Index n
let get (Index n) = n
(* User_index and Post_index are the same type at runtime but distinct by convention *)
```

OCaml's phantom types are a convention — the runtime has no distinction. Rust's `PhantomData` enforces the distinction at the type level.

## Key Differences

1. **Runtime cost**: `PhantomData` is zero-size — no runtime overhead; OCaml phantom types are also zero-cost at runtime since the `'a` type parameter is erased.
2. **Lifetime enforcement**: Rust `PhantomData<&'a T>` enforces that `Handle<'a, T>` cannot outlive `'a` at compile time; OCaml phantom types cannot express lifetime constraints.
3. **Variance control**: `PhantomData` precisely controls variance (covariant, contravariant, invariant); OCaml phantom types are covariant by default unless annotated.
4. **Raw pointer wrappers**: Rust raw pointer wrappers use `PhantomData<T>` to tell the compiler what the pointer "logically owns"; OCaml raw pointers (via `Bigarray` or `ctypes`) rely on programmer discipline.

## Exercises

1. **Typed generation handle**: Implement `struct GenerationHandle<'arena, T> { id: u32, _p: PhantomData<&'arena T> }` where `'arena` ensures the handle cannot outlive the arena it was allocated from.
2. **Two phantom types**: Create `struct Matrix<T, Rows, Cols> { data: Vec<T>, _p: PhantomData<(Rows, Cols)> }` and implement `fn transpose(m: Matrix<T, R, C>) -> Matrix<T, C, R>` — use phantom row/col types to prevent transposing the wrong way.
3. **Invariant phantom**: Change `Handle<'a, T>` to use `PhantomData<&'a mut T>` — explain what changes in terms of variance and what programs the compiler now rejects.
