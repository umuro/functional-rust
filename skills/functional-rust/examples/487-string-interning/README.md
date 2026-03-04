# 487: String Interning

**Difficulty:** 2  **Level:** Intermediate

Cache identical strings so you store each unique value once and compare by pointer, not content.

## The Problem This Solves

In compilers, parsers, and data pipelines you often encounter the same string thousands or millions of times: variable names, field names, log levels, city names. Allocating a fresh `String` for each occurrence wastes memory and makes equality checks O(n) in string length.

String interning solves both problems. The intern table maps each unique string to a shared reference. The second time you see `"identifier"`, you get back the same pointer you got the first time. Equality check becomes pointer comparison — O(1). Memory usage drops to one copy per unique string.

This is how language runtimes store symbol tables, how databases store column names, and how many serialization formats handle repeated keys.

## The Intuition

A coin minting machine. Every request for a "quarter" returns the same physical coin from the vault, not a freshly minted one. Anyone holding a quarter knows it's the same value as any other quarter — they don't need to inspect the details, just compare the reference. The vault only grows when a truly new denomination is requested.

## How It Works in Rust

1. **Simple interner with `Arc<str>`**:
   ```rust
   use std::collections::HashMap;
   use std::sync::{Arc, Mutex};

   struct Interner(Mutex<HashMap<String, Arc<str>>>);

   impl Interner {
       fn intern(&self, s: &str) -> Arc<str> {
           let mut map = self.0.lock().unwrap();
           if let Some(interned) = map.get(s) {
               return interned.clone();
           }
           let arc: Arc<str> = s.into();
           map.insert(s.to_string(), arc.clone());
           arc
       }
   }
   ```
2. **Compare interned strings by pointer**:
   ```rust
   let a = interner.intern("hello");
   let b = interner.intern("hello");
   assert!(Arc::ptr_eq(&a, &b)); // same allocation
   ```
3. **Single-threaded** — swap `Mutex` for `RefCell` in non-concurrent contexts.
4. **Typed symbols** — wrap `Arc<str>` in a newtype for type safety:
   ```rust
   #[derive(Clone, PartialEq, Eq, Hash)]
   struct Symbol(Arc<str>);
   ```

## What This Unlocks

- **O(1) equality** — pointer comparison replaces string scan; hash maps keyed on interned strings run faster.
- **Memory efficiency** — a log processor that sees `"ERROR"` a million times stores it once.
- **Symbol tables** — the foundation for variable binding in interpreters and compilers.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Intern mechanism | `String.intern` (stdlib!) | Manual `HashMap<String, Arc<str>>` |
| Reference type | `string` (already internable) | `Arc<str>` or newtype wrapper |
| Thread safety | GC-managed | `Mutex` around intern table |
| Equality check | `==` (may intern automatically) | `Arc::ptr_eq` for pointer comparison |
