# Collecting Results — Comparison

## Core Insight
Converting `[Result<T,E>]` to `Result<[T], E>` is a fundamental operation in both languages. Rust builds it into `collect()` via the type system; OCaml requires a manual combinator.

## OCaml Approach
- Write `sequence` or `traverse` manually (fold + reverse)
- No stdlib function for `Result list -> list Result` before external libs
- Must be explicit about short-circuit behavior
- Libraries like `Base` or `Lwt` provide this

## Rust Approach
- `iter.collect::<Result<Vec<T>, E>>()` — one line, built-in
- `FromIterator` trait impl handles the short-circuit
- `try_fold` for more control over accumulation
- Type inference usually figures out the target type

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Built-in | No (manual `sequence`) | Yes (`collect()`) |
| Short-circuits | Must implement | Automatic |
| Type inference | N/A | Drives `collect()` target |
| Traverse (map+collect) | Manual `traverse` | `.map(f).collect()` |
| Empty input | `Ok []` | `Ok(vec![])` |
