📖 **[View on hightechmind.io →](https://hightechmind.io/rust/505-boxing-closures)**

---

# Boxing Closures
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


`Box<dyn Fn(A) -> B>` erases the concrete closure type, enabling heterogeneous collections of closures, dynamic dispatch, and storage as struct fields when the closure type is not known at compile time.

## Problem Statement

Every Rust closure has a unique anonymous type — two closures with identical signatures have different types and cannot be stored in the same `Vec` or struct field. `Box<dyn Fn>` resolves this by heap-allocating the closure and storing only a fat pointer (address + vtable). This is the mechanism behind: event handler registries, middleware chains, dependency injection containers, and any pattern that selects behaviour at runtime. The cost is one heap allocation per boxed closure and one vtable lookup per call — acceptable for most use cases.

## Learning Outcomes

- Define type aliases `BoxedFn = Box<dyn Fn(i32) -> i32>` for ergonomics
- Store multiple closures with different captures in a `HashMap<String, BoxedFn>`
- Chain `Vec<BoxedFn>` closures with `fold` for pipeline execution
- Select closures at runtime with match/string dispatch
- Build a `ClosureVec` that accepts `impl Fn + 'static` and stores as `Box<dyn Fn>`

## Rust Application

`BoxedFn` type alias and creation:

```rust
pub type BoxedFn = Box<dyn Fn(i32) -> i32>;

pub fn make_boxed_adder(n: i32) -> BoxedFn {
    Box::new(move |x| x + n)
}
```

Heterogeneous collection of closures:

```rust
let mut map: HashMap<String, BoxedFn> = HashMap::new();
map.insert("double".into(), Box::new(|x| x * 2));
map.insert("square".into(), Box::new(|x| x * x));
```

Pipeline via `fold`:

```rust
pub fn chain_closures(closures: Vec<BoxedFn>, value: i32) -> i32 {
    closures.iter().fold(value, |acc, f| f(acc))
}
```

## OCaml Approach

OCaml functions are first-class with uniform representation — no boxing is needed for heterogeneous collections:

```ocaml
let ops : (string * (int -> int)) list = [
  ("double", fun x -> x * 2);
  ("square", fun x -> x * x);
]

let chain closures value =
  List.fold_left (fun acc f -> f acc) value closures
```

OCaml's first-class function values already carry their captured environment via GC-managed closures, so no explicit `Box` or dynamic dispatch is needed.

## Key Differences

1. **Boxing requirement**: Rust needs `Box<dyn Fn>` for heterogeneous closure collections; OCaml's uniform representation handles this natively.
2. **`'static` bound**: `Box<dyn Fn + 'static>` requires captured values to have `'static` lifetime — no borrowed references to local variables. OCaml's GC has no such constraint.
3. **Vtable overhead**: Rust's `Box<dyn Fn>` has a 2-pointer fat pointer and vtable dispatch; `impl Fn` is zero-overhead. OCaml's closures always use indirect dispatch.
4. **Type aliases**: Rust's `type BoxedFn = Box<dyn Fn(i32) -> i32>` is a readability convention; OCaml uses `type fn_t = int -> int` similarly.

## Exercises

1. **Middleware stack**: Build a `struct Middleware { handlers: Vec<Box<dyn Fn(Request) -> Response>> }` where each handler can short-circuit the chain by returning early.
2. **Event emitter**: Implement `EventEmitter<E>` with a `HashMap<String, Vec<Box<dyn Fn(&E)>>>` that registers and fires named events.
3. **`impl Fn` vs. `Box<dyn Fn>` benchmark**: Use `criterion` to measure the call overhead of `Box<dyn Fn(i32)->i32>` vs. `impl Fn(i32)->i32` for 10 million iterations.
