# 001: Higher-Order Functions

**Difficulty:** 1  **Level:** Beginner

Pass functions as arguments, return them as values, and use `map`/`filter`/`fold` — the three most useful tools in functional programming.

## The Problem This Solves

Imagine you need to double every number in a list, then do the same but triple them, then square them. Without higher-order functions, you write nearly identical loops three times. Change the logic? Edit in three places. That's how bugs are born.

Higher-order functions let you write the loop *once* and pass in what changes: the transformation. `double_all` and `triple_all` become the same function with different arguments. This is the insight behind `map`: apply *any* function to every element of a list, without writing the loop yourself.

The same principle applies to `filter` (keep elements matching a condition) and `fold` (combine elements into a single value). Once you have these three, you can express almost any data transformation as a clean, readable pipeline — no imperative loops required.

## The Intuition

Python developers already know this:
```python
doubled = [x * 2 for x in numbers]          # map
evens   = [x for x in numbers if x % 2 == 0] # filter
total   = sum(numbers)                         # fold/reduce
```

JavaScript:
```js
numbers.map(x => x * 2).filter(x => x % 2 === 0).reduce((a, b) => a + b, 0)
```

Rust looks almost identical to the JS version:
```rust
numbers.iter().map(|x| x * 2).filter(|x| x % 2 == 0).sum()
```

The `|x| x * 2` syntax is a closure — Rust's version of an arrow function or lambda. The `||` holds the parameters, and the expression after is the body.

## How It Works in Rust

```rust
// A function that takes another function as an argument
fn apply<A, B>(f: impl Fn(A) -> B, x: A) -> B {
    f(x)  // just call f with x
}

// Apply a function twice — compose with itself
fn twice<A>(f: impl Fn(A) -> A, x: A) -> A {
    f(f(x))
}

// A function that RETURNS a function (closure)
fn adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n  // `move` captures `n` so the closure can outlive this call
}

let add10 = adder(10);  // add10 is now a function
println!("{}", add10(7)); // 17

// map / filter / fold on slices
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
//                                          ^^^^ destructure the &i32 reference

let evens: Vec<i32> = numbers.iter().copied().filter(|x| x % 2 == 0).collect();
//                              ^^^^^^ turn &i32 into i32 before filtering

let total: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
// fold takes: initial value, then a closure (accumulator, current) -> new accumulator

// All three together: square → keep > 10 → sum
let result = map_filter_fold(
    numbers.iter().copied(),
    |x| x * x,          // map: square
    |x| x > &10,        // filter: keep only > 10
    0,
    |acc, x| acc + x,   // fold: sum
);
// 16 + 25 + 36 + 49 + 64 + 81 + 100 = 371
```

The `<A, B>` in function signatures are *generics* — they mean "this works for any types A and B." `impl Fn(A) -> B` means "any function (or closure) that takes A and returns B."

## What This Unlocks

- **Data processing** — transform and aggregate any collection without writing loops: filter active users, sum invoice amounts, normalize strings
- **Callbacks and event handling** — pass behavior as an argument, like sorting with a custom comparator or handling errors with a fallback function
- **Building your own abstractions** — write utilities like `retry(n, f)` or `timed(f)` that wrap any function with extra behavior

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Lambda syntax | `fun x -> x + 1` | `\|x\| x + 1` |
| Map a list | `List.map f xs` | `xs.iter().map(f).collect()` |
| Filter | `List.filter pred xs` | `xs.iter().filter(pred).collect()` |
| Fold/reduce | `List.fold_left f init xs` | `xs.iter().fold(init, f)` |
| Function type | `int -> int` | `impl Fn(i32) -> i32` |
| Return a function | Natural — just return a lambda | `fn f() -> impl Fn(...)` with `move` closure |
