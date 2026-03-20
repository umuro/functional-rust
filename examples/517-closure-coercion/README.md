📖 **[View on hightechmind.io →](https://hightechmind.io/rust/517-closure-coercion)**

---

# Closure-to-Function-Pointer Coercion

## Problem Statement

C and systems programming have long relied on function pointers for callbacks — they are a fixed machine-word size, require no heap allocation, and map directly to a call instruction. Rust preserves this capability: non-capturing closures and named functions both coerce to `fn` pointer types, enabling zero-overhead callbacks in FFI-compatible APIs. The constraint is intentional — a capturing closure has extra data that a raw pointer cannot represent. Understanding when coercion works and when it fails helps you choose between `fn`, `impl Fn`, and `Box<dyn Fn>`.

## Learning Outcomes

- Why non-capturing closures coerce to `fn` pointers but capturing ones do not
- How named functions serve as `fn` pointer values
- How to build dispatch tables using arrays of `fn` pointers (uniform size, no fat pointer)
- When to use `fn` vs `impl Fn` vs `Box<dyn Fn>` for different API shapes
- How FFI callbacks require `fn` pointer types for ABI compatibility

## Rust Application

`apply_fn_ptr(f: fn(i32) -> i32, x: i32)` accepts only `fn` pointers. `make_transform_table()` returns `[fn(i32) -> i32; 4]` — an array of four function pointers including a non-capturing lambda. `build_pipeline(ops: Vec<fn(i32) -> i32>)` stores pointers in a `Vec` and folds over them. The `coercion_demo` function comments show that `let _: fn(i32) -> i32 = |x| x * 2` compiles but `let y = 5; let _: fn(i32) -> i32 = |x| x + y` does not.

Key patterns:
- `fn(i32) -> i32` — concrete function pointer type (single machine word, no allocation)
- Non-capturing `|x| x + 10` coercing into `fn` slot in array literal
- `type Callback = fn(i32) -> i32` — named alias for FFI-compatible pointer type

## OCaml Approach

OCaml has no function pointer / closure distinction at the value level — all functions are closures, and closed-over environments are heap-allocated. There is no direct coercion concept. For C FFI, OCaml uses `ctypes` or `Callback.register`, which wrap OCaml functions behind C-callable thunks. Performance-sensitive dispatch uses arrays of functions just as in Rust, but every entry is a closure regardless.

```ocaml
let ops = [| (fun x -> x * 2); (fun x -> x * 3) |]
let apply i x = ops.(i) x
```

## Key Differences

1. **Uniform representation**: Rust `fn` pointers are a single pointer word with no closure environment; OCaml always has a (pointer, environment) pair even for non-capturing functions.
2. **FFI compatibility**: Rust `fn` pointers map directly to C function pointers — usable in `extern "C"` callbacks without wrappers; OCaml requires `ctypes` machinery or `Callback.register`.
3. **Size guarantees**: Rust `fn` pointers have a known, fixed size enabling `[fn(T) -> U; N]` arrays; OCaml function values are opaque pointers of uniform size too, but via GC indirection.
4. **Type safety**: Rust distinguishes `fn(i32) -> i32` from `fn(i64) -> i32` at the type level with no implicit coercion; OCaml's type system similarly rejects mismatched function types at compile time.

## Exercises

1. **FFI table**: Build a `[fn(i32) -> i32; N]` dispatch table and write a function that takes an index and applies the corresponding operation, returning an error variant for out-of-bounds indices.
2. **Pipeline from strings**: Write `build_named_pipeline(ops: &[&str])` that looks up named operations (`"double"`, `"negate"`, etc.) in a `HashMap<&str, fn(i32) -> i32>` and returns a composed `fn` pipeline.
3. **Capturing fallback**: Demonstrate that `Box<dyn Fn(i32) -> i32>` can hold both `fn` pointers and capturing closures, and write a dispatcher that tries a `fn` pointer table first and falls back to a `Box<dyn Fn>` registry.
