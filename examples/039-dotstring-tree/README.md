📖 **[View on hightechmind.io →](https://hightechmind.io/rust/039-dotstring-tree)**

---

# 039 — Convert a Tree to a Dotstring Representation

## Problem Statement

The dotstring representation of a binary tree (OCaml 99 Problems #39) uses a preorder traversal where leaves are represented by `.` characters. A node `x` with left subtree `l` and right subtree `r` becomes `x` followed by the dotstring of `l` then `r`. The self-delimiting property — you can parse the string left to right without needing parentheses — makes it efficient for both storage and streaming.

Dotstrings are used in compact tree serialization, hash computation of trees (trees with the same structure and values have the same dotstring hash), and in algorithms that transmit tree structure over byte channels. The format is equivalent to a Huffman-encoded tree stored as a bitstring prefix code.

## Learning Outcomes

- Produce the dotstring by combining preorder traversal with leaf markers
- Understand why dotstrings are self-delimiting (each position is consumed exactly once during parsing)
- Implement the inverse: parse a dotstring back to a tree using a position cursor
- Verify the round-trip invariant
- Distinguish from the parenthesized format of example 036

- Serialize trees as pre-order traversal with `.` for absent children — no delimiters needed between children
- Understand that the dotstring format is self-delimiting: the parser knows structure from the character stream

## Rust Application

`to_dotstring(tree: &Tree<char>) -> String`: `Leaf` → `"."`, `Node(c, l, r)` → `format!("{}{}{}", c, to_dotstring(l), to_dotstring(r))`. The parsing function `from_dotstring(s: &[char], pos: &mut usize) -> Tree<char>`: read `s[*pos]`, increment `*pos`; if `.` return `Leaf`; otherwise parse left subtree (advancing pos), then right subtree, return `Node(c, l, r)`.

## OCaml Approach

OCaml's version: `let rec to_dotstring = function | Leaf -> "." | Node (c, l, r) -> String.make 1 c ^ to_dotstring l ^ to_dotstring r`. Reconstruction: `let rec from_dotstring pos s = if s.[pos] = '.' then (Leaf, pos + 1) else let c = s.[pos] in let (l, p1) = from_dotstring (pos + 1) s in let (r, p2) = from_dotstring p1 s in (Node (c, l, r), p2)`. Returns `(tree, next_position)` pairs.

## Key Differences

1. **Cursor style**: Rust uses `&mut usize` position cursor (mutable reference). OCaml returns `(result, new_position)` pairs — the state-threading style that avoids mutation.
2. **Self-delimiting proof**: A dotstring is self-delimiting because: `.` consumes 1 character; a node character `c` consumes 1 + |left_dotstring| + |right_dotstring| characters, and both subtrees are self-delimiting by induction.
3. **Efficiency**: Both implementations are O(n) where n is the number of characters. String concatenation in OCaml with `^` inside recursion is O(n²) total; use `Buffer` for O(n).
4. **vs parenthesized**: The dotstring is more compact — no parentheses or commas needed. The parenthesized format is more human-readable.

1. **Dotstring format:** A tree is serialized as `"x.y.z.."` where each character is a node value and `.` represents a leaf. The format encodes the pre-order traversal, with leaves as dots.
2. **Serialization as pre-order:** The dotstring encoding processes nodes in pre-order (root, left, right). Reading the string back requires stateful parsing — consuming characters as you recurse.
3. **String vs Stream:** The serialization produces a `String`; the parser (example 040) consumes it as a `Chars` iterator. Threading the iterator through recursive calls is the key parsing pattern.

4. **Compact encoding:** The dotstring format uses no delimiters — it is self-delimiting because each node's children are exactly two successive tokens. This is unique to binary trees; n-ary trees would need delimiters.

## Exercises

1. **Dotstring hash**: Write `tree_hash(tree: &Tree<char>) -> u64` that computes a deterministic hash of the tree by hashing its dotstring. Two structurally equal trees must produce the same hash.
2. **Streaming decode**: Implement `DotStringDecoder` as an iterator that accepts one character at a time and emits `Node` or `Leaf` events as they are recognized. This enables streaming tree processing.
3. **Binary dotstring**: Convert the character-based dotstring to a bitstring format: `0` for Leaf, `1` followed by left/right for Node. Count bits needed vs character bytes.

4. **Compact format**: Design and implement an alternative serialization format that uses fewer characters — for example, using single characters for leaves and parentheses for internal nodes.
5. **Multi-character labels**: Extend the dotstring format to handle node labels with multiple characters by using a delimiter (e.g., `|` to separate the label from children). Implement both serialization and parsing.
