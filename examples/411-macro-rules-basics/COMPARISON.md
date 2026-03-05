# OCaml vs Rust: Declarative Macros

## Side-by-Side Code

### OCaml — Functions (no macros)
```ocaml
(* No macro_rules! equivalent in OCaml *)
(* Use functions or ppx preprocessors *)

let min_of a b = if a < b then a else b
let max_of a b = if a > b then a else b

let repeat n f =
  for _ = 1 to n do f () done

let () =
  repeat 3 (fun () -> print_endline "Hello");
  Printf.printf "min=%d\n" (min_of 3 7)
```

### Rust — macro_rules!
```rust
macro_rules! min_of {
    ($a:expr) => { $a };
    ($a:expr, $($rest:expr),+) => {{
        let rest = min_of!($($rest),+);
        if $a < rest { $a } else { rest }
    }};
}

macro_rules! repeat {
    ($n:expr, $body:block) => {
        for _ in 0..$n $body
    };
}

fn main() {
    repeat!(3, { println!("Hello"); });
    println!("min={}", min_of!(3, 7, 1, 9));
}
```

---

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Compile-time codegen | ppx (external) | Built-in `macro_rules!` |
| Variadic functions | Lists or labeled args | Macro repetition `$(...)*` |
| File/line info | `__LOC__` | `file!()`, `line!()` |
| Hygiene | N/A | Hygienic by default |
| Pattern matching | In functions | In macro arms |

---

## macro_rules! Syntax

```rust
macro_rules! name {
    // Pattern 1
    ($pattern:designator) => {
        expansion
    };
    // Pattern 2
    ($x:expr, $($rest:expr),*) => {
        expansion with repetition
    };
}
```

### Designators
- `expr` — expression
- `ident` — identifier
- `ty` — type
- `pat` — pattern
- `tt` — token tree (anything)
- `block` — code block

---

## Repetition

```rust
macro_rules! vec_of {
    ($($x:expr),* $(,)?) => {
        vec![$($x),*]
    };
}

let v = vec_of![1, 2, 3];  // vec![1, 2, 3]
```

- `$(...)*` — zero or more
- `$(...)+` — one or more
- `$(...)?` — zero or one

---

## 5 Takeaways

1. **OCaml lacks built-in macros; ppx is external.**
   Rust's `macro_rules!` is part of the language.

2. **Macros enable variadic "functions" at compile time.**
   `min_of!(1, 2, 3, 4)` works for any number of args.

3. **Pattern matching in macros is powerful.**
   Different expansions based on input shape.

4. **`file!()` and `line!()` enable better error messages.**
   Available inside macros for debugging.

5. **Macros are hygienic — variables don't leak.**
   A variable inside a macro won't shadow outer scope.
