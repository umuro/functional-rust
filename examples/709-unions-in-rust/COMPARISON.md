# OCaml vs Rust: Unions / Tagged Unions

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml: algebraic variants ARE safe tagged unions.
   The compiler tracks the discriminant and guarantees exhaustive matching. *)

type value =
  | Int   of int
  | Float of float
  | Bool  of bool

let describe (v : value) : string =
  match v with
  | Int   n -> Printf.sprintf "Int(%d)"   n
  | Float f -> Printf.sprintf "Float(%g)" f
  | Bool  b -> Printf.sprintf "Bool(%b)"  b

let size_of_value (v : value) : int =
  match v with
  | Int   _ -> 8
  | Float _ -> 8
  | Bool  _ -> 1

let () =
  let vals = [Int 42; Float 3.14; Bool true; Int (-7)] in
  List.iter (fun v ->
    Printf.printf "%s (size=%d)\n" (describe v) (size_of_value v)
  ) vals
```

### Rust — idiomatic enum (OCaml-equivalent)
```rust
/// Idiomatic Rust: the compiler generates the tag and dispatch for you.
#[derive(Debug, Clone, PartialEq)]
pub enum ValueEnum {
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl ValueEnum {
    pub fn describe(&self) -> String {
        match self {
            ValueEnum::Int(n)   => format!("Int({n})"),
            ValueEnum::Float(f) => format!("Float({f})"),
            ValueEnum::Bool(b)  => format!("Bool({b})"),
        }
    }
}
```

### Rust — explicit tagged union (raw `union` + enum tag)
```rust
#[repr(C)]
union RawValue {
    int_val:   i64,
    float_val: f64,
    bool_val:  u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag { Int, Float, Bool }

pub struct Value {
    tag:  Tag,
    data: RawValue,
}

impl Value {
    pub fn int(n: i64) -> Self {
        Value { tag: Tag::Int, data: RawValue { int_val: n } }
    }

    pub fn as_int(&self) -> Option<i64> {
        if self.tag == Tag::Int {
            // SAFETY: tag confirmed, int_val is the active field.
            Some(unsafe { self.data.int_val })
        } else {
            None
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust (enum) | Rust (raw union) |
|---------|-------|-------------|-----------------|
| Variant type | `type value = Int of int \| Float of float \| Bool of bool` | `enum ValueEnum { Int(i64), Float(f64), Bool(bool) }` | `union RawValue { int_val: i64, float_val: f64, bool_val: u8 }` |
| Accessor | pattern match | pattern match | `unsafe { union.int_val }` guarded by tag |
| Tag tracking | implicit (compiler) | implicit (compiler) | explicit `enum Tag` field |
| Safety | always safe | always safe | requires `unsafe` |
| C-ABI compatible | no | no | yes (with `#[repr(C)]`) |

## Key Insights

1. **OCaml variants = tagged unions under the hood.** Every OCaml algebraic type is represented as a tag word plus a payload. The compiler manages both invisibly; you only see safe pattern matching.

2. **Rust `enum` is the idiomatic equivalent.** For almost all Rust code, `enum` is the right choice — the compiler handles the tag, guarantees exhaustive matching, and the code is always safe.

3. **Raw `union` exists for C interop.** When you need a `repr(C)` struct that maps byte-for-byte to a C `union` definition, you use Rust's `union`. Every field access requires `unsafe` because the compiler cannot know which field is live.

4. **The safe-wrapper pattern.** Pair the raw `union` with an enum discriminant in an outer struct and expose `Option`-returning methods. All `unsafe` stays inside these methods; callers never see it. This is the Rust analogue of what OCaml's runtime does automatically.

5. **Memory layout control.** `#[repr(C)]` unions guarantee a specific layout, enabling zero-cost FFI with C libraries that use `union` fields — something OCaml variants cannot provide directly.

## When to Use Each Style

**Use `enum` (idiomatic Rust) when:** you are writing pure Rust and need a type-safe sum type. This is the default and the right choice 99 % of the time.

**Use raw `union` when:** you are writing FFI bindings that must match a C `union` layout exactly, or building low-level data structures (e.g., a JIT compiler's value representation) where you need to control every byte of memory and are prepared to manage the tag yourself.
