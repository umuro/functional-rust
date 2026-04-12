📖 **[View on hightechmind.io →](https://hightechmind.io/rust/413-macro-fragment-types)**

---

# 413: Macro Fragment Types (Designators)
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

`macro_rules!` patterns match different syntactic categories through designators: `$e:expr` matches any expression, `$i:ident` matches an identifier, `$t:ty` matches a type, `$p:pat` matches a pattern, `$b:block` matches a block. Choosing the right designator is critical — `expr` captures the full expression including operators, while `tt` captures a single token tree. Misusing designators leads to confusing parse errors or overly restrictive macros that refuse valid inputs.

Understanding fragment types is essential for writing robust macros that accept natural Rust syntax rather than awkward restricted subsets.

## Learning Outcomes

- Learn the complete set of macro fragment designators: `expr`, `ident`, `ty`, `pat`, `literal`, `block`, `stmt`, `item`, `meta`, `tt`, `lifetime`
- Understand what each designator accepts and its "NFA following token" restrictions
- See how `ident` enables generating method names, field names, and identifiers
- Learn how `ty` accepts full type expressions including generics
- Understand when to use `tt` (token tree) for maximum flexibility

## Rust Application

In `src/lib.rs`, `dbg_expr!` uses `$e:expr` and `stringify!($e)` to print both the source text and value. `make_getter!($field:ident : $ty:ty)` uses `ident` for the field name (generates a method name) and `ty` for the return type. `make_default_fn!` uses `$ty:ty` and `$default:expr`. The `$field:ident` enables generating `pub fn $field(...)` where the identifier becomes the method name.

## OCaml Approach

OCaml PPX extensions work on the AST directly: `[%ppx_gen field_name]` receives the parsed `Parsetree.expression` or `Parsetree.pattern` value rather than tokens. PPX has direct access to parsed types (`core_type`), patterns (`pattern`), and expressions (`expression`) — more structured than Rust's token-level matching, but requiring knowledge of OCaml's AST types.

## Key Differences

1. **Token vs. AST**: Rust macros work on token streams; OCaml PPX works on the parsed AST. Rust is more flexible but requires manual parsing; OCaml has structure but requires AST knowledge.
2. **Following token rules**: Rust fragment designators have restrictions on what can follow them (e.g., after `expr`, only `,`/`;`/`)` can appear); OCaml PPX has no such restrictions.
3. **Identifier generation**: Rust uses `$ident:ident` to capture and reuse identifiers; OCaml PPX uses `Ast_builder.evar` and `Ast_builder.pvar` to create identifier nodes.
4. **Type capture**: Rust's `$t:ty` captures full type syntax; OCaml PPX receives `core_type` which is already parsed into an AST node.

## Exercises

1. **Property macro**: Implement `property!($name:ident : $ty:ty = $default:expr)` that generates a `struct` field, a getter returning `&$ty`, a setter `set_$name(&mut self, val: $ty)`, and a default value initializer.
2. **Enum constructor**: Write `make_enum_from!(EnumName { Variant1(Type1), Variant2(Type2) })` that generates both `From<Type1> for EnumName` and `From<Type2> for EnumName` implementations.
3. **Debug print with type**: Create `typed_dbg!($e:expr)` that prints `"{expr_text}: {type_name} = {value:?}"`. Use `std::any::type_name::<T>()` inside the expansion to show the inferred type name.
