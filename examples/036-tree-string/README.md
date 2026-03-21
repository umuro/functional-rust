📖 **[View on hightechmind.io →](https://hightechmind.io/rust/036-tree-string)**

---

# 036 — Represent a Binary Tree as a String
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Serializing a binary tree to a string and back (OCaml 99 Problems #36) is a fundamental serialization problem. The format: `"a(b(d,e),c(,f))"` represents a tree where `a` is the root, with left child `b` (which has children `d` and `e`) and right child `c` (which has only right child `f`). Commas separate children; empty positions are left blank.

Tree serialization appears everywhere: JSON serialization of nested objects, XML document trees, S-expressions in Lisp (the most direct ancestor of this format), and protocol buffers. The round-trip invariant `parse(serialize(t)) == t` is the key correctness criterion.

## Learning Outcomes

- Serialize a `Tree<char>` to a string representation
- Parse the string back to a tree (recursive descent parsing)
- Use `String::with_capacity` and `write!` for efficient string building
- Implement a simple recursive descent parser with an index/cursor
- Verify the round-trip invariant: `from_str(to_str(t)) == t`

## Rust Application

`to_string(tree: &Tree<char>) -> String`: `Leaf` → `""`, `Node(c, l, r)` → format `"{c}({left},{right})"` where left and right are recursive calls. `from_str` is a recursive descent parser: read one character (the node value), then if `(` follows, parse left subtree, expect `,`, parse right subtree, expect `)`. Use a `&mut usize` index to track position in the input string.

## OCaml Approach

OCaml's version: `let rec to_string = function | Leaf -> "" | Node (c, l, r) -> let ls = to_string l and rs = to_string r in if ls = "" && rs = "" then String.make 1 c else Printf.sprintf "%c(%s,%s)" c ls rs`. The parser: consume a character, then if the next character is `(`, recursively parse left and right with comma between, then consume `)`.

## Key Differences

1. **String building**: Rust builds strings with `format!` macro or `String::push_str`. OCaml uses `Printf.sprintf` or `Buffer`. Both are efficient when used correctly.
2. **Parser state**: Rust passes `&mut usize` as a position cursor. OCaml can use a mutable reference or return `(tree, remaining_string)` from each parser function.
3. **Error handling**: Rust's parser should return `Result<Tree<char>, String>`. OCaml uses exceptions (`failwith "parse error"`) in imperative code or `option`/`result` in functional style.
4. **Leaf representation**: Empty string `""` for `Leaf` makes the format non-self-delimiting. The `(,)` markers tell the parser where subtrees begin and end.

## Exercises

1. **Generic tree string**: Generalize to `Tree<T>` where T implements `Display` and `FromStr`. The format becomes `"{value}({left},{right})"` with any displayable value.
2. **S-expression format**: Implement a Lisp-style format: `"(a (b d e) (c () f))"` where `()` represents a leaf. This is self-delimiting and easier to parse.
3. **JSON format**: Serialize the tree as JSON: `{"value": "a", "left": {...}, "right": null}`. Use `serde_json` for serialization and deserialization.
