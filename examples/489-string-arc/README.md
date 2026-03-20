📖 **[View on hightechmind.io →](https://hightechmind.io/rust/489-string-arc)**

---

# Arc<str> for Shared Strings
**Difficulty:** ⭐  
**Category:** Functional Programming  


`Arc<str>` is a reference-counted fat pointer to an immutable string slice, enabling cheap cloning and safe sharing across threads without copying the string data.

## Problem Statement

When multiple threads or data structures need to share the same string without copying it, the choice is: `Arc<String>` (adds an unnecessary `String` wrapper layer), `Arc<str>` (direct reference-counted string slice), or `Rc<str>` (single-threaded). `Arc<str>` is the idiomatic choice for shared immutable strings: it allocates a single block containing both the reference count and the string bytes, implements `Send + Sync`, and derefs to `&str` for all standard string operations. It is used by Tokio, Actix, and many parser libraries to avoid copying strings across task/thread boundaries.

## Learning Outcomes

- Create `Arc<str>` from a `&str` with `Arc::from(s)`
- Clone an `Arc<str>` cheaply (only increments a reference count)
- Verify shared ownership with `Arc::ptr_eq`
- Send an `Arc<str>` to another thread via `thread::spawn`
- Understand that `Arc<str>` is a single allocation vs. `Arc<String>`'s double allocation

## Rust Application

Creating and sharing:

```rust
let s: Arc<str> = Arc::from("hello");
let s2 = Arc::clone(&s);
assert!(Arc::ptr_eq(&s, &s2));   // same allocation
```

Sending across thread boundary:

```rust
let s: Arc<str> = Arc::from("hello");
let s2 = Arc::clone(&s);
thread::spawn(move || assert_eq!(&*s2, "hello")).join().unwrap();
```

`Arc<str>` derefs to `&str`, so all `str` methods work directly:

```rust
let s: Arc<str> = Arc::from("hi");
assert_eq!(s.len(), 2);
```

## OCaml Approach

OCaml strings are automatically shared by the GC — no manual reference counting is needed. Multiple bindings to the same string value may or may not share the same physical memory (compiler-dependent), and the GC handles all lifetime management:

```ocaml
let s = "hello"
let s2 = s  (* may share physical memory *)

(* For explicit sharing with atomics in Multicore OCaml *)
let s_shared = Atomic.make "hello"
let s2 = Atomic.get s_shared
```

OCaml's `(==)` can check physical identity, but there is no guarantee two equal strings share memory unless you use a string interner (example 487).

## Key Differences

1. **Manual vs. automatic RC**: Rust requires explicit `Arc::clone` and `Arc::ptr_eq`; OCaml's GC tracks sharing automatically.
2. **`Send` proof**: Rust requires `Arc` (atomic reference count) for cross-thread sharing; `Rc` (non-atomic) is rejected by the compiler. OCaml's domain system handles this automatically.
3. **Single allocation**: `Arc<str>` stores refcount + bytes in one allocation; `Arc<String>` requires two (the `Arc` wrapper + `String`'s heap buffer). OCaml's GC block similarly stores the header + data together.
4. **Immutability**: `Arc<str>` is always immutable; for mutable shared strings, Rust requires `Arc<Mutex<String>>`. OCaml's `Atomic.make` provides a mutable shared reference.

## Exercises

1. **String cache**: Build a `struct StringCache(HashMap<u64, Arc<str>>)` that hashes strings and returns the existing `Arc<str>` for duplicate content, combining interning (example 487) with `Arc<str>` sharing.
2. **Message broker**: Implement a simple pub-sub where publishers send `Arc<str>` messages to N subscriber `mpsc::Sender` channels — measure that the message bytes are only allocated once regardless of subscriber count.
3. **`Arc<[u8]>` buffer pool**: Extend the concept to `Arc<[u8]>` for shared network packet buffers; implement a `BufferPool` that recycles buffers when the last consumer drops them.
