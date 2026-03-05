# 424: Custom #[derive(MyTrait)]

**Difficulty:** 4  **Level:** Expert

Generate trait implementations automatically for any struct or enum — the same mechanism powering `#[derive(Debug, Serialize, Clone)]`.

## The Problem This Solves

Some traits have an obvious mechanical implementation that depends only on a type's structure. `Debug` prints fields. `Serialize` iterates them. `Hash` combines field hashes. Writing these by hand for every new struct is tedious, error-prone (forget to update when adding a field), and produces hundreds of lines of boilerplate.

Custom derive macros solve this: annotate your struct with `#[derive(YourTrait)]` and the macro reads the struct's fields at compile time, generates the implementation, and inserts it as if you'd written it by hand. The generated code is visible via `cargo expand`, it's zero-overhead, and it stays in sync — add a field and the derive regenerates.

This is the mechanism the entire Rust ecosystem is built on. `serde`'s `Serialize`/`Deserialize`, `thiserror`'s `Error`, `clap`'s `Parser` — all derive macros. Learning to write one makes you fluent in how the ecosystem actually works.

## The Intuition

A derive macro reads your struct's fields and generates an `impl YourTrait for YourStruct` block at compile time — like a code generator that runs every time you build.

## How It Works in Rust

```rust
// In your proc-macro crate:
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;  // struct name as Ident

    // Extract named field names
    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => f.named.iter()
                .map(|f| &f.ident)
                .collect::<Vec<_>>(),
            _ => vec![],
        },
        _ => panic!("Describe only works on structs"),
    };

    // Generate the impl
    quote! {
        impl Describe for #name {
            fn describe(&self) -> String {
                let mut parts = vec![format!("{}", stringify!(#name))];
                #( parts.push(format!("{}: {:?}", stringify!(#fields), self.#fields)); )*
                parts.join(", ")
            }
        }
    }.into()
}

// Usage in another crate:
#[derive(Describe)]
struct User { name: String, age: u32 }

let u = User { name: "Alice".into(), age: 30 };
println!("{}", u.describe());  // "User, name: "Alice", age: 30"
```

1. `parse_macro_input!(input as DeriveInput)` — parse tokens into a structured AST.
2. Navigate `data.fields` to find field names and types.
3. `quote! { impl Trait for #name { ... } }` — generate the implementation.
4. `#( ... )*` in `quote!` iterates over a collection, like `macro_rules!` repetition.

## What This Unlocks

- **Boilerplate elimination**: One `#[derive(YourTrait)]` replaces 50 lines of mechanical implementation per struct.
- **Field-aware generation**: Access field names, types, attributes (`#[serde(rename = "...")]`) and generate accordingly.
- **Ecosystem-level patterns**: Everything from `serde` to `sqlx` to `clap` is built exactly this way.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Auto-derived implementations | `[@@deriving show, eq]` via `ppx_deriving` | `#[derive(Trait)]` via proc macro |
| Field introspection | `Ppxlib` AST traversal | `syn::DeriveInput` → `Data::Struct` → `Fields` |
| Code generation | `Ppxlib.Ast_builder.Default` | `quote!` macro with `#name`, `#(...)* ` |
| Separate crate | PPX is separate library | proc-macro crate required |
| Attribute reading | `[@attr]` on fields | `field.attrs` in `syn` — parse with `syn::parse` |
