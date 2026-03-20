📖 **[View on hightechmind.io →](https://hightechmind.io/rust/152-char-parser)**

---

# Character Parsers

## Problem Statement

All parsers ultimately reduce to reading individual characters. Primitive character parsers — match this specific character, match any character, match a character not in this set, match any of these characters — form the atomic vocabulary from which all other parsers are constructed. Getting these primitives right (correct UTF-8 handling, informative error messages, correct remaining input slicing) is essential for building correct higher-level parsers.

## Learning Outcomes

- Implement the fundamental character parsers: `char_parser`, `any_char`, `none_of`, `one_of`
- Understand correct UTF-8 character slicing using `char::len_utf8()`
- Learn how error messages should name what was expected vs. what was found
- See how these primitives combine to form digit, letter, and alphanumeric parsers

## Rust Application

`char_parser(expected: char)` peeks at the first character of the input with `input.chars().next()`. On match, it advances by `c.len_utf8()` bytes (critical for correct multi-byte UTF-8 handling — not `+1`). `none_of(chars: Vec<char>)` matches any character not in the set. `one_of(chars: Vec<char>)` matches any in the set. Each returns `ParseResult<char>` — either the matched character and remaining input, or an error naming the expected input.

## OCaml Approach

In OCaml's `angstrom` library, `char 'a'` and `satisfy is_alpha` are the primitives. OCaml's `Uchar` module handles Unicode; `angstrom` internally works on `Bigstring` for performance. OCaml's `any_char` is `take 1`. The pattern matches are structurally identical to Rust's, but without lifetime annotations.

## Key Differences

1. **UTF-8 safety**: Rust must use `len_utf8()` to advance correctly; OCaml's string model (bytes or Uchar) similarly requires care at byte boundaries.
2. **Ownership**: Rust returns `&'a str` slices without copying; OCaml typically returns new strings or offsets into a buffer.
3. **Error messages**: Both should include the expected character name in the error; this is convention rather than enforcement in both languages.
4. **Performance**: Rust's `chars().next()` decodes one codepoint from UTF-8 efficiently; OCaml's equivalent is `String.get_utf_8_uchar`.

## Exercises

1. Implement `upper_case_char() -> Parser<char>` and `lower_case_char() -> Parser<char>` using `char::is_uppercase`.
2. Write `ascii_parser(c: char) -> Parser<char>` that panics at creation time if `c` is not ASCII (to catch programming errors early).
3. Benchmark parsing a 1MB string character-by-character using `any_char` and measure throughput.
