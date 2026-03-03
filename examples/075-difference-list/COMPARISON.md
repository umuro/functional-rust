# Difference List — OCaml vs Rust Comparison

## Core Insight

Difference lists solve a problem that's acute in functional languages: list append (`@` / `++`) is O(n) because linked lists can only be traversed from the head. By representing a list as a function `rest → our_elements ++ rest`, append becomes function composition — O(1). In Rust, `Vec::extend()` is already O(m) (where m is the appended portion), making this pattern more educational than practical.

## OCaml Approach

Brilliantly minimal. A difference list is just `'a list -> 'a list` — a type alias for a function. `empty = Fun.id`, `singleton x = fun rest -> x :: rest`, `append a b = fun rest -> a (b rest)`. Pure function composition, no data structure at all. This is where OCaml's first-class functions shine.

## Rust Approach

Requires `Box<dyn Fn(Vec<T>) -> Vec<T>>` for the stored function, since closures have unique types. Each operation allocates a new boxed closure. The `T: Clone` bound is needed because closures may be called multiple times. In practice, Rust programmers would just use `Vec` with `extend()` — it's simpler and faster.

## Comparison Table

| Aspect        | OCaml                           | Rust                                    |
|---------------|---------------------------------|-----------------------------------------|
| **Memory**    | Closures (GC'd)                | `Box<dyn Fn>` (heap allocated)          |
| **Null safety** | N/A                          | N/A                                     |
| **Errors**    | N/A                            | N/A                                     |
| **Iteration** | Function application            | Function application + Vec ops          |
| **Necessity** | Solves O(n) append problem     | Unnecessary — Vec already O(1) push     |

## Things Rust Learners Should Notice

1. **The problem doesn't exist in Rust** — `Vec` has O(1) amortized push; no need for difference lists
2. **`Box<dyn Fn>` overhead** — each closure allocation costs more than Vec operations
3. **`T: 'static + Clone`** — required for closures to own and reproduce values
4. **Function composition** — `|rest| f(g(rest))` is the core trick, same in both languages
5. **Know when NOT to port** — some FP patterns don't translate because Rust's data structures already solve the problem

## Further Reading

- [Difference list (Wikipedia)](https://en.wikipedia.org/wiki/Difference_list)
- [Vec::extend](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.extend)
- [When to use FP patterns in Rust](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
