[key-value-store on hightechmind.io](https://hightechmind.io/posts/functional-rust/key-value-store)

---

## Problem Statement

Implement a mutable key-value store backed by `HashMap<String, String>` with get/set/delete/contains/keys operations. The `impl Into<String>` pattern for `set` allows callers to pass `&str` or `String` without explicit conversion. Return sorted keys for deterministic output. Also implement an immutable functional variant using association lists.

## Learning Outcomes

- Build a `KvStore` struct wrapping `HashMap<String, String>` with CRUD methods
- Use `impl Into<String>` for ergonomic insertion: accepts both `&str` and `String` at call sites
- Implement `get` returning `Option<&str>` (borrowed from the map's stored value)
- Implement sorted `keys()` by collecting, sorting, and returning `Vec<&str>`
- Contrast with an immutable association-list approach: `Vec<(String, String)>` updated via functional operations

## Rust Application

```rust
pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore { data: HashMap::new() }
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.data.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn keys(&self) -> Vec<&str> {
        let mut keys: Vec<&str> = self.data.keys().map(|s| s.as_str()).collect();
        keys.sort();
        keys
    }
}
```

`impl Into<String>` on `set` parameters triggers an implicit `.into()` conversion. When the caller passes `"hello"` (`&str`), Rust calls `<&str as Into<String>>::into()` automatically. This eliminates `.to_string()` at every call site without adding `AsRef<str>` bounds.

`get` returns `Option<&str>` — a borrowed string slice from the map's heap-allocated `String`. The lifetime is implicitly tied to `&self`, so the returned `&str` cannot outlive the store.

`delete` returns `bool` by checking whether `.remove()` returned `Some`. This avoids a separate `.contains()` call followed by `.remove()` — single lookup.

## OCaml Approach

```ocaml
(* Mutable: Hashtbl *)
module KvStore = struct
  type t = (string, string) Hashtbl.t

  let create () = Hashtbl.create 16
  let set tbl k v = Hashtbl.replace tbl k v
  let get tbl k = Hashtbl.find_opt tbl k
  let delete tbl k = Hashtbl.remove tbl k
  let contains tbl k = Hashtbl.mem tbl k
  let keys tbl =
    Hashtbl.fold (fun k _ acc -> k :: acc) tbl []
    |> List.sort String.compare
end

(* Functional: association list *)
let set_al k v pairs = (k, v) :: List.filter (fun (k', _) -> k' <> k) pairs
let get_al k pairs = List.assoc_opt k pairs
let delete_al k pairs = List.filter (fun (k', _) -> k' <> k) pairs
```

OCaml's `Hashtbl` is a mutable hash table similar to Rust's `HashMap`. The functional association list approach creates new lists on every update — persistent but O(n) for all operations.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Hash table | `HashMap` — O(1) average | `Hashtbl` — O(1) average (mutable) |
| Ergonomic insertion | `impl Into<String>` | Direct `string` — no conversion needed |
| Functional variant | `Vec<(String, String)>` — O(n) lookup | Association list — same |
| Sorted keys | Manual `.sort()` | `List.sort String.compare` |
| Remove result | `Option<String>` — know if key existed | `unit` — `Hashtbl.remove` is void |

`HashMap` does not guarantee key order. Use `BTreeMap` when sorted iteration matters for correctness (e.g., when serializing to a deterministic format). The `keys()` method here sorts on each call — for read-heavy usage, consider maintaining a sorted `BTreeMap` instead.

## Exercises

1. Add a `merge(other: &KvStore)` method that copies all entries from `other`, with `other` winning on conflicts.
2. Implement `update<F: Fn(&str) -> String>(key: &str, f: F)` that applies a function to an existing value.
3. Add an event log: record each `set` and `delete` with a timestamp in a `Vec<(Instant, Event)>`.
4. Implement `prefix_keys(prefix: &str) -> Vec<&str>` that returns all keys starting with the given prefix.
5. Build a `persistent_kv_store` that serializes to/from a file using `serde_json` on `HashMap<String, String>`.
