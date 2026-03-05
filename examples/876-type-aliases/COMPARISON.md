# Comparison: Type Aliases

## Simple Aliases

**OCaml:**
```ocaml
type user_id = int
type name = string
type user = { id : user_id; uname : name }
```

**Rust:**
```rust
type UserId = u64;
type Name = String;
struct User { id: UserId, name: Name }
```

## Generic Aliases

**OCaml:**
```ocaml
type 'a validator = 'a -> bool
type ('a, 'b) transform = 'a -> 'b
```

**Rust:**
```rust
type Validator<T> = fn(&T) -> bool;
type Transform<A, B> = fn(A) -> B;
```

## Complex Type Shorthand

**OCaml:**
```ocaml
type point = float * float
type polygon = point list
type 'a predicate = 'a -> bool
```

**Rust:**
```rust
type Point = (f64, f64);
type Polygon = Vec<Point>;
type Predicate<T> = Box<dyn Fn(&T) -> bool>;
```

## Aliases are Transparent (Both Languages)

**OCaml:**
```ocaml
type user_id = int
let x : user_id = 42
let y : int = x  (* OK — same type *)
```

**Rust:**
```rust
type UserId = u64;
let x: UserId = 42;
let y: u64 = x;  // OK — same type
```
