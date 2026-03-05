# Fallible Iterator — Comparison

## Core Insight
Iterators that can fail at each step need two layers: "are there more items?" (Option/Seq) and "did this item succeed?" (Result). Both languages layer these naturally.

## OCaml Approach
- `Seq` yields `Result` values: `Seq.t (('a, 'e) result)`
- Manual recursive processing with `Seq.Cons`/`Seq.Nil`
- Stateful iterator via mutable record
- No standard `FallibleIterator` abstraction

## Rust Approach
- `Iterator<Item = Result<T, E>>` — natural composition
- `collect::<Result<Vec<T>, E>>()` for short-circuit collection
- Custom `take_while_ok` / `process_all` for different strategies
- `fallible-iterator` crate for dedicated abstraction

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Type | `(T, E) result Seq.t` | `Iterator<Item=Result<T,E>>` |
| Laziness | `Seq` is lazy | `Iterator` is lazy |
| Short-circuit | Manual recursion | `collect()` or `?` in loop |
| All results | Manual fold | `partition` / custom |
| Stateful | Mutable record | `impl Iterator` with fields |
