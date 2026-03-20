📖 **[View on hightechmind.io →](https://hightechmind.io/rust/165-keyword-parser)**

---

# Keyword Parser

## Problem Statement

Keywords like `if`, `else`, `while`, and `return` must be distinguished from identifiers. A naive string parser would match `"if"` at the start of `"ifstream"` — failing to recognize that `"ifstream"` is an identifier containing the prefix `"if"`. Keyword parsers enforce word boundary checking: the keyword must not be followed by an identifier character. This is the difference between lexer-level and parser-level thinking in grammar design.

## Learning Outcomes

- Understand the word boundary problem: why `tag("if")` alone is insufficient for keywords
- Implement `keyword` as `tag` followed by a negative lookahead (no identifier character)
- Learn how keyword parsers interact with identifier parsers in a complete lexer
- See the "longest match" rule as the standard resolution for keyword/identifier ambiguity

## Rust Application

`keyword(kw: &str) -> Parser<&str>` matches the exact string, then checks that the next character is not an identifier character (using `is_ident_char`). If the check fails — the keyword is followed by `'a'..='z'`, `'0'..='9'`, or `'_'` — the parser fails, leaving the input at the original position. This ensures `"ifstream"` is rejected as a keyword match while `"if "` and `"if("` succeed. The negative lookahead does not consume the following character.

## OCaml Approach

In OCaml's angstrom, the common pattern is:
```ocaml
let keyword kw =
  string kw *> peek_char >>= function
  | Some c when is_ident_char c -> fail ("expected end of keyword '" ^ kw ^ "'")
  | _ -> return kw
```
`peek_char` looks at the next character without consuming it, providing the lookahead. This is equivalent to Rust's approach and equally necessary for correct keyword parsing.

## Key Differences

1. **Lookahead**: Both use non-consuming lookahead (peek); the implementation detail is language-specific but the semantics are identical.
2. **Negative lookahead**: Both express "keyword not followed by identifier char"; Rust checks the remainder string; OCaml uses `peek_char`.
3. **Reserved words**: Some parsers pre-collect keywords into a `HashSet` and check identifiers against it; this is an alternative to individual keyword parsers.
4. **Error messages**: Failing keyword parsers should report "expected keyword 'X'"; reporting "expected identifier" would mislead the user.

## Exercises

1. Build a `keywords` combinator that tries a list of keywords in order: `keywords(["if", "else", "while"])`.
2. Write a parser that correctly distinguishes `"for"` (keyword) from `"formula"` (identifier) and test both.
3. Implement `reserved_word(words: &[&str]) -> impl Fn(&str) -> bool` as a fast lookup for keyword exclusion in identifier parsing.
