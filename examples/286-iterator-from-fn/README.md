📖 **[View on hightechmind.io →](https://hightechmind.io/rust/286-iterator-from-fn)**

---

# 286: Creating Iterators with from_fn()

## Problem Statement

Building a custom iterator by defining a struct and implementing `Iterator` is powerful but verbose for simple cases. `std::iter::from_fn()` provides a lightweight alternative: create an iterator directly from a closure returning `Option<T>`. The closure captures its own mutable state, and each call produces the next element or `None` to terminate. This is the functional approach to generators — describing iteration as a stateful function.

## Learning Outcomes

- Understand `from_fn(f)` as creating an iterator from a `FnMut() -> Option<T>` closure
- Use captured mutable variables in the closure to maintain iteration state
- Implement Fibonacci and other stateful sequences with less boilerplate than a struct
- Recognize `from_fn` as the bridge between stateful computation and the iterator ecosystem

## Rust Application

`std::iter::from_fn(f)` takes a `FnMut() -> Option<T>` and produces an iterator. The closure captures state:

```rust
// Counter from 1 to max
pub fn counter(max: i32) -> impl Iterator<Item = i32> {
    let mut n = 0;
    std::iter::from_fn(move || {
        n += 1;
        if n <= max { Some(n) } else { None }
    })
}

// Fibonacci with overflow protection
pub fn fibonacci() -> impl Iterator<Item = u64> {
    let (mut a, mut b) = (0u64, 1u64);
    std::iter::from_fn(move || {
        let val = a;
        let next = a.checked_add(b)?; // None on overflow terminates iterator
        a = b;
        b = next;
        Some(val)
    })
}
```

## OCaml Approach

OCaml's `Seq.unfold` is the direct equivalent: it takes an initial state and a function `state -> (element * state) option`:

```ocaml
let counter max =
  Seq.unfold (fun n -> if n > max then None else Some (n, n+1)) 1

let fibonacci =
  Seq.unfold (fun (a, b) -> Some (a, (b, a+b))) (0, 1)
```

`Seq.unfold` and `from_fn` are semantically equivalent — both produce lazy sequences from stateful generators.

## Key Differences

1. **State style**: Rust's `from_fn` uses captured mutable variables (mutable closure captures); OCaml's `unfold` passes state as a pure value through each step.
2. **Functional vs imperative**: OCaml's `unfold` is purely functional (state is passed explicitly); Rust's `from_fn` is imperative (state is mutated in place).
3. **Termination**: Both return `None`/`None` to signal exhaustion; `checked_add` in Rust enables overflow-safe termination.
4. **Composability**: Both integrate with their respective adapter ecosystems — `from_fn` iterators get all `Iterator` adapters, `Seq.unfold` gets all `Seq` functions.

## Exercises

1. Use `from_fn` to implement a random number generator iterator that produces pseudo-random numbers using an LCG (linear congruential generator) with captured seed state.
2. Implement the Collatz sequence using `from_fn`, terminating when the value reaches 1.
3. Build a `retry_until_success` iterator using `from_fn` that calls a fallible function and keeps retrying until it returns `Ok`, yielding the attempts.
