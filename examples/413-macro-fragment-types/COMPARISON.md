# OCaml vs Rust: Macro Fragment Types

## Fragment Specifiers

| Designator | Captures | Example |
|------------|----------|---------|
| `expr` | Expression | `2 + 3`, `foo()` |
| `ident` | Identifier | `my_var`, `MyType` |
| `ty` | Type | `i32`, `Vec<String>` |
| `pat` | Pattern | `Some(x)`, `_` |
| `literal` | Literal | `"hello"`, `42` |
| `block` | Block | `{ statements }` |
| `stmt` | Statement | `let x = 1;` |
| `tt` | Token tree | Anything |

---

## Examples

### expr — Any expression
```rust
macro_rules! dbg_expr {
    ($e:expr) => {
        println!("{} = {:?}", stringify!($e), $e);
    };
}

dbg_expr!(2 + 3 * 4);  // "2 + 3 * 4 = 14"
```

### ident — Identifiers for code generation
```rust
macro_rules! make_getter {
    ($field:ident : $ty:ty) => {
        fn $field(&self) -> &$ty { &self.$field }
    };
}

struct Point { x: i32, y: i32 }
impl Point {
    make_getter!(x: i32);
    make_getter!(y: i32);
}
```

### ty — Type names
```rust
macro_rules! make_default {
    ($name:ident -> $ret:ty) => {
        fn $name() -> $ret { Default::default() }
    };
}

make_default!(empty_vec -> Vec<i32>);
```

### pat — Patterns
```rust
macro_rules! matches_pat {
    ($val:expr, $pat:pat) => { matches!($val, $pat) };
}

matches_pat!(Some(42), Some(_));  // true
```

---

## OCaml Equivalent

OCaml uses ppx for metaprogramming:

```ocaml
(* No direct equivalent to designators *)
(* ppx_deriving generates code from attributes *)

[@@deriving show, eq]
type point = { x: int; y: int }
```

---

## 5 Takeaways

1. **`expr` is most common — captures any expression.**
   Works for math, function calls, variables.

2. **`ident` enables code generation.**
   Generate getters, setters, function names.

3. **`ty` captures type annotations.**
   Useful for generic macro-generated code.

4. **`literal` is more restrictive than `expr`.**
   Only actual literals, not variables.

5. **`tt` is the escape hatch.**
   Captures anything when other designators fail.
