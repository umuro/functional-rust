📖 **[View on hightechmind.io →](https://hightechmind.io/rust/157-choice-parser)**

---

# Choice Parser

## Problem Statement

Grammars have alternatives: a value is either a number, a string, a boolean, or null. The `alt` (or `choice`) combinator tries parsers in order, returning the first success. If all fail, it returns the last error. This ordered choice is the basis for all ambiguity resolution in parsing — the order of alternatives determines precedence. Getting alternatives right (trying most specific before most general) is key to correct parsers.

## Learning Outcomes

- Understand `alt`/`choice` as ordered alternative (PEG-style, not CFG-style)
- Learn why order matters: `choice([tag("if"), identifier])` vs. `choice([identifier, tag("if")])`
- See how backtracking enables alternatives to recover from failed branches
- Practice building a boolean parser and a simple expression parser using `choice`

## Rust Application

`choice(parsers: Vec<Parser<T>>) -> Parser<T>` tries each parser on the same input. If a parser succeeds, its result is returned. If it fails without consuming input (the standard case in this simple implementation), the next parser is tried. If all fail, the last failure is returned. The implementation correctly restores `input` to its original value between attempts (backtracking). Ordering parsers from most to least specific avoids false matches.

## OCaml Approach

OCaml's angstrom uses `<|>` as the choice operator:
```ocaml
let value = int_parser <|> string_parser <|> bool_parser <|> null_parser
```
Angstrom's choice is greedy and does not backtrack by default — if the first parser consumes input and then fails, the alternative is not tried. This is a key difference from Parsec/Rust's simple implementation. Backtracking requires wrapping the parser in `try` / `option`.

## Key Differences

1. **Backtracking semantics**: Rust's simple `choice` always backtracks fully; angstrom's `<|>` does not backtrack after consuming input (requires explicit `option`/`try`).
2. **Error reporting**: On total failure, Rust returns the last error; angstrom returns a custom error combining all branch messages.
3. **Ordered vs. unordered**: Both treat alternatives as ordered (PEG semantics); neither attempts all alternatives simultaneously (CFG semantics).
4. **Precedence**: Higher alternatives shadow lower ones if they match the same prefix; this is intentional in PEG parsers for disambiguating grammars.

## Exercises

1. Write a `json_value_parser` using `choice` that handles `null`, `true`/`false`, numbers, and strings.
2. Demonstrate the ordering issue: show that `choice([identifier, tag("true")])` treats `"true"` as an identifier, while `choice([tag("true"), identifier])` parses it as a boolean.
3. Build an enum parser that maps each alternative to a Rust enum variant.
