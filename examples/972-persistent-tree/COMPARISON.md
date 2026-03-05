# Persistent BST — Comparison

## Core Insight
A persistent BST creates only O(log n) new nodes per operation — the path from root to the changed node. All unchanged subtrees are shared between the old and new versions. OCaml's GC makes this transparent; Rust uses `Rc::clone` on unchanged subtrees (O(1) pointer copy) and `Rc::new(subtree.insert(x))` for the new path.

## OCaml Approach
- `type 'a bst = Empty | Node of 'a bst * 'a * 'a bst` — clean recursive type
- `insert l x` creates new nodes on the path; unchanged branches reused by GC
- `Node (insert l x, v, r)` — `r` is shared (pointer, not copy)
- `delete`: `min_val r` to find in-order successor for node removal
- All operations return new trees; old trees remain valid and accessible
- No explicit sharing mechanism — GC handles it

## Rust Approach  
- `enum Bst<T> { Empty, Node(Rc<Bst<T>>, T, Rc<Bst<T>>) }` — Rc wraps children
- `Rc::clone(r)` — O(1) pointer copy, shares unchanged subtree
- `Rc::new(l.insert(x))` — allocates new node on changed path only
- `(**l).clone()` — deref Rc, then clone the Bst (needed for delete leaf case)
- `T: Ord + Clone` — needed for comparison and copying values into new nodes
- Both old and new trees valid as long as any Rc points to them

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Sharing mechanism | GC implicit | `Rc::clone` explicit |
| New node on path | `Node (insert l x, v, r)` | `Node(Rc::new(l.insert(x)), v.clone(), Rc::clone(r))` |
| Unchanged branch | Reused automatically | `Rc::clone(branch)` (O(1)) |
| Node lifetime | GC | Rc drops when last ref gone |
| Trait bounds | None (structural) | `T: Ord + Clone` |
| Delete successor | `min_val r; delete r m` | `r.min_val(); r.delete(&m)` |
| New nodes per op | O(log n) | O(log n) |
