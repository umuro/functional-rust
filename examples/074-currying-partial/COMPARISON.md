## Core Insight

In OCaml, `let add x y = x + y` is actually `let add = fun x -> fun y -> x + y`. Partial application (`add 5`) returns a function. Rust functions aren't curried — you use closures to simulate partial application.

## OCaml Approach
- All functions are automatically curried
- `let add x y = x + y` → `add 5` returns `fun y -> 5 + y`
- Free partial application of any prefix of arguments

## Rust Approach
- Functions are NOT curried
- Closures capture variables: `let add5 = |y| 5 + y;`
- `move` closures for ownership transfer
- Can return closures with `impl Fn(T) -> U`

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Curried by default | Yes | No |
| Partial application | `f x` (give fewer args) | Closure capturing |
| Return function | Automatic | `impl Fn(T) -> U` |
| Closure syntax | `fun x -> ...` | `\|x\| ...` |
