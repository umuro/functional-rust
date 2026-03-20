📖 **[View on hightechmind.io →](https://hightechmind.io/rust/040-dotstring-parse)**

---

# 040 — Parse a Dotstring Back to a Binary Tree
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Parsing a dotstring back to a binary tree (OCaml 99 Problems, complement to #39) is the deserialization half of the round-trip. It is also a textbook example of recursive descent parsing: each call to the parse function consumes exactly one self-delimiting unit (either a `.` for leaf or a character followed by two recursive calls for a node).

Recursive descent parsers are the basis for most hand-written language parsers (JSON parsers, expression evaluators, configuration file readers). The self-delimiting property of dotstrings makes this one of the simplest examples of recursive descent — no lookahead, no backtracking, no ambiguity.

## Learning Outcomes

- Write a recursive descent parser that consumes characters left to right
- Use a `&mut usize` position cursor to track how much input has been consumed
- Handle the single-character lookahead needed to distinguish `.` from node values
- Implement error handling for malformed dotstrings
- Connect this parser structure to JSON/XML parsers and formal language grammars

## Rust Application

`parse_dotstring(chars: &[char], pos: &mut usize) -> Result<Tree<char>, String>`: check bounds, read `chars[*pos]`, increment `*pos`. If the character is `.`, return `Ok(Tree::Leaf)`. Otherwise it is a node value; parse left subtree recursively (which may fail), then parse right subtree, then return `Ok(Tree::node(c, left, right))`. Error cases: unexpected end of input, or invalid character.

## OCaml Approach

OCaml's functional version returns `(Tree<char>, int)` pairs: `let rec parse s pos = if pos >= String.length s then failwith "unexpected end" else let c = s.[pos] in if c = '.' then (Leaf, pos + 1) else let (l, p1) = parse s (pos + 1) in let (r, p2) = parse s p1 in (Node (c, l, r), p2)`. The position threads through all recursive calls as an explicit argument and return value.

## Key Differences

1. **Error handling**: Rust's `Result<Tree<char>, String>` makes parse errors explicit in the type system. OCaml's `failwith` raises an exception — callers must use `try...with` to catch it. A functional OCaml style would return `option` or `result`.
2. **Position threading**: Rust: `&mut usize` (single shared mutable reference). OCaml: return `(result, new_pos)` pairs (no mutation). Both are equivalent; OCaml's style is more testable.
3. **Composability**: The OCaml functional style composes naturally — you can build more complex parsers by sequencing `parse` calls. Rust with `&mut usize` also composes but requires managing the shared mutable state.
4. **Parser combinators**: Both approaches generalize to parser combinator libraries. Rust has `nom`, `pest`, `winnow`. OCaml has `angstrom`, `mparser`. The dotstring parser is the simplest case of what these libraries handle.

## Exercises

1. **Robust parser**: Add full error recovery: report the position of the error and the character that caused it. Return `Result<Tree<char>, ParseError>` where `ParseError` includes position and message.
2. **Parser combinator**: Rewrite the parser using a type `Parser<T> = impl Fn(&[char], usize) -> Result<(T, usize), String>`. Define `map`, `and_then`, and `or` combinators, then compose them to build the dotstring parser.
3. **Extended format**: Extend the parser to handle multi-character node values enclosed in brackets: `[abc]` for a node with value `"abc"`. Modify the grammar and the parser together.
