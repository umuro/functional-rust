# Partition Results — Comparison

## Core Insight
`collect()` short-circuits at the first error. When you want ALL successes AND all failures, you need `partition` — it processes every element.

## OCaml Approach
- `List.partition` with a predicate, then unwrap each side
- Fold-based approach accumulates into two lists
- Must reverse lists after fold (cons builds in reverse)

## Rust Approach
- `Iterator::partition(Result::is_ok)` splits into two `Vec<Result>`s
- Then `unwrap`/`unwrap_err` each side (safe because we just partitioned)
- `filter_map(Result::ok)` / `filter_map(Result::err)` for one-sided extraction
- Fold approach is also idiomatic

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Partition | `List.partition is_ok` | `iter.partition(Result::is_ok)` |
| Unwrap after | Manual pattern match | `Result::unwrap` (safe post-partition) |
| One-sided | `List.filter_map` | `filter_map(Result::ok)` |
| Performance | Two passes (partition + map) | Same |
| Use case | Collect all errors for reporting | Same |
