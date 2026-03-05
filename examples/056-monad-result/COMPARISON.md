## Core Insight

A monad is a type with `return` (wrap a value) and `bind` (chain operations that may fail). Result is a monad: `Ok` is return, `and_then`/`bind` chains fallible operations, short-circuiting on `Err`.

## OCaml Approach
- `Result.bind result f` — chains fallible functions
- `Result.map result f` — transforms the Ok value
- Manual pattern matching as alternative
- `let*` syntax with binding operators (OCaml 4.08+)

## Rust Approach
- `.and_then(f)` — monadic bind
- `.map(f)` — functor map
- `?` operator — desugar to match + early return
- Method chaining is idiomatic

## Comparison Table

| Operation | OCaml | Rust |
|-----------|-------|------|
| Return/wrap | `Ok x` | `Ok(x)` |
| Bind | `Result.bind r f` | `r.and_then(f)` |
| Map | `Result.map f r` | `r.map(f)` |
| Sugar | `let* x = r in ...` | `let x = r?;` |
| Short-circuit | Pattern match | `?` operator |
