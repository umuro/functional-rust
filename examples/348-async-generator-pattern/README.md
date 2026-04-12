📖 **[View on hightechmind.io →](https://hightechmind.io/rust/348-async-generator-pattern)**

---

# 348: Async Generator Pattern
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Some computations produce values lazily — database result sets, network streams, event queues — where computing all values upfront would be wasteful or impossible. Generators (Python's `yield`, JavaScript's `function*`, C#'s `yield return`) allow a function to produce values one at a time, suspending between yields. Rust doesn't have stable generators yet (RFC 2996 is in progress), but the pattern is emulated via the `Iterator` trait for synchronous generators and async streams (`Stream` trait from `futures`) for async generators. Understanding this pattern prepares you for when Rust's native generator syntax lands.

## Learning Outcomes

- Implement a generator-like struct that holds mutable state and produces values via `next()`
- Derive the `Iterator` trait to make the generator composable with iterator adapters
- Use `reset()` to replay a generator from the beginning
- Understand that generators are stateful iterators — each call to `next()` advances the state
- Recognize the difference between a generator (lazy sequence) and a `Vec` (eager sequence)
- See how `from_fn` and `successors` are Rust's built-in generator constructors

## Rust Application

```rust
pub struct Generator<T> {
    items: Vec<T>,
    index: usize,
}

impl<T: Clone> Generator<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items, index: 0 }
    }
    pub fn next(&mut self) -> Option<T> {
        if self.index < self.items.len() {
            let item = self.items[self.index].clone();
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
    pub fn reset(&mut self) {
        self.index = 0;
    }
}

impl<T: Clone> Iterator for Generator<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> { Generator::next(self) }
}

// Simpler: Rust's built-in lazy generators
let fib = std::iter::from_fn({
    let (mut a, mut b) = (0u64, 1u64);
    move || {
        let next = a;
        (a, b) = (b, a + b);
        Some(next)
    }
});
```

The `Generator` struct is essentially an `Iterator` with an explicit `reset()`. For true lazy generation without precomputing all values, use `std::iter::from_fn` or `std::iter::successors` — they take a closure that captures mutable state and call it on each `next()`.

## OCaml Approach

OCaml 4.14+ has `Seq` for lazy sequences:

```ocaml
let range start stop =
  let rec go i () =
    if i >= stop then Seq.Nil
    else Seq.Cons (i, go (i + 1))
  in
  go start

(* Use as: Seq.take 5 (range 0 100) |> List.of_seq *)
```

`Seq.t` is a lazy list — each node is a thunk (`unit -> 'a Seq.node`) computed on demand. In OCaml 5, `Effect`-based generators allow `yield`-like semantics within a delimited continuation, closer to Python generators.

## Key Differences

| Aspect | Rust `Iterator` | OCaml `Seq.t` |
|--------|----------------|---------------|
| Laziness | On-demand `next()` | Thunk-per-node |
| Mutability | Mutable state in closure/struct | Immutable (new thunk each time) |
| Native yield | Not yet (generator RFC pending) | Effect-based in OCaml 5 |
| Composability | Full iterator adapter chain | `Seq.map`, `Seq.filter`, `Seq.flat_map` |
| Async equivalent | `futures::Stream` | `Lwt_seq` / streaming Lwt promises |

## Exercises

1. **Fibonacci generator**: Implement a `Fibonacci` struct that implements `Iterator<Item = u64>`, producing the infinite Fibonacci sequence; take the first 20 values with `.take(20).collect::<Vec<_>>()`.
2. **Chunked generator**: Add a `chunks(size: usize)` method to `Generator<T>` that returns groups of `size` items at a time as `Vec<T>`; handle the last partial chunk correctly.
3. **Async stream**: Using `tokio_stream` or `futures::stream::unfold`, implement an async generator that yields numbers 1..N with a 10ms delay between each; collect the first 5.
