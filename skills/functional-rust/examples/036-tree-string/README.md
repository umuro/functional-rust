# 036: Tree String

**Difficulty:** ⭐⭐  **Level:** Foundations

Serialize a binary tree to a string like `"a(b(d,e),c(,f))"` and parse it back.

## The Problem This Solves

Data structures live in memory. But memory is ephemeral — you need to save a tree to disk, send it over a network, or embed it in a config file. That requires serialization: converting the in-memory structure to a portable string.

This example implements a human-readable format: `a(left,right)`. A leaf node with no children is just its character. A node with children wraps them in parentheses: `a(b,c)`. An empty subtree is an empty string. Round-tripping — serialize then parse and get back the exact same tree — is the key test.

Real-world relevance: Newick format (phylogenetic trees in biology), S-expressions in Lisp, and many configuration formats use exactly this parenthetical nesting approach.

## The Intuition

Serialization and parsing are mirror images. Serialization descends the tree and builds up a string. Parsing reads left-to-right and reconstructs the tree.

**Serialize**: 
- `Leaf` → `""` (empty string)  
- `Node(l, 'a', r)` where both children are leaves → `"a"`  
- Otherwise → `"a(serialize(l),serialize(r))"`

**Parse** (recursive descent parser):
- Read a character — that's the current node's value.
- If the next char is `(`, parse left child, expect `,`, parse right child, expect `)`.
- If no `(`, it's a leaf node.
- If we're at `,` or `)` or end of string, return `Leaf`.

This is a classic *recursive descent parser* — one of the most important patterns in programming language implementation. Every JSON parser, every YAML parser, every SQL parser is built on this idea.

## How It Works in Rust

```rust
fn tree_to_string(tree: &Tree) -> String {
    match tree {
        Tree::Leaf => String::new(),  // empty = empty string

        Tree::Node(l, v, r) => match (l.as_ref(), r.as_ref()) {
            // Leaf node — just the character
            (Tree::Leaf, Tree::Leaf) => v.to_string(),

            // Internal node — wrap children in parens
            _ => format!("{}({},{})", v, tree_to_string(l), tree_to_string(r)),
        },
    }
}

fn parse(chars: &[char], pos: &mut usize) -> Tree {
    if *pos >= chars.len()
        || chars[*pos] == ','
        || chars[*pos] == ')' {
        return Tree::leaf();  // end of input or closing delimiter → Leaf
    }

    let v = chars[*pos];
    *pos += 1;

    if *pos < chars.len() && chars[*pos] == '(' {
        *pos += 1;  // skip '('
        let left = parse(chars, pos);
        // pos now points at ','
        *pos += 1;  // skip ','
        let right = parse(chars, pos);
        // pos now points at ')'
        *pos += 1;  // skip ')'
        Tree::node(left, v, right)
    } else {
        Tree::node(Tree::leaf(), v, Tree::leaf())  // no parens → leaf node
    }
}
```

**Example round-trip:**
```
Tree → "a(b(d,e),c(,f))" → Tree (identical)
```

The `&mut usize` position pointer advances as the parser consumes input — another example of threading mutable state through recursion.

## What This Unlocks

- **Serialization patterns**: the same serialize/parse symmetry applies to JSON, XML, and custom formats.
- **Recursive descent parsing**: this mini-parser is the foundation of every hand-written language parser.
- **Config files**: embed tree-structured data in plain text without a library.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String building | `String.concat` / `Printf.sprintf` | `format!()` macro |
| Mutable position in parser | `ref int` or return pair | `&mut usize` position pointer |
| Character access | `String.get` | `chars().collect::<Vec<_>>()` then index |
| Empty string | `""` | `String::new()` |
