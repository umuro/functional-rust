📖 **[View on hightechmind.io →](https://hightechmind.io/rust/424-proc-macro-derive)**

---

# 424: Proc Macro Derive
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A custom derive macro generates trait implementations automatically from a type's definition. When you annotate `#[derive(MyTrait)]`, the proc macro receives the struct/enum definition as a token stream, parses it to find field names and types, and emits an `impl MyTrait for TheType` block. This is the mechanism behind `serde::Deserialize` — it inspects every field name and type, generating JSON deserialization code specific to that struct's shape, something impossible with `macro_rules!`.

Custom derive macros appear whenever library authors want users to opt-in to generated boilerplate: ORMs generating SQL mappings, test frameworks generating fixtures, protocol libraries generating serialization code.

## Learning Outcomes

- Understand the derive proc macro lifecycle: `TokenStream` in → parse with `syn` → generate with `quote!` → `TokenStream` out
- Learn how `syn::DeriveInput` represents parsed struct definitions with field names and types
- See how `quote::quote!` generates code with token interpolation using `#variable` syntax
- Understand the crate separation requirement: proc macros in `proc-macro = true` crates
- Learn how `#[proc_macro_derive(Name)]` registers the macro for use with `#[derive(Name)]`

## Rust Application

The `src/lib.rs` demonstrates what a derive proc macro generates. A real implementation would be in a separate crate with dependencies on `syn` and `quote`. The derive macro pattern: `pub fn my_derive(input: TokenStream) -> TokenStream`, call `syn::parse_macro_input!(input as DeriveInput)`, extract the `ident` (struct name) and fields, then `quote! { impl MyTrait for #name { ... } }`.

## OCaml Approach

OCaml's `ppx_deriving` library provides the framework for writing custom derivers. A deriver registers with `Ppx_deriving.register` providing `type_decl -> structure_item list` functions. The OCaml AST types (`type_declaration`, `label_declaration`) correspond to `syn::DeriveInput` and `syn::Field`. Code generation uses `Ast_builder.Default` module functions rather than `quote!`.

## Key Differences

1. **Registration**: Rust registers via `#[proc_macro_derive(Name)]`; OCaml uses `Ppx_deriving.register "name" (module Deriver)`.
2. **Code generation**: Rust uses `quote!` with `#` interpolation; OCaml uses `Ast_builder` with explicit AST node construction.
3. **Error location**: Rust can use `span` from `syn` for precise error locations; OCaml uses `Location.t` values from the AST.
4. **Testing**: Rust tests proc macros with `trybuild` crate (compile-fail tests); OCaml uses `ppx_deriving`'s test infrastructure.

## Exercises

1. **Describe trait**: Implement a `#[derive(Describe)]` that generates `impl Describe for T { fn describe() -> String { "T { field1: type1, field2: type2 }" } }` using field names and type names from `syn`. Test it on a two-field struct.
2. **Getters derive**: Write `#[derive(Getters)]` generating `pub fn field_name(&self) -> &FieldType` for every field. Handle `pub` and private fields, skipping fields with `#[getter(skip)]` attribute.
3. **Builder derive**: Implement `#[derive(Builder)]` that generates a `{StructName}Builder` with setter methods and a `build()` method. Handle `Option<T>` fields as optional, other fields as required.
