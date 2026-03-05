📖 **[View on hightechmind.io →](https://hightechmind.io/rust/584-pattern-type-ascription)**

---

# 584: Type Patterns and `as`

**Difficulty:** 3  **Level:** Intermediate

Annotate types in binding patterns, use `as` for numeric casts, and downcast through `Any` — Rust's approaches to type annotation and runtime type inspection.

## The Problem This Solves

Rust is statically typed, but there are cases where type information needs to be coerced, annotated explicitly, or inspected at runtime. Three scenarios come up regularly: narrowing a numeric type for a specific context (`i32` → `u8` for a byte), annotating a binding in a `let` to guide inference (`let x: i32 = compute()`), and dispatching on the concrete type behind a `dyn Any` trait object.

The `as` keyword handles numeric casts with defined semantics (truncation for narrowing). Type annotations in `let` bindings guide the type checker when inference is ambiguous. `Any::downcast_ref` provides safe runtime type inspection without transmute.

Understanding when each mechanism is appropriate — and what guarantees it provides — prevents subtle bugs like silent truncation of large integers or incorrect `Any` downcasts.

## The Intuition

`as` is explicit, always compiles for numeric types, and has well-defined behavior: widening preserves value, narrowing truncates, signed↔unsigned reinterprets bits. It does *not* panic — but `300i32 as u8` silently gives `44` (300 mod 256). Use it deliberately.

Type ascription in `let x: Type = ...` is just annotation — it tells the type checker what you intend, fails at compile time if the types don't match, and produces no runtime code.

`Any` downcasting is the runtime equivalent: `downcast_ref::<T>()` returns `Option<&T>` — `Some` if the concrete type is `T`, `None` otherwise. It's safe, but limited to `'static` types.

## How It Works in Rust

**Type annotation in binding pattern:**
```rust
let x: i32 = 300;
let y: f64 = x as f64;  // widen: 300 → 300.0
let z: u8  = x as u8;   // narrow: 300 → 44 (truncates)
```

**Enum dispatch with typed variants:**
```rust
enum Value { Int(i64), Float(f64), Str(String), Bool(bool) }

impl Value {
    fn to_f64(&self) -> Option<f64> {
        match self {
            Value::Int(n)   => Some(*n as f64),  // 'as' widens i64→f64
            Value::Float(f) => Some(*f),
            Value::Str(s)   => s.parse().ok(),   // fallible conversion
            _               => None,
        }
    }
}
```
`as` for numerics, `.parse()` for string-to-numeric, `From`/`Into` for semantic conversions.

**`Any` downcast — runtime type inspection:**
```rust
fn describe_any(v: &dyn Any) -> &'static str {
    if      v.downcast_ref::<i32>()   .is_some() { "i32" }
    else if v.downcast_ref::<f64>()   .is_some() { "f64" }
    else if v.downcast_ref::<String>().is_some() { "String" }
    else { "unknown" }
}
```
`downcast_ref::<T>` is `O(1)` — it compares `TypeId`s, not names.

**Key rule — prefer `From`/`Into` for semantic conversions:**
```rust
let s = String::from("hello");
let bytes: Vec<u8> = s.into_bytes();  // From — documented, no information loss
```
`as` for numerics, `From`/`Into` for types, `.parse()` for strings.

## What This Unlocks

- **Numeric type coercion** — express intent with `as`, knowing the truncation semantics exactly.
- **Plugin/dynamic type dispatch** — `dyn Any` + `downcast_ref` for heterogeneous collections without unsafe.
- **Type-guided inference** — annotate bindings to guide the compiler when multiple types would satisfy inference.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Numeric cast | `Int64.of_int`, `float_of_int` (explicit, named) | `x as f64`, `x as u8` (keyword, well-defined) |
| Type annotation in binding | `let x : int = ...` | `let x: i32 = ...` (same idea) |
| Runtime type inspection | `Obj.tag`, GADT witness | `dyn Any` + `downcast_ref::<T>()` |
| Safe type conversion | `type coercion` rarely | `From`/`Into` — compile-time verified |
