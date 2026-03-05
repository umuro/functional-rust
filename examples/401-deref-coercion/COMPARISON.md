# OCaml vs Rust: Deref Coercions

## Side-by-Side Code

### OCaml — Manual unwrapping (no auto-coercion)
```ocaml
module Box = struct
  type 'a t = { value: 'a }
  let create v = { value = v }
  let deref { value } = value
end

let use_string (s : string) =
  Printf.printf "String: %s\n" s

let () =
  let boxed = Box.create "hello" in
  use_string (Box.deref boxed)  (* Explicit deref required *)
```

### Rust — Automatic deref coercion
```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

fn use_str(s: &str) {
    println!("String: {}", s);
}

fn main() {
    let boxed = MyBox(String::from("hello"));
    use_str(&boxed);  // Auto-coerces: &MyBox<String> -> &String -> &str
}
```

---

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Automatic coercion | None — explicit unwrap required | Deref chain followed automatically |
| Smart pointer ergonomics | Must call accessor function | Methods resolve through deref chain |
| String types | Single `string` type | `String` (owned) / `&str` (borrowed) |
| Custom containers | Manual accessor pattern | Implement `Deref` trait |
| Coercion depth | N/A | Unlimited chain: `Box<Vec<String>>` → `&str` |
| Runtime cost | None (no coercion exists) | Zero — compile-time transformation |
| Mutability | Separate mutable wrapper | `DerefMut` for mutable access |

---

## The Deref Chain

```rust
// Compiler follows Deref impls until types match:
&Box<String>  →  &String  →  &str
    │              │           │
    └── Box::deref ┘           │
                   └── String::deref ┘
```

In OCaml, you'd need:
```ocaml
let inner = Box.deref (StringBox.deref boxed_string_box) in
use_string inner  (* Two explicit unwraps *)
```

---

## Key Differences

### 1. Automatic vs Explicit

**OCaml**: Every wrapper requires explicit unwrapping
```ocaml
let x = Box.deref (Box.create 42) in
print_int x
```

**Rust**: Compiler inserts derefs automatically
```rust
let x = MyBox(42);
println!("{}", *x);  // Or even: println!("{}", x); with Display
```

### 2. Method Resolution

**OCaml**: Methods don't "pass through" wrappers
```ocaml
let s = Box.create "hello" in
String.length (Box.deref s)  (* Must unwrap first *)
```

**Rust**: Methods resolve through deref chain
```rust
let s = MyBox(String::from("hello"));
s.len()  // Finds str::len() through MyBox -> String -> str
```

### 3. API Flexibility

**OCaml**: Functions must accept the exact type
```ocaml
let print_string (s : string) = ...
(* Cannot pass Box.t without unwrapping *)
```

**Rust**: Functions accepting `&str` work with many types
```rust
fn greet(s: &str) { ... }
greet(&String::from("hi"));     // &String -> &str
greet(&Box::new("hi".into()));  // &Box<String> -> &String -> &str
greet(&MyBox::new("hi".into())); // &MyBox<String> -> &String -> &str
```

---

## 5 Takeaways

1. **Deref coercion is implicit but predictable.**
   The compiler follows `Deref` implementations at compile time — no runtime
   dispatch, no hidden costs.

2. **OCaml's explicit style has benefits too.**
   No magic means no surprises. What you write is what you get.

3. **Custom smart pointers get full ergonomics in Rust.**
   Implement `Deref<Target=T>` and your type behaves like `&T` everywhere.

4. **The `&str` / `&[T]` pattern enables universal APIs.**
   Write functions accepting borrowed slices; they work with all owning types.

5. **Method resolution follows the deref chain.**
   `box.len()` where `box: MyBox<String>` finds `str::len()` automatically.
