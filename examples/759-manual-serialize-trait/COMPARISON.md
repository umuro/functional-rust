# OCaml vs Rust: Manual Serialize Trait

## Trait Definition

### Rust
```rust
pub trait Serialize {
    fn serialize(&self, out: &mut Output);
    
    fn to_bytes(&self) -> Vec<u8> {
        let mut out = Output::new();
        self.serialize(&mut out);
        out.into_bytes()
    }
}
```

### OCaml
```ocaml
module type SERIALIZE = sig
  type t
  val serialize : t -> Buffer.t -> unit
  val to_bytes : t -> bytes
end
```

## Implementing for Types

### Rust
```rust
impl Serialize for u32 {
    fn serialize(&self, out: &mut Output) {
        out.write_u32(*self);
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self, out: &mut Output) {
        out.write_u32(self.len() as u32);
        for item in self {
            item.serialize(out);
        }
    }
}
```

### OCaml
```ocaml
let serialize_int32 n buf =
  Buffer.add_int32_le buf n

let serialize_list serialize_elem lst buf =
  serialize_int32 (Int32.of_int (List.length lst)) buf;
  List.iter (fun x -> serialize_elem x buf) lst
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Polymorphism | Functions | Traits |
| Generic impl | Manual | `impl<T: Trait>` |
| Buffer | `Buffer.t` | Custom `Output` |
| Endianness | `add_int32_le` | `to_le_bytes()` |
