# Comparison: Lazy Sequences — OCaml vs Rust

## Core Insight

OCaml's `Seq` provides lazy sequences as an explicit abstraction layered on top of eager lists. Rust's iterators are lazy from the ground up — `map`, `filter`, `take` all return lazy adaptors that only evaluate when consumed by `collect`, `for_each`, etc. This design means Rust doesn't need a separate `Seq` module; every iterator is already a lazy sequence.

## OCaml

```ocaml
let fibs = Seq.unfold (fun (a, b) -> Some (a, (b, a + b))) (0, 1)
let primes = Seq.ints 2 |> Seq.filter is_prime
let first10 = Seq.take 10 fibs |> Seq.iter (Printf.printf "%d ")
```

## Rust

```rust
pub fn fibs() -> impl Iterator<Item = u64> {
    std::iter::successors(Some((0, 1)), |&(a, b)| Some((b, a + b)))
        .map(|(a, _)| a)
}

let first10: Vec<u64> = fibs().take(10).collect();
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Lazy by default | No (lists are eager) | Yes (iterators are lazy) |
| Infinite range | `Seq.ints 0` | `0..` (built-in) |
| Unfold | `Seq.unfold f seed` | `std::iter::successors` or `from_fn` |
| Take N | `Seq.take n seq` | `.take(n)` |
| Filter | `Seq.filter pred seq` | `.filter(pred)` |
| Consume | `Seq.iter f seq` | `.collect()` or `.for_each()` |
| Custom iterator | Implement `unit -> 'a Seq.node` | `impl Iterator for T` |

## Learner Notes

- **`impl Iterator<Item = T>`**: Return type hides the concrete iterator type — like OCaml's abstract `'a Seq.t`
- **`successors`**: Generates `Some(next)` from previous — perfect for Fibonacci-style sequences
- **`from_fn`**: Most flexible — closure with mutable state, returns `Option<T>` per call
- **No allocation**: Rust's lazy chains allocate nothing until `collect()` — OCaml's `Seq` also avoids allocation via thunks
- **`0..`**: Rust's range syntax creates an infinite iterator — no `Seq.ints` needed
