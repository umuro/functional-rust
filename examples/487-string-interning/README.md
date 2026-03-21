📖 **[View on hightechmind.io →](https://hightechmind.io/rust/487-string-interning)**

---

# String Interning
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



String interning deduplicates equal strings by returning `Arc<str>` handles that share a single heap allocation, enabling O(1) equality comparison by pointer and reduced memory usage for repeated strings.

## Problem Statement

Compilers, parsers, and databases deal with many repeated string values: variable names, SQL column names, log message templates. Without interning, each copy of `"username"` is a separate heap allocation — wasting memory and requiring character-by-character comparison for equality. **Interning** maintains a global table of unique strings; the same content always returns the same pointer. This enables pointer equality (`ptr_eq`) instead of string comparison — O(1) vs. O(N) — and a 2–10× memory reduction for symbol-heavy workloads. LLVM's `StringRef`, Java's `String.intern()`, and Python's string interning all use this pattern.

## Learning Outcomes

- Build an `Interner` backed by `HashMap<String, Arc<str>>`
- Return `Arc<str>` as a cheap cloneable handle to an interned string
- Verify pointer identity with `Arc::ptr_eq` to confirm interning works
- Wrap the interner in `Mutex` for thread-safe shared access
- Understand `Arc<str>` as a fat-pointer reference-counted string without a `String` wrapper

## Rust Application

`Interner::intern` looks up the string in a `HashMap`; on a hit, it clones the `Arc` (cheap — increments a reference count); on a miss, it allocates `Arc::from(s)`, inserts it, and returns a clone:

```rust
pub fn intern(&mut self, s: &str) -> Arc<str> {
    if let Some(v) = self.table.get(s) { return Arc::clone(v); }
    let arc: Arc<str> = Arc::from(s);
    self.table.insert(s.to_string(), Arc::clone(&arc));
    arc
}
```

Two interns of the same string return pointers to the same allocation:

```rust
assert!(Arc::ptr_eq(&i.intern("hi"), &i.intern("hi")));
```

`SyncInterner` wraps `Interner` in `Mutex` for multi-threaded use.

## OCaml Approach

OCaml strings are immutable and GC-managed; the GC does not intern by default. A manual interner uses a `Hashtbl`:

```ocaml
let table : (string, string) Hashtbl.t = Hashtbl.create 64

let intern s =
  match Hashtbl.find_opt table s with
  | Some v -> v   (* same physical string *)
  | None -> Hashtbl.add table s s; s

(* Pointer equality *)
let same a b = a == b  (* physical equality in OCaml *)
```

OCaml's `(==)` is physical (pointer) equality; `(=)` is structural. Interning allows using `(==)` for string identity checks.

## Key Differences

1. **Reference counting**: Rust uses `Arc<str>` to track interned string lifetimes — the interner table holds one count, callers hold additional counts; OCaml's GC handles lifetime automatically.
2. **Pointer equality**: Rust uses `Arc::ptr_eq`; OCaml uses `(==)` (physical equality).
3. **Thread safety**: Rust's `SyncInterner` wraps in `Mutex`; OCaml requires a `Mutex` too, but the GC ensures no use-after-free even without one on reads.
4. **`Arc<str>` vs. `Arc<String>`**: `Arc<str>` is a fat pointer (address + length) with one allocation; `Arc<String>` would add a `String` wrapper layer unnecessarily.

## Exercises

1. **Weak references**: Replace the interner's `Arc<str>` values with `Weak<str>` so interned strings are freed when no external handles remain, turning the table into a cache rather than a permanent registry.
2. **Lock-free interner**: Replace `Mutex<Interner>` with a `DashMap<String, Arc<str>>` (from the `dashmap` crate) for concurrent interning without a global lock.
3. **Symbol type**: Wrap `Arc<str>` in a `newtype struct Symbol(Arc<str>)` and implement `PartialEq`/`Hash` using pointer identity rather than string content for O(1) hash map lookups.
