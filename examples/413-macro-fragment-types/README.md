📖 **[View on hightechmind.io →](https://hightechmind.io/rust/413-macro-fragment-types)**

---

# 413: Macro Fragment Specifiers

**Difficulty:** 3  **Level:** Advanced

The type system for macro inputs — `expr`, `ident`, `ty`, `pat`, `literal`, `block`, `stmt`, `tt` each capture different kinds of syntax.

## The Problem This Solves

A `macro_rules!` pattern like `($x)` would match anything — but the macro wouldn't know *what kind* of thing it captured. Can it call `$x()` as a function? Use it as a type? Pass it to another macro? Without type information, macro expansion is unsafe — misusing a captured fragment leads to cryptic errors deep in expanded code.

Fragment specifiers solve this by tagging what category of syntax a capture must match. `$x:expr` matches any expression. `$name:ident` matches only identifiers. `$t:ty` matches type syntax. `$p:pat` matches patterns. The compiler validates the specifier at macro definition time and at call sites. If you pass a type where an expression is expected, you get a clear error at the call site, not inside the expansion.

Choosing the right fragment specifier also determines what you can *do* with the captured fragment in the template. An `ident` can name a function or field. A `ty` can appear in a type position. An `expr` can appear where a value is needed. Using the wrong one produces a compiler error.

## The Intuition

Think of fragment specifiers as the "types" of macro parameters. Just as a function signature `fn f(x: i32)` constrains what you can pass, `($x:expr)` constrains the macro input. Each specifier has a specific syntactic grammar it matches and a set of positions where it can be used in the template.

The most flexible specifier is `tt` (token tree) — it matches any single token or any `()`, `[]`, `{}` group. It's the escape hatch: if no specifier fits, use `tt` and let the inner expansion handle parsing. But it gives up the structure that narrower specifiers provide.

## How It Works in Rust

```rust
// expr: any expression — evaluate and show its source text and value
macro_rules! dbg_expr {
    ($e:expr) => {
        {
            let val = $e;
            println!("  {} = {:?}", stringify!($e), val);  // stringify! gets source text
            val
        }
    };
}

// ident: identifiers — use to generate field names and function names
macro_rules! make_getter {
    ($field:ident : $ty:ty) => {
        fn $field(&self) -> &$ty { &self.$field }  // $field becomes a method name
    };
}

// ty: types — use in type position
macro_rules! make_default_fn {
    ($name:ident -> $ret:ty) => {
        fn $name() -> $ret { Default::default() }  // $ret is used as a return type
    };
}

// literal: constant literal values — strings, numbers, booleans
macro_rules! repeat_str {
    ($s:literal, $n:literal) => { $s.repeat($n) }
}

// block: a { ... } block expression
macro_rules! time_block {
    ($name:literal, $block:block) => {
        {
            let t = std::time::Instant::now();
            let result = $block;  // $block expands as-is
            println!("'{}' took {:?}", $name, t.elapsed());
            result
        }
    };
}

// pat: patterns — use in match arms and if let
macro_rules! matches_variant {
    ($val:expr, $pat:pat) => {
        matches!($val, $pat)
    };
}

struct Person { name: String, age: u32 }
impl Person {
    fn new(name: &str, age: u32) -> Self { Person { name: name.to_string(), age } }
    make_getter!(name: String);  // generates: fn name(&self) -> &String
    make_getter!(age: u32);      // generates: fn age(&self) -> &u32
}

make_default_fn!(empty_string -> String);  // generates: fn empty_string() -> String

fn main() {
    let x = dbg_expr!(2 + 3 * 4);     // prints "2 + 3 * 4 = 14"
    dbg_expr!(x > 10);                 // prints "x > 10 = true"

    let p = Person::new("Alice", 30);
    println!("name={}, age={}", p.name(), p.age());

    println!("default: {:?}", empty_string());

    let opt: Option<i32> = Some(42);
    println!("is Some: {}", matches_variant!(opt, Some(_)));
    println!("is None: {}", matches_variant!(opt, None));

    println!("{}", repeat_str!("ab", 3));  // "ababab"

    let sum = time_block!("sum_block", {
        (1..=1000i64).sum::<i64>()  // entire block is captured as $block
    });
    println!("sum = {}", sum);
}
```

**Fragment specifier reference:**

| Specifier | Matches | Usable as |
|-----------|---------|-----------|
| `expr` | Any expression | value, in expression positions |
| `ident` | Identifier | function name, field name, variable name |
| `ty` | Type syntax | type annotations, return types |
| `pat` | Pattern | `match` arms, `if let`, destructuring |
| `literal` | Literal value | compile-time constants |
| `block` | `{ ... }` block | expression positions |
| `stmt` | Statement | statement positions |
| `tt` | Any token tree | anything — most flexible, least typed |

## What This Unlocks

- **Code generators** — `make_getter!(field: Type)` generating accessors, `make_setter!`, `make_from_str!` — generate entire method families from a field list.
- **Test harnesses** — `test_case!(name, input, expected)` using `ident` for the test function name, `expr` for inputs — generates `#[test]` functions.
- **Instrumentation** — `dbg_expr!` style macros that capture both source text (via `stringify!`) and the evaluated value — impossible with regular functions.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Macro input categories | PPX handles AST nodes — complex, typed, extensible | `macro_rules!` fragment specifiers — simpler, built-in set, hygienic |
| Identifier capture | PPX: `Ast.Longident.t` | `$name:ident` — captured and reusable in template |
| Type capture | PPX: `Ast.core_type` | `$t:ty` — used directly in type positions |
| Pattern capture | PPX: `Ast.pattern` | `$p:pat` — used in `match` arms and `if let` |
