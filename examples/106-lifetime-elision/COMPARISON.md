# OCaml vs Rust: Lifetime Elision

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml has no lifetime annotations — the GC handles memory safety *)
let first_word s =
  match String.index_opt s ' ' with
  | Some i -> String.sub s 0 i
  | None -> s

type text_buffer = { content : string }
let get_content buf = buf.content
let get_length buf = String.length buf.content
```

### Rust (idiomatic — elision applies)
```rust
// Compiler infers: fn first_word<'a>(s: &'a str) -> &'a str
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or(s)
}

struct TextBuffer { content: String }

impl TextBuffer {
    // Compiler infers: fn get_content<'a>(&'a self) -> &'a str
    fn get_content(&self) -> &str { &self.content }
    fn get_length(&self) -> usize { self.content.len() }
}
```

### Rust (explicit — when elision cannot resolve ambiguity)
```rust
// Two input references: compiler cannot know which to tie the output to
fn pick_first<'a>(a: &'a str, _b: &str) -> &'a str { a }

// Struct holding a reference always requires explicit annotation
struct Excerpt<'a> { text: &'a str }
```

## Type Signatures

| Concept | OCaml | Rust (elided) | Rust (explicit) |
|---------|-------|---------------|-----------------|
| String slice | `string` | `&str` | `&'a str` |
| Function (1 ref in → ref out) | `string -> string` | `fn f(s: &str) -> &str` | `fn f<'a>(s: &'a str) -> &'a str` |
| Method returning borrowed field | `t -> string` | `fn m(&self) -> &str` | `fn m<'a>(&'a self) -> &'a str` |
| Struct with borrowed field | impossible (GC owns) | *(must annotate)* | `struct S<'a> { f: &'a str }` |

## Key Insights

1. **No annotations vs. hidden annotations:** OCaml's GC removes the need for lifetime reasoning entirely. Rust's elision rules make the *most common* lifetime relationships implicit — the annotations exist, the compiler just fills them in.
2. **Three deterministic rules:** (1) each input ref gets its own lifetime; (2) a single input lifetime propagates to outputs; (3) `&self`/`&mut self` propagates to outputs. When these rules yield one answer, you write nothing.
3. **Struct fields always need annotations:** Elision only applies to function signatures. A struct that holds a reference must declare `struct Foo<'a> { field: &'a T }` — there is no single obvious input to elide from.
4. **Ambiguous inputs force explicit annotations:** `fn f(a: &str, b: &str) -> &str` is a compile error because rule 2 no longer applies — two lifetimes, two possible sources. Rust forces you to pick: `fn f<'a>(a: &'a str, b: &str) -> &'a str`.
5. **Mental expansion:** Reading elided code is easiest when you mentally restore the annotations: `fn get_content(&self) -> &str` → `fn get_content<'a>(&'a self) -> &'a str`. This tells you the returned `&str` cannot outlive `self`.

## When to Use Each Style

**Use elided lifetimes when:** the function has a single input reference, or is a method where `&self` is the obvious donor — which covers the vast majority of real Rust code.  
**Use explicit lifetimes when:** there are multiple input references and the output could borrow from more than one, when a struct stores a reference, or when you want to document a non-obvious lifetime relationship for readers.
