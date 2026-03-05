# OCaml vs Rust: Serde Derive Concept

## What Derive Generates

### Rust
```rust
// You write:
#[derive(Serialize)]
struct Point { x: i32, y: i32 }

// The macro generates:
impl Serialize for Point {
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        serializer.serialize_struct_start("Point", 2);
        serializer.serialize_field("x");
        self.x.serialize(serializer);
        serializer.serialize_field("y");
        self.y.serialize(serializer);
        serializer.serialize_struct_end();
    }
}
```

### OCaml (ppx_deriving)
```ocaml
(* You write: *)
type point = { x: int; y: int } [@@deriving yojson]

(* The ppx generates: *)
let point_to_yojson p =
  `Assoc [
    ("x", `Int p.x);
    ("y", `Int p.y)
  ]
```

## Serializer Trait Pattern

### Rust
```rust
pub trait Serializer {
    fn serialize_i32(&mut self, v: i32);
    fn serialize_str(&mut self, v: &str);
    fn serialize_struct_start(&mut self, name: &str, len: usize);
    fn serialize_field(&mut self, name: &str);
    fn serialize_struct_end(&mut self);
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Macro system | PPX | Proc macros |
| Type-driven | ppx_deriving | serde derive |
| Output format | Specific (`yojson`) | Generic (Serializer trait) |
| Format agnostic | Multiple ppx needed | Single `Serialize` impl |
