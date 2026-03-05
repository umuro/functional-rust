# Option to Result — Comparison

## Core Insight
Both languages need to bridge the gap between "absence" (Option/None) and "error" (Result/Error). The conversion adds semantic meaning to what was just a missing value.

## OCaml Approach
- Pattern match on `option` and return `Ok`/`Error`
- Custom `ok_or` and `ok_or_else` helpers (not in stdlib before 4.08)
- Lazy version uses `fun () -> error_value` thunk

## Rust Approach
- Built-in `Option::ok_or(err)` and `Option::ok_or_else(|| err)`
- Reverse: `Result::ok()` and `Result::err()` to get Options
- Chains naturally: `.ok_or_else(|| ...)?.and_then(...)`

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Option to Result | Custom helper or match | `.ok_or()` / `.ok_or_else()` |
| Result to Option | Custom helper | `.ok()` / `.err()` |
| Lazy error | `fun () -> ...` thunk | `\|\| ...` closure |
| Chaining | `\|>` pipeline | `.method()` chain |
| In stdlib | Since 4.08 (partial) | Always available |
