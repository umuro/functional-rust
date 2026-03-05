# OCaml vs Rust: Deref Coercions

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml has no deref coercions — every conversion is explicit.
   You cannot pass a `bytes` where a `string` is expected. *)

let greet (name : string) = Printf.printf "Hello, %s!\n" name

let () =
  let s = "Alice" in
  greet s;                          (* only string literals / string values work *)
  let b = Bytes.of_string "Bob" in
  greet (Bytes.to_string b)         (* must convert explicitly *)
```

### Rust (idiomatic — coercions do the work)
```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn sum(data: &[i32]) -> i32 {
    data.iter().sum()
}

let s: String = String::from("Alice");
greet(&s);                           // &String → &str  (one coercion)

let boxed: Box<String> = Box::new(String::from("Bob"));
greet(&boxed);                       // &Box<String> → &String → &str  (two coercions)

let v: Vec<i32> = vec![1, 2, 3];
sum(&v);                             // &Vec<i32> → &[i32]  (one coercion)
```

### Rust (custom Deref — same mechanism for user types)
```rust
use std::ops::Deref;

struct MyVec<T>(Vec<T>);

impl<T> Deref for MyVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] { &self.0 }
}

fn sum_my_vec(v: &MyVec<i32>) -> i32 {
    sum(v)   // &MyVec<i32> coerces to &[i32] via Deref
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| String borrow | `string` (value, GC-managed) | `&str` (slice reference) |
| Owned string | `string` (immutable) | `String` (heap-owned) |
| Slice borrow | `'a array` | `&[T]` |
| Owned list | `'a list` | `Vec<T>` |
| Smart pointer | `'a ref` | `Box<T>`, `Rc<T>`, `Arc<T>` |
| Auto conversion | not available | `Deref` coercion (transitive) |

## Key Insights

1. **Explicit vs implicit conversion:** OCaml requires `Bytes.to_string`, `string_of_int`, etc. at every boundary. Rust's `Deref` trait lets the compiler insert conversions silently when the types are related by a deref chain.

2. **Transitive chains:** `&Box<String>` becomes `&str` in two steps — the compiler finds the shortest coercion path automatically. OCaml has no equivalent; you compose converters by hand.

3. **Write general functions once:** A Rust function taking `&str` or `&[T]` is automatically compatible with `String`, `Box<String>`, `Rc<String>`, `Vec<T>`, `Box<Vec<T>>`, and any custom newtype that implements `Deref`. In OCaml you would need separate functions or a functor.

4. **Custom types join the club:** Implementing `Deref` for a newtype wrapper makes it transparently usable wherever the inner type is expected — no trait objects or conversion helpers needed.

5. **No runtime cost:** Deref coercions are purely compile-time. The emitted machine code is identical to writing the dereference by hand; there is no boxing, virtual dispatch, or allocation.

## When to Use Each Style

**Use idiomatic Rust (`&str` / `&[T]` parameters) when:** you want callers to freely pass any owning or borrowing form of the data — this is the normal case for library functions.

**Use custom `Deref` when:** you have a newtype wrapper that should behave like its inner type in most contexts (e.g., `MyVec`, `Path`, `OsStr`).
