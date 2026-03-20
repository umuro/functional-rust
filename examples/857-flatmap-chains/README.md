📖 **[View on hightechmind.io →](https://hightechmind.io/rust/857-flatmap-chains)**

---

# FlatMap / Bind Chains

## Problem Statement

Long sequences of fallible or absent-value operations create deeply nested code with `match` or `if let`. FlatMap (bind/and_then) chains linearize these sequences: each step receives the value from the previous step and returns a new wrapped value. The chain short-circuits on the first failure. This enables multi-step data processing pipelines — parse JSON, extract a field, convert the type, look it up in a database, format the result — written as readable flat code without the pyramid of doom. FlatMap chains are the foundation of Rust async/await (which desugars to state machines over flatmapped futures), Rust's `?` operator chains, and iterator `flat_map`.

## Learning Outcomes

- Chain `and_then` calls to build multi-step Option/Result pipelines
- Understand early exit: the first `None`/`Err` terminates the chain immediately
- Use `flat_map` on iterators: map each element to an iterator and flatten the result
- Compare `map` + `flatten` vs. direct `flat_map` — they are equivalent
- Recognize the pattern in async Rust: `future.await?.process()?.format()?`

## Rust Application

```rust
fn parse_json(s: &str) -> Option<&str> {
    if s.starts_with('{') { Some(s) } else { None }
}
fn extract_field<'a>(json: &'a str, field: &str) -> Option<&'a str> {
    json.find(field).map(|i| &json[i..])
}
fn process(s: &str) -> Result<String, String> {
    if s.len() > 5 { Ok(s.to_uppercase()) }
    else { Err("Too short".to_string()) }
}
// Multi-step chain: each step can fail independently
let result: Option<Result<String, String>> = 
    parse_json(r#"{"key": "value"}"#)
        .and_then(|json| extract_field(json, "key"))
        .map(|field| process(field));
```

The chain demonstrates chaining over `Option` then transitioning to `Result`. `and_then` is used when the function returns `Option<T>`; `map` when it returns `T` directly or wraps in a different monad. The nested `Option<Result<>>` type shows a real complexity: mixing monads requires `transpose()` or `flatten` to normalize. In practice, using `Result` throughout (with `Option::ok_or`) avoids this.

## OCaml Approach

OCaml chains with `>>=`: `parse_json s >>= extract_field "key" >>= fun field -> process field`. The transition from `option` to `result` uses `Option.to_result ~none:"missing"`. OCaml's `let*` do-notation (ppx_let): `let* json = parse_json s in let* field = extract_field "key" json in process field`. This reads like sequential imperative code while retaining the monadic structure. `List.concat_map` (the List equivalent of `flat_map`) flattens nested lists.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Chain syntax | `.and_then(...)` chain | `>>=` or `let*` |
| Mixed monads | `Option::ok_or` then `Result` chain | `Option.to_result ~none` |
| Iterator flatmap | `flat_map(f)` on `Iterator` | `List.concat_map` |
| Early exit | First `None`/`Err` terminates | Same |
| Async equivalent | `?.await?` chains | `lwt` `>>=` chains |
| Readability | Method chain | Pipe or do-notation |

## Exercises

1. Implement a complete data pipeline: read a file, parse each line as JSON, extract a numeric field, compute the sum — using `?` for error propagation.
2. Use `Iterator::flat_map` to expand a list of ranges into a flat list of integers.
3. Demonstrate that `.map(f).flatten()` is equivalent to `.flat_map(f)` with a concrete test.
4. Implement a `then_with` combinator that only executes the next step if the previous step succeeds, logging failures.
5. Write the same multi-step pipeline using explicit `match` statements and compare line count and readability.
