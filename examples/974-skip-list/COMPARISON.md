# Skip List — Comparison

## Core Insight
A skip list is a sorted linked list with multiple levels of "skip" pointers. Inserting at a random height (O(log n) average) gives probabilistic O(log n) search. OCaml naturally models pointers as `'a node option` (nullable reference); Rust avoids unsafe raw pointers by using an arena (`Vec<Node>`) and integer indices — safe, cache-friendly, and avoids lifetime complexity.

## OCaml Approach
- `mutable forward: 'a node option array` — array of optional node pointers per level
- `Obj.magic ()` for uninitiated header value (unsafe but practical)
- Mutable `ref` cells for traversal: `let current = ref sl.header`
- `while !continue_ do ... done` pattern for conditional loops
- `Random.float 1.0` for probabilistic level generation
- Direct mutation: `update.(i).forward.(i) <- Some new_node`

## Rust Approach
- Arena `Vec<SkipListNode>` with `usize` index references (no raw pointers)
- Index `0` = header sentinel (always exists); `forward[i] == 0` means nil
- Custom `Rng` (xorshift) for deterministic testing
- `for i in (0..self.level).rev()` — iterate levels top to bottom
- `self.nodes[idx]` for node access by index
- No `unsafe` code needed — arena pattern solves the "many mutable pointers" problem

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Node pointers | `'a node option` | `usize` index into arena |
| Null/nil | `None` | Index `0` (header) |
| Arena | n/a (GC heap) | `Vec<SkipListNode>` |
| Mutation | `mutable` fields | `&mut self` |
| Traversal ptr | `let current = ref header` | `let mut current = 0usize` |
| RNG | `Random.float 1.0` | Custom xorshift Rng |
| Unsafe | `Obj.magic` for init | None required |
| Nil forward | `None` check | `next == 0` check |
