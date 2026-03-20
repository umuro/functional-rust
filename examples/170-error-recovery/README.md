📖 **[View on hightechmind.io →](https://hightechmind.io/rust/170-error-recovery)**

---

# Error Recovery

## Problem Statement

Production parsers must provide actionable error messages, not "parse error at offset 423." Users need to know what went wrong, where (line and column), and what was expected. Error recovery goes further: instead of stopping at the first error, the parser continues, collecting multiple errors. This requires tracking position (line/column from offset), enriching errors with expected-token information, and synchronizing on error tokens to resume parsing.

## Learning Outcomes

- Understand position tracking (offset, line, column) in a parser
- Learn how to enrich error messages with "expected X, got Y" information
- See the panic-mode error recovery strategy: skip to a synchronization token
- Appreciate why error quality is a significant differentiator in production parsers

## Rust Application

`Position { offset, line, col }` is threaded through the parser alongside the input. When a parser fails, it reports the position of the failure. `ParseError { position, expected, found }` provides structured error information. `skip_to_sync(input, pos, sync_chars)` advances past input until a synchronization character (`;`, `}`, newline) is found, allowing the parser to resume. Multiple errors are collected in a `Vec<ParseError>` rather than stopping at the first.

## OCaml Approach

OCaml's Menhir provides built-in error recovery via `error` tokens and recovery rules in the grammar. Hand-written OCaml parsers use `angstrom`'s `Consume.No` and position tracking via `pos` combinators. `angstrom` reports byte offsets; converting to line/column requires scanning the input up to the offset. Error reporting in production OCaml parsers (like `merlin`) tracks position via a state monad.

## Key Differences

1. **Position threading**: Rust passes `Position` as an explicit value; OCaml often uses a state monad or implicit position tracking in the input type.
2. **Error continuation**: Both languages' simple parsers stop at the first error; production parsers in both use panic-mode recovery or grammar-level `error` tokens.
3. **Error type**: Rust's `ParseError` is a structured value; OCaml's errors are typically strings or custom error types in the result type.
4. **Generator support**: Menhir provides automatic error recovery and localization; hand-written parsers in both languages must implement this manually.

## Exercises

1. Add a `collect_errors` wrapper that runs a parser and returns all errors found, continuing past each one using skip-to-semicolon recovery.
2. Implement line/column tracking by scanning the input up to the error offset and counting newlines.
3. Write an `expected` combinator that wraps any parser and adds a human-readable "expected X" to its error message.
