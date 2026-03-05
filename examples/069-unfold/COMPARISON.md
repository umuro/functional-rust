## Core Insight

Unfold is the dual of fold: while fold reduces a collection to a value, unfold generates a collection from a seed. Given a seed and a function `seed -> (element, new_seed) option`, unfold produces a list.

## OCaml Approach
- `Seq.unfold` (OCaml 4.14+) generates a `Seq`
- Manual recursive unfold for lists
- Seed function returns `Some (value, next_seed)` or `None`

## Rust Approach
- `std::iter::from_fn(|| ...)` — stateful closure
- `std::iter::successors(seed, |prev| ...)` — explicit unfold
- Custom iterator with state struct

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Unfold | `Seq.unfold f seed` | `successors(seed, f)` |
| Stateful gen | Manual closure | `from_fn(\|\| ...)` |
| Laziness | `Seq` is lazy | Iterators are lazy |
| Termination | Return `None` | Return `None` |
