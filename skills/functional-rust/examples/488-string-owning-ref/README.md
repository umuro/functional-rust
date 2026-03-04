# 488: Owning References with Rc\<str\>

**Difficulty:** 2  **Level:** Intermediate

Reference-counted immutable string slices — share string data in a single thread without cloning.

## The Problem This Solves

You have a string that multiple parts of your program need to read. The naive solution is to clone it everywhere — but now you have N copies in memory, and each clone is an allocation. The opposite extreme is passing `&str` references, which works but ties you to lifetimes that quickly become unwieldy in data structures.

`Rc<str>` sits in the middle. It's a reference-counted pointer to an immutable string slice: one heap allocation, multiple owners, no lifetime annotation required, no unnecessary copies. When the last `Rc<str>` drops, the string is freed.

Use `Rc<str>` when you're building trees, graphs, or caches in a single-threaded context that share string data — AST nodes sharing identifiers, a DOM sharing tag names, a symbol table in an interpreter.

## The Intuition

A shared document in a filing cabinet. Multiple employees hold a key (the `Rc`) to the same drawer (the string). They can all read it simultaneously. Nobody has their own copy. When the last employee hands back their key, the drawer is cleared. The cabinet tracks how many keys are outstanding.

## How It Works in Rust

1. **Create `Rc<str>` from a string literal or `String`**:
   ```rust
   use std::rc::Rc;
   let s: Rc<str> = "hello world".into();
   let s2: Rc<str> = String::from("hello world").into();
   ```
2. **Clone is cheap** — increments a counter, no string copy:
   ```rust
   let clone = s.clone(); // just bumps refcount
   assert_eq!(s, clone);
   ```
3. **Deref to `&str`** — use all `str` methods directly:
   ```rust
   println!("{}", s.to_uppercase()); // Rc<str> derefs to &str
   ```
4. **Embed in structs without lifetime parameters**:
   ```rust
   struct AstNode {
       name: Rc<str>,   // no 'a lifetime annotation needed
       children: Vec<AstNode>,
   }
   ```
5. **For thread-safe sharing** — use `Arc<str>` instead (see example 489).

## What This Unlocks

- **Shared ownership without cloning** — multiple data structures hold the same string bytes with one allocation.
- **Lifetime-free data structures** — no `<'a>` parameters propagating through every type that holds a string.
- **Cheap `Clone` derives** — `#[derive(Clone)]` on a struct with `Rc<str>` fields is cheap; no deep copies.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Shared string reference | Structural sharing via GC | `Rc<str>` with explicit refcount |
| Thread safety | GC handles it | `Rc` is single-thread only |
| Lifetime annotation | Not needed | Not needed (that's the point) |
| vs `String` | — | `Rc<str>`: shared + immutable; `String`: owned + mutable |
