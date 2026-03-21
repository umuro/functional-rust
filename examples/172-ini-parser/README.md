📖 **[View on hightechmind.io →](https://hightechmind.io/rust/172-ini-parser)**

---

# INI File Parser
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

INI files are used for configuration in Windows, Python's `configparser`, Git's `.gitconfig`, and countless other tools. The format has sections `[name]`, key-value pairs `key = value`, comments `; ...` or `# ...`, and optional whitespace. Building an INI parser demonstrates real-world parsing with multiple line types requiring different handling, and produces a structured result (`HashMap<String, HashMap<String, String>>`) directly usable by applications.

## Learning Outcomes

- Parse a multi-line format with heterogeneous line types (section headers, key-value pairs, comments, blanks)
- Learn how `choice` selects the correct parser for each line type
- See how parsing produces structured output: a map of section names to key-value maps
- Practice combining all previously learned combinators into a complete, useful parser

## Rust Application

The parser handles four line types: `[section_name]` (section header), `key = value` (assignment), `; comment` or `# comment` (comment, discarded), and blank lines (ignored). Each line type has its own parser. The top-level parser applies `many0(choice([section_parser, comment_parser, blank_parser, key_value_parser]))` and builds a `Vec<IniSection>`. Whitespace is trimmed from keys and values. The result converts to `HashMap<String, HashMap<String, String>>`.

## OCaml Approach

OCaml's `Str` module provides regex-based line parsing — a simpler approach:
```ocaml
let section_re = Str.regexp "\\[\\([^]]+\\)\\]"
let kv_re = Str.regexp "\\([^=]+\\)=\\(.*\\)"
```
This is less composable but practical for simple INI files. Combinator-based OCaml INI parsers exist but are less common than regex-based approaches for configuration file parsing.

## Key Differences

1. **Line-oriented vs. token-oriented**: INI files are fundamentally line-oriented; the combinator approach handles this by including newline matching in each line parser.
2. **Comments**: Both discard comment lines; combining comment skipping with whitespace skipping produces a clean value stream.
3. **Multi-line values**: Some INI variants support continuation lines (`value = ...\n  continuation`); neither basic parser handles this.
4. **Encoding**: Both parse UTF-8; section names and keys with non-ASCII characters require explicit allowance in the character predicates.

## Exercises

1. Add support for `#`-prefixed comments in addition to `;`-prefixed ones.
2. Handle inline comments: `key = value  ; this is a comment` where the value is everything before the `;`.
3. Implement a global section for key-value pairs before the first `[section]` header.
