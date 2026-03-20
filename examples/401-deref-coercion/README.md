📖 **[View on hightechmind.io →](https://hightechmind.io/rust/401-deref-coercion)**

---

# 401: Deref and Deref Coercions
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Rust's ownership system produces many wrapper types: `Box<T>`, `Arc<T>`, `String`, `Vec<T>`. Without deref coercions, using these types would require explicit unwrapping everywhere — `(*my_box).some_method()`, `(&my_string).as_str()`. The `Deref` trait and Rust's deref coercion rules automatically convert `&Box<T>` to `&T`, `&String` to `&str`, and `&Vec<T>` to `&[T]` when the compiler needs to. This makes functions accepting `&str` work seamlessly with `String`, `Box<String>`, `Arc<String>`, and any other type that dereferences to `str`.

Deref coercions are fundamental to `std`: they explain why `Box<T>` behaves like `T`, why `String` works where `&str` is expected, and how custom smart pointers integrate with the rest of the language.

## Learning Outcomes

- Understand the `Deref` trait and its `type Target` associated type
- Learn how Rust's deref coercion inserts `*` automatically at type boundaries
- See how implementing `Deref<Target = T>` makes a type behave like a pointer to `T`
- Understand the deref chain: `Arc<String>` → `String` → `str` through multiple coercions
- Learn `DerefMut` for mutable dereferences and its role in transparent mutation

## Rust Application

In `src/lib.rs`, `MyBox<T>` implements `Deref<Target = T>` by returning `&self.0`. With this, `*my_box` automatically gives `T`, and functions accepting `&T` accept `&MyBox<T>` through coercion. `string_length(s: &str)` accepts `&String`, `&Box<String>`, and `&MyBox<String>` all transparently. `DerefMut` provides the mutable dereference path. The coercion chain `&MyBox<String>` → `&String` → `&str` applies up to N dereferences automatically.

## OCaml Approach

OCaml has no automatic coercions — all conversions are explicit. `Buffer.contents buf` to get a string from a buffer, `String.to_bytes s` for byte conversion, etc. OCaml's objects support subtype coercion (`(obj :> base_class_type)`) but this is structural subtyping, not deref-based. The programmer writes explicit conversion functions where Rust would insert implicit deref coercions.

## Key Differences

1. **Implicit vs. explicit**: Rust inserts deref coercions automatically; OCaml requires explicit conversion at every call site.
2. **Chain depth**: Rust applies deref coercions transitively to arbitrary depth; OCaml's explicit conversions never chain automatically.
3. **Mutable deref**: Rust's `DerefMut` enables `*my_box = new_value` through the smart pointer; OCaml uses explicit setter functions or mutable record fields.
4. **Smart pointer integration**: Rust's deref system lets `Box`, `Arc`, `Rc`, `Mutex` guards, and custom types all "disappear" at use sites; OCaml smart pointers require explicit unwrapping.

## Exercises

1. **Custom smart pointer**: Implement a `Counted<T>` smart pointer that wraps `T` and counts how many times it has been dereferenced. Implement `Deref<Target = T>` and `DerefMut`. Write tests verifying the count increments.
2. **Coercion chain**: Write a function `fn bytes_length(s: &[u8]) -> usize`. Show that it accepts `Vec<u8>`, `Box<Vec<u8>>`, and a custom `ByteBuffer` implementing `Deref<Target = Vec<u8>>` through successive coercions.
3. **Rc-like pool**: Implement a simplified `Pool<T>` with `clone_ref` returning a `PoolRef<T>` that implements `Deref<Target = T>`. Use an `Arc<T>` internally for the value, demonstrating how smart pointers compose with deref.
