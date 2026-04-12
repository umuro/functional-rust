📖 **[View on hightechmind.io →](https://hightechmind.io/rust/579-pattern-string-matching)**

---

# String Pattern Matching
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Many programs dispatch on string commands: REPL commands, configuration keys, protocol keywords, HTTP methods. While `HashMap` lookup handles large vocabularies, small fixed sets are more readable as `match` expressions. Rust supports matching on `&str` literals, combining with or-patterns and guards for commands that start with a prefix. This pattern is common in CLI tools, game loops, and simple interpreters.

## Learning Outcomes

- How `match s { "quit" | "exit" => ... }` matches string literal alternatives
- How guards `s if s.starts_with('/')` extend literal matching with predicates
- Why `match` on strings uses `&str` (not `String`) for pattern matching
- How to compare `match` vs `if/else` chains for string dispatch
- Where string match dispatch appears: CLI REPLs, command parsers, protocol handlers

## Rust Application

`classify_cmd(s: &str)` uses `"quit" | "exit" | "q" => "quit"` — or-pattern on string literals. `s if s.starts_with('/') => "command"` — guard for prefix matching. The pattern compiles to a series of equality comparisons (or a jump table for small sets). `classify_cmd_if` shows the equivalent `if/else` chain — more verbose, no exhaustiveness checking.

Key patterns:
- `"literal" => expr` — exact string match
- `"a" | "b" | "c" => expr` — or-pattern on strings
- `s if s.starts_with(prefix) => expr` — guard-based prefix match
- `"" => expr` — empty string pattern

## OCaml Approach

OCaml string matching is identical in syntax and semantics:

```ocaml
let classify_cmd s = match s with
  | "quit" | "exit" | "q" -> "quit"
  | "help" | "?" | "h" -> "help"
  | s when String.length s > 0 && s.[0] = '/' -> "command"
  | "" -> "empty"
  | _ -> "unknown"
```

## Key Differences

1. **&str vs string**: Rust matches on `&str` slices; OCaml matches on `string` values — both are the natural string type in their language.
2. **Prefix guards**: Both use guards for prefix matching — Rust `s.starts_with(...)`, OCaml `String.sub s 0 1 = "/"`.
3. **Exhaustiveness**: Both require `_` for strings since the domain is infinite.
4. **Performance**: Both compile string patterns to sequential comparisons — a `HashMap` is faster for large vocabularies.

## Exercises

1. **HTTP method dispatch**: Write `fn http_action(method: &str) -> &'static str` matching "GET", "POST", "PUT", "DELETE", "PATCH" with an `_ => "unsupported"` fallthrough.
2. **Prefix routing**: Implement `fn route(path: &str) -> &'static str` using guards for paths starting with "/api/", "/admin/", "/static/" — and exact matches for "/" and "/health".
3. **Case-insensitive**: Modify `classify_cmd` to handle uppercase variants using `s if s.eq_ignore_ascii_case("quit")` guards.
