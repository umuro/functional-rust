# OCaml vs Rust: Lifetimes in Structs

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml structs own their data — no lifetime needed *)
type excerpt = { text : string; page : int }

let make_excerpt text page = { text; page }

let () =
  let book = "Call me Ishmael. Some years ago..." in
  let exc = make_excerpt (String.sub book 0 16) 1 in
  assert (exc.text = "Call me Ishmael.");
  Printf.printf "Excerpt p.%d: %s\n" exc.page exc.text
```

### Rust (idiomatic — struct borrows data)
```rust
#[derive(Debug)]
struct Excerpt<'a> {
    text: &'a str,
    page: u32,
}

fn main() {
    let book = String::from("Call me Ishmael. Some years ago...");
    let exc = Excerpt { text: &book[..16], page: 1 };
    assert_eq!(exc.text, "Call me Ishmael.");
    println!("Excerpt p.{}: {}", exc.page, exc.text);
    // Compiler guarantees: exc cannot outlive book
}
```

### Rust (functional — struct with multiple borrowed fields)
```rust
#[derive(Debug)]
struct Article<'a> {
    title: &'a str,
    author: &'a str,
    body: &'a str,
}

impl<'a> Article<'a> {
    fn summarize(&self) -> String {
        format!("{} by {} ({} chars)", self.title, self.author, self.body.len())
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Struct with string | `type t = { text: string }` | `struct T<'a> { text: &'a str }` |
| Constructor | `let make t p = { text=t; page=p }` | `fn new(text: &'a str, page: u32) -> Self` |
| Method returning borrow | returns owned `string` | `fn get(&self) -> &'a str` |
| Lifetime parameter | implicit (GC manages) | explicit `'a` on struct and `impl` |

## Key Insights

1. **Ownership vs GC:** In OCaml, `string` fields are heap-allocated and reference-counted by the GC — the struct owns a copy. In Rust, `&'a str` is a borrow: the struct holds a pointer without owning the data, so it must be proven valid.

2. **Explicit lifetime parameter:** The `'a` on `Excerpt<'a>` is Rust's way of encoding "this struct is only valid while the referenced `str` is alive." OCaml's GC makes this implicit — any live reference prevents collection.

3. **Compiler-enforced scope:** Rust will reject code that moves `Excerpt` to a scope where the referenced `String` is no longer live. OCaml, Java, and Python silently keep the source alive (or crash in C). Rust does this at zero runtime cost.

4. **Zero-copy views:** Because `&'a str` is a slice into existing memory, creating an `Excerpt` never allocates. The OCaml equivalent (`String.sub`) copies bytes. Lifetimes make zero-copy safe without any runtime bookkeeping.

5. **Multiple lifetime parameters:** A struct can carry multiple lifetimes (`struct Pair<'a, 'b>`) when its fields borrow from different sources with potentially different scopes — something invisible in GC languages but explicit and precise in Rust.

## When to Use Each Style

**Use borrowing structs (`&'a str`) when:** you want zero-copy views into existing data — parsing, tokenizing, window operations, or any case where you'd otherwise copy a substring just to store it temporarily.

**Use owned structs (`String`) when:** the struct needs to outlive its source, be sent across threads, or stored in a collection that must own its data. The tradeoff is an allocation, but you gain unrestricted lifetime.
