📖 **[View on hightechmind.io →](https://hightechmind.io/rust/118-deref-coercions)**

---

# Deref Coercions
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Rust has many smart pointer types — `String`, `Vec<T>`, `Box<T>`, `Arc<T>`, `Rc<T>` — each wrapping a more primitive type. Without automatic conversion, every function accepting `&str` would reject `&String`, forcing callers to write `.as_str()` everywhere. Deref coercions solve this by letting the compiler insert implicit dereferences: `&String` becomes `&str`, `&Vec<T>` becomes `&[T]`, transitively. This gives ergonomic APIs without sacrificing type safety.

## Learning Outcomes

- Understand the `Deref` trait and how the compiler applies coercions automatically
- Learn to write functions that accept the most general borrowed form (`&str`, `&[T]`)
- See how `Box<T>`, `Arc<T>`, and `Rc<T>` participate in coercion chains
- Implement `Deref` on a custom wrapper type to integrate with the coercion system

## Rust Application

The code shows `greet(name: &str)` accepting `&String`, `&Box<String>`, and `&Arc<String>` at call sites without explicit conversion. The compiler inserts `.deref()` calls as needed. `MyVec<T>` implements `Deref<Target = [T]>`, so `&MyVec<i32>` automatically coerces to `&[i32]` wherever slices are expected. The chain can be arbitrarily long: `&Box<String>` → `&String` → `&str`.

## OCaml Approach

OCaml does not have deref coercions — every type conversion is explicit. However, OCaml's module system and polymorphism reduce the need for them: a function taking `string` already accepts any `string` value directly without wrapping. OCaml does have subtyping for object types and variant inheritance via polymorphic variants, but these are unrelated to pointer coercions.

## Key Differences

1. **Implicitness**: Rust applies deref coercions automatically at argument positions; OCaml requires explicit conversion functions (e.g., `Bytes.to_string`).
2. **Ownership model**: Rust's coercions work on borrowed references (`&T`) — they never imply copying or ownership transfer.
3. **Custom integration**: Implementing `Deref` on a Rust newtype makes it participate in the coercion chain; OCaml has no equivalent mechanism.
4. **Transitive chains**: Rust follows the full chain (`&Box<Vec<u8>>` → `&Vec<u8>` → `&[u8]`); in OCaml all such conversions must be written explicitly.

## Exercises

1. Write a function `print_all(items: &[impl Display])` and call it with `&Vec<i32>` and `&[i32; 4]` without any explicit conversion.
2. Create a `LoggedVec<T>` newtype that implements `Deref<Target = Vec<T>>` and logs every access.
3. Demonstrate a three-step coercion chain: `&Box<String>` → `&String` → `&str` in a single call.
