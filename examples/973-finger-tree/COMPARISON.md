# Finger Tree — Comparison

## Core Insight
A finger tree stores digits (1-4 elements) at each end and a spine of `Node` elements. When digits overflow (4 items), they're packed into `Node3` and pushed into the spine — which is itself a `FingerTree<Node<T>>`. This nesting (`FingerTree<Node<FingerTree<Node<...>>>>`) is the key insight. OCaml handles this type-level recursion implicitly; Rust requires `Box` at each recursive level to bound the size.

## OCaml Approach
- `type 'a finger_tree = Empty | Single of 'a | Deep of 'a digit * 'a node finger_tree * 'a digit`
- The spine `'a node finger_tree` is a `finger_tree` parameterized with `node`
- No explicit boxing — OCaml values are pointer-sized by default
- Pattern matching with `function` sugar
- `push_front (Node3 (b,c,d)) spine` — recurse into spine with packed nodes

## Rust Approach
- `FingerTree<T>` with `Box<FingerTree<Node<T>>>` for spine
- `Box` required to give the recursive type a known size
- Each method call on spine is typed as `FingerTree<Node<T>>` — different type parameter
- `match *l` — deref Box to match inner Digit
- Consuming `self` in `push_front`/`push_back` (value semantics, functional style)

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Recursive type param | `'a node finger_tree` | `Box<FingerTree<Node<T>>>` |
| Boxing | Implicit (GC) | Explicit `Box` |
| Spine type | `'a node finger_tree` | `FingerTree<Node<T>>` |
| Pattern match Box | n/a | `match *l { Digit::One(a) => ... }` |
| Push into spine | `push_front (Node3 (b,c,d)) spine` | `spine.push_front(Node::Node3(b,c,d))` |
| Value vs reference | Return new value | Consume `self`, return new value |
| to_list | `digit_to_list @ node_to_list @ ...` | `Vec` + extend |
