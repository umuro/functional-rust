📖 **[View on hightechmind.io →](https://hightechmind.io/rust/910-iterator-find-map)**

---

# 910-iterator-find-map — Iterator find_map

## Problem Statement

A common pattern: try a transformation on each element, take the first success, ignore failures. Parsing the first valid integer from a list of strings, finding the first `key=value` pair in a config, finding the first element longer than a threshold — all follow this pattern. The naive approach uses `filter_map(f).next()`, but `find_map(f)` expresses the intent more directly: "find the first element for which f returns Some." OCaml's `List.find_map` was added in 4.10. Haskell's `Data.Maybe.mapMaybe` and `listToMaybe . mapMaybe f` serve the same role. It is the "optional value from the first successful transformation" operation.

## Learning Outcomes

- Use `.find_map(f)` to find the first `Some(...)` result in one pass
- Recognize it as more expressive than `.filter_map(f).next()`
- Apply find_map to parsing, searching, and pattern matching in sequences
- Implement the recursive OCaml-style `find_map_rec`
- Distinguish from `.find(pred)` (predicate, returns element) vs `.find_map(f)` (transform, returns transformed value)

## Rust Application

`first_int` uses `strings.iter().find_map(|s| s.parse::<i32>().ok())` — parses each string, returns the first success. `first_long_len` uses `find_map(|s| if s.len() > min_len { Some(s.len()) } else { None })`. `first_kv` uses `find_map(|s| s.split_once('='))` to find the first `key=value` pair and split it simultaneously. `find_map_rec` implements the recursive version: `f(head).or_else(|| find_map_rec(tail, f))`.

## OCaml Approach

`List.find_map: ('a -> 'b option) -> 'a list -> 'b option` (since 4.10) is the direct equivalent. Before 4.10: `let find_map f xs = match List.filter_map f xs with [] -> None | x :: _ -> Some x` (inefficient) or recursive manual implementation. `List.find_opt: ('a -> bool) -> 'a list -> 'a option` is the simpler case (no transformation). For sequences: `Seq.find_map` is available in OCaml 5.1+.

## Key Differences

1. **Standard library presence**: `find_map` was added to OCaml in 4.10; Rust has had it since 1.30. Both are now standard.
2. **filter_map + next**: Rust `filter_map(f).next()` is equivalent but less intent-revealing than `find_map(f)`; same tradeoff in OCaml.
3. **Short-circuit**: Both short-circuit — they stop at the first `Some` result without evaluating the rest.
4. **find vs find_map**: Rust `.find(pred)` returns `Option<&T>` (the element); `.find_map(f)` returns `Option<U>` (the transformed value) — more general.

## Exercises

1. Implement `find_valid_config(sources: &[&str]) -> Option<Config>` using `find_map` to try each source and return the first successfully parsed config.
2. Write `first_match_group<'a>(patterns: &[Regex], text: &'a str) -> Option<&'a str>` using `find_map` to return the first regex match.
3. Implement `resolve_path(dirs: &[&Path], filename: &str) -> Option<PathBuf>` using `find_map` that searches directories for the file.
