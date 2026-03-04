# 228: Natural Transformations

**Difficulty:** 3  **Level:** Advanced

A function that converts one container type to another without touching the values inside.

## The Problem This Solves

You often need to switch between container types. A function that might fail returns `Option<T>`, but another API expects `Vec<T>`. Or you want to normalize `Result<T, E>` to `Option<T>` to simplify branching. The naive approach is to write these conversions ad-hoc every time — scattering `match opt { Some(x) => vec![x], None => vec![] }` across your codebase.

The problem gets worse when you realize these conversions have a special property worth preserving: they should work the same regardless of what `T` is. A function that converts `Option<i32>` to `Vec<i32>` and a function that converts `Option<String>` to `Vec<String>` should behave identically in structure — the *container* changes, the *contents* don't.

Without this discipline, you might write conversions that accidentally peek inside the values, or ones that break when you apply a transformation before vs. after the conversion. This leads to subtle bugs in data pipeline code.

Natural transformation is the name for this pattern — converting containers in a way that's completely "transparent" to the inner type. This exists to solve exactly that pain.

## The Intuition

Think of a container as a "box with a label." `Option<T>` is a box that might be empty or have one item. `Vec<T>` is a box that can have zero or more items.

A **natural transformation** is a rule for relabeling the box — swapping `Option` for `Vec` — without opening the box or touching the items inside.

The concrete example: `option_to_vec` converts `None` to `[]` and `Some(x)` to `[x]`. It never looks at what `x` is. You can give it `Option<i32>`, `Option<String>`, `Option<MyStruct>` — the conversion rule is identical.

The "natural" part has a precise meaning: it doesn't matter whether you transform the items first and then change the container, or change the container first and then transform the items. Both orders give the same result. In code:

```rust
// Convert then map: Some(5) -> [5] -> [10]
option_to_vec(Some(5)).into_iter().map(|x| x * 2).collect::<Vec<_>>()

// Map then convert: Some(5) -> Some(10) -> [10]  
option_to_vec(Some(5).map(|x| x * 2))

// These are always equal — that's what "natural" means
```

This commutativity guarantee is called the **naturality condition**, and the code verifies it explicitly.

## How It Works in Rust

```rust
// A natural transformation: Vec<T> -> Option<T>
// Takes the first element (or None if empty)
// Notice: fn<T> — works for ANY T without any bounds
pub fn safe_head<T: Clone>(list: &[T]) -> Option<T> {
    list.first().cloned()
}

// The reverse transformation: Option<T> -> Vec<T>
pub fn option_to_vec<T>(opt: Option<T>) -> Vec<T> {
    match opt {
        None => vec![],          // empty container -> empty container
        Some(x) => vec![x],     // one item -> one-element list
    }
}

// Verify the naturality condition for any nat transform
// "nat_t" and "nat_u" are the same function, just instantiated at different types
// (Rust monomorphizes — one generic fn becomes multiple concrete fns at compile time)
pub fn verify_naturality<T, U>(
    f: impl Fn(T) -> U,          // the function to apply to values
    nat_t: impl Fn(&[T]) -> Option<T>,  // nat transform at type T
    nat_u: impl Fn(&[U]) -> Option<U>,  // nat transform at type U
    list: &[T],
) -> bool
where T: Clone, U: PartialEq {
    let mapped: Vec<U> = list.iter().map(|x| f(x.clone())).collect();
    let lhs = nat_u(&mapped);        // transform values, then change container
    let rhs = nat_t(list).map(f);    // change container, then transform values
    lhs == rhs                        // must be equal!
}

// Natural transformations compose — chain two container changes
pub fn nat_composed<T: Clone>(list: &[T]) -> Vec<T> {
    option_to_vec(safe_head(list))   // Vec -> Option -> Vec
}
```

The `verify_naturality` function needs two parameters for the same generic function because Rust monomorphizes generics — there's no way to pass "the same generic function twice" as a single value. This is a known limitation vs. OCaml.

## What This Unlocks

- **Data pipeline normalization:** Consistently convert between `Option`, `Result`, `Vec` at API boundaries without ad-hoc match expressions scattered everywhere.
- **Container adapters:** Build composable adapters (`result_to_opt`, `opt_to_vec`, etc.) that can be chained predictably because they're all natural transformations.
- **Iterator combinators:** `Iterator::flatten`, `Option::into_iter`, `Result::ok` are all natural transformations — understanding this explains why they compose so cleanly.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Polymorphic function | `'a list -> 'a option` intrinsically polymorphic | Generic `fn<T>`, monomorphized per use |
| Passing nat transforms as values | Single polymorphic function value | Need two monomorphized copies (`nat_t` and `nat_u`) |
| Naturality verification | Works with one polymorphic `nat` argument | Requires `nat_t: fn(&[T])->Option<T>` and `nat_u: fn(&[U])->Option<U>` |
| Naturality by construction | Parametric polymorphism guarantees it | Same — any `fn<T>(Vec<T>) -> Option<T>` that ignores `T` is automatically natural |
| Ownership | References, GC-managed | `Clone` required to return owned values |
