# Function Composition: OCaml vs Rust

## The Core Insight
Composition is the essence of functional programming: build complex behavior by snapping together simple functions. OCaml expresses this through the pipe operator `|>` and custom composition operators. Rust achieves it through iterator method chaining, which is arguably even more natural for data processing pipelines.

## OCaml Approach
OCaml's pipe operator `|>` is the workhorse for composition:
```ocaml
let result = data
  |> List.map (fun x -> x * 2)
  |> List.filter (fun x -> x > 5)
  |> List.fold_left ( + ) 0
```
Custom composition operators are easy to define:
```ocaml
let compose f g x = f (g x)
let ( >> ) f g x = g (f x)
```
Functions compose naturally because of automatic currying — `List.map f` returns a function ready to accept a list.

## Rust Approach
Rust's iterator adapters ARE composition — each method returns a new iterator:
```rust
let result: i64 = data.iter()
    .map(|x| x * 2)
    .filter(|x| x > &5)
    .sum();
```
For explicit function composition, closures do the job:
```rust
fn compose<A,B,C>(f: impl Fn(B)->C, g: impl Fn(A)->B) -> impl Fn(A)->C {
    move |x| f(g(x))
}
```
Rust's zero-cost abstractions mean iterator chains compile to the same code as hand-written loops.

## Key Differences
| Aspect | OCaml | Rust |
|--------|-------|------|
| Pipe operator | `\|>` (built-in) | No equivalent (method chaining instead) |
| Composition | Custom `compose` / `>>` | Closures or method chains |
| Iterator chains | `List.map f \|> List.filter g` | `.map(f).filter(g)` |
| Lazy evaluation | Lists are eager | Iterators are lazy |
| Type inference | Full | Usually needs return type hints |
| Performance | Intermediate lists allocated | Zero-cost (fused into single pass) |

## What Rust Learners Should Notice
- **Method chaining replaces `|>`**: Rust's `.map().filter().collect()` is the idiomatic equivalent of OCaml's pipe chains. It reads left-to-right and composes naturally.
- **Iterators are lazy**: Unlike OCaml's `List.map` which creates a new list, Rust iterators don't allocate until `.collect()`. This is a major performance advantage.
- **Explicit composition is rare in Rust**: While you *can* build `compose(f, g)`, idiomatic Rust prefers method chains or closures that call both functions inline.
- **`Box<dyn Fn>` vs `impl Fn`**: Composing functions dynamically requires `Box<dyn Fn>` (heap allocation). Static composition with `impl Fn` is zero-cost but can't be stored in collections.

## Further Reading
- [The Rust Book — Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [OCaml Stdlib — Fun module](https://v2.ocaml.org/api/Fun.html)
- [Rust Iterator documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
