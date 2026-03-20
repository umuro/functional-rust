📖 **[View on hightechmind.io →](https://hightechmind.io/rust/518-function-pointers)**

---

# Function Pointers vs Closures
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Two abstractions represent callable values in Rust: `fn` pointers (a plain machine address) and closures (address plus captured environment). The tension between them matters in practice: `fn` pointers have a known, fixed size — useful for FFI, const contexts, and uniform dispatch tables. Closures are more powerful but carry hidden state and require generics or boxing. Choosing the wrong abstraction forces unnecessary heap allocation or limits caller flexibility. This example compares the two side-by-side including their memory layout.

## Learning Outcomes

- The concrete memory difference between `fn` pointers, non-capturing closures, and capturing closures
- How `apply_fn_ptr(f: fn(i32) -> i32)` differs from `apply_generic<F: Fn(i32) -> i32>(f: F)`
- Why named functions can be used directly as `fn` pointer values
- When to prefer `fn` (FFI, tables, const) vs `impl Fn` (generic) vs `Box<dyn Fn>` (dynamic)
- How `std::mem::size_of_val` reveals the size of each callable kind

## Rust Application

`square`, `cube`, and `double` are named functions usable as `fn(i32) -> i32` values. `math_ops()` returns `Vec<(&'static str, fn(i32) -> i32)>` — a named dispatch table. `apply_generic<F: Fn(i32) -> i32>` accepts both `fn` pointers and any closure via monomorphization. `size_comparison()` calls `std::mem::size_of_val` on a `fn` pointer (one word), a non-capturing lambda (zero bytes), and a capturing lambda (size of captured `i32`).

Key patterns:
- `fn(i32) -> i32` — zero-overhead, no allocation, FFI-safe
- `F: Fn(i32) -> i32` — zero-cost generic, monomorphized at compile time
- `std::mem::size_of_val(&closure)` — inspect runtime size of a callable

## OCaml Approach

OCaml has a unified function type — there is no `fn` pointer vs closure distinction at the source level. All functions are closures; non-capturing ones compile to a record with a code pointer and an empty environment. The compiler optimizes away the environment allocation for known non-capturing functions in many cases, but the type does not distinguish them.

```ocaml
let square x = x * x
let ops = [("square", square); ("double", fun x -> x * 2)]
let apply f x = f x
```

## Key Differences

1. **Size visibility**: Rust exposes `size_of_val` to measure closure size at compile time; OCaml treats all function values as one-word GC pointers, hiding the environment.
2. **FFI boundary**: Rust `fn` pointers cross C FFI boundaries natively; OCaml functions require `ctypes` wrappers or `Callback.register` for the same.
3. **Generic dispatch**: Rust monomorphizes `F: Fn(T) -> U` into separate code per closure type; OCaml uses boxing (value representation) — no separate copies but with indirection.
4. **Dispatch tables**: Rust `Vec<fn(i32) -> i32>` stores uniform-size pointers with no allocation overhead per entry; OCaml `list` of functions stores GC-managed boxed values.

## Exercises

1. **Benchmark dispatch**: Measure the performance difference between calling via `fn(i32) -> i32`, via `impl Fn(i32) -> i32`, and via `Box<dyn Fn(i32) -> i32>` in a tight loop using `std::hint::black_box`.
2. **Const dispatch table**: Define a `const` array `OPS: [fn(i32) -> i32; 4]` at the module level and verify it is accessible in `const` evaluation contexts.
3. **Dynamic registry**: Build a `HashMap<String, Box<dyn Fn(i32) -> i32>>` where named functions and capturing closures can both be registered, then write a `run(name, arg)` dispatcher.
