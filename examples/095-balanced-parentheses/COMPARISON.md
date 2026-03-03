# Comparison: Balanced Parentheses — OCaml vs Rust

## Core Insight

Both languages model this as a stack problem. OCaml's list-as-stack is natural because lists are the primary data structure. Rust's `Vec` is the stack, with `push`/`pop` replacing cons/pattern-match. The key Rust addition is `try_fold` — a fold that can short-circuit on failure, combining functional style with early exit.

## OCaml

```ocaml
let is_balanced s =
  let matching = function ')' -> '(' | ']' -> '[' | '}' -> '{' | _ -> ' ' in
  let rec check stack i =
    if i = String.length s then stack = []
    else match s.[i] with
    | '(' | '[' | '{' as c -> check (c :: stack) (i + 1)
    | ')' | ']' | '}' as c ->
      (match stack with
       | top :: rest when top = matching c -> check rest (i + 1)
       | _ -> false)
    | _ -> check stack (i + 1)
  in check [] 0
```

## Rust — try_fold

```rust
pub fn is_balanced_fold(s: &str) -> bool {
    let result = s.chars().try_fold(Vec::new(), |mut stack, c| {
        match c {
            '(' | '[' | '{' => { stack.push(c); Some(stack) }
            ')' | ']' | '}' => {
                let exp = match c { ')'=>'(', ']'=>'[', '}'=>'{', _=>unreachable!() };
                if stack.pop() == Some(exp) { Some(stack) } else { None }
            }
            _ => Some(stack),
        }
    });
    matches!(result, Some(s) if s.is_empty())
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Stack type | `char list` (immutable) | `Vec<char>` (mutable) |
| Push | `c :: stack` | `stack.push(c)` |
| Pop + check | `match stack with top :: rest when ...` | `stack.pop() == Some(expected)` |
| Early exit | Return `false` in recursion | `try_fold` returns `None` |
| Pattern matching | `as c` binds in or-pattern | Same syntax |

## Learner Notes

- **`try_fold`**: Rust's secret weapon — like `fold` but returns `None`/`Err` to stop early
- **List vs Vec**: OCaml's immutable list is safe but allocates on every push; Rust's Vec mutates in place
- **`matches!` macro**: Concise pattern matching for boolean checks — `matches!(x, Some(s) if s.is_empty())`
- **Guard patterns**: Both OCaml (`when`) and Rust (`if`) support guards in match arms
