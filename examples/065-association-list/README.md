📖 **[View on hightechmind.io →](https://hightechmind.io/rust/065-association-list)**

---

# 065 — Association List

## Problem Statement

An association list (alist) is the simplest possible key-value store: a list of `(key, value)` pairs where lookup scans from the front and new entries are prepended (shadowing older entries). It is O(1) insert, O(n) lookup, but correct and simple. OCaml and Lisp use alists extensively for environments (variable → value mappings) in interpreters.

Alists are the foundation of lexical scoping: when a new binding is added with `insert`, it shadows the old binding for the same key — exactly how local variable shadowing works in interpreters. They appear in config overlays, feature flag systems, and small-map scenarios where `HashMap` overhead is not justified (< 10 entries).

## Learning Outcomes

- Implement `insert`, `lookup`, and `remove` on `Vec<(K, V)>` using functional patterns
- Understand that prepend (O(1)) shadows existing keys without removing them
- Use `iter().find_map()` for declarative lookup
- Understand the shadowing semantics: most recently inserted key wins
- Compare alist with `HashMap` and `BTreeMap` for different use cases

- Implement `Vec<(K, V)>` as an association list with O(n) lookup via `iter().find(|(k, _)| k == key)`
- Understand when alists outperform `HashMap`: small collections, non-hashable keys, ordered insertion required

## Rust Application

`insert` prepends to a `Vec<(K, V)>` — the new pair goes at index 0, shadowing any existing pair with the same key. `lookup` uses `iter().find(|(key, _)| key == k).map(|(_, v)| v)` — returns a reference to the first matching value. `remove` filters out the first occurrence. `lookup_iter` uses `find_map` for a more idiomatic variant.

## OCaml Approach

OCaml's association list: `let insert k v lst = (k, v) :: lst`. Lookup: `List.assoc k lst` (raises `Not_found`) or `List.assoc_opt k lst` (returns `option`). Remove: `List.remove_assoc k lst`. These are all built into OCaml's standard library. The `List.assoc_opt` function is a recent addition (4.05+); older code uses `List.assoc` with `try/with`.

## Key Differences

1. **Built-in support**: OCaml has `List.assoc_opt`, `List.assoc`, `List.remove_assoc` in stdlib. Rust has no alist module; you implement it from `Vec` or use `HashMap`.
2. **Shadowing semantics**: Both implement shadowing by prepending. OCaml's `List.assoc` finds the first (most recent) match. Rust's `find` does the same.
3. **Ownership**: Rust's `lookup` returns `Option<&V>` — a reference tied to the alist's lifetime. OCaml returns the value directly (GC handles lifetime).
4. **Performance crossover**: Alist lookup is O(n). For n > ~10 keys, `HashMap` is faster. For n < 5, alist avoids hashing overhead. The right choice depends on usage patterns.

1. **List vs `HashMap`:** Association lists (alists) are O(n) for lookup. `HashMap` is O(1) amortized. Alists are used for small collections, configuration (order matters), and when keys are not hashable.
2. **`Vec<(K, V)>` in Rust:** The Rust equivalent of an association list is `Vec<(K, V)>`. Lookup: `alist.iter().find(|(k, _)| k == &key).map(|(_, v)| v)`. OCaml: `List.assoc key alist`.
3. **First match wins:** Both OCaml's `List.assoc` and a linear scan return the first matching key. This allows "shadowing" — adding a new binding at the front overrides an old one.
4. **`HashMap` vs alist:** Use `HashMap` when the collection is large or lookup frequency is high. Use alist when the collection is small (< 10 items), insertion order matters, or when using it as an immutable persistent map.

## Exercises

1. **Env interpreter**: Use an alist as the environment in a simple variable-substitution interpreter. `eval(env: &[(String, i32)], expr: &Expr) -> i32` where `Expr` has `Var(String)` and `Const(i32)`.
2. **Update vs shadow**: Write `update(k: K, v: V, list: Vec<(K, V)>) -> Vec<(K, V)>` that replaces the value for an existing key rather than shadowing. Use `iter().map()` to transform matching pairs.
3. **To HashMap**: Write `to_hashmap<K: Eq + Hash, V: Clone>(alist: &[(K, V)]) -> HashMap<K, V>` that converts the alist to a HashMap. Note: earlier entries shadow later ones in the alist, so process in reverse order when inserting into the HashMap.

4. **Merge alists**: Implement `merge_alists<K: Eq + Clone, V: Clone>(a: &[(K, V)], b: &[(K, V)]) -> Vec<(K, V)>` that merges two association lists, with entries from `a` shadowing entries from `b` for duplicate keys.
5. **Alist to HashMap**: Implement `alist_to_hashmap<K: Eq + Hash, V>(alist: Vec<(K, V)>) -> HashMap<K, V>` and `hashmap_to_alist<K: Clone, V: Clone>(map: &HashMap<K, V>) -> Vec<(K, V)>`. Note that the alist-to-map conversion discards all but the last value for duplicate keys.
