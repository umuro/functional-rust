📖 **[View on hightechmind.io →](https://hightechmind.io/rust/427-syn-quote-basics)**

---

# 427: `syn` and `quote!` Basics
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Writing proc macros without `syn` and `quote` is like writing a compiler without an AST — you'd be manipulating raw token streams manually. `syn` parses Rust token streams into a rich AST: `DeriveInput`, `ItemFn`, `Type`, `Expr`, `Ident`. `quote!` generates Rust code from these AST nodes with clean `#variable` interpolation. Together, they are the standard toolkit for every serious Rust proc macro, used by `serde`, `tokio`, `clap`, and virtually every derive macro in the ecosystem.

Understanding `syn` and `quote` is the gateway to implementing production-quality proc macros that generate correct, well-formatted Rust code.

## Learning Outcomes

- Understand how `syn::parse_macro_input!` parses a `TokenStream` into a typed AST
- Learn how `DeriveInput` provides `ident`, `generics`, and `data` (struct/enum/union)
- See how `quote!` uses `#ident` interpolation to generate code from parsed values
- Understand `proc_macro2::TokenStream` vs. `proc_macro::TokenStream` (the bridge between proc macro boundary and quote)
- Learn how to iterate struct fields using `syn::Fields` to generate per-field code

## Rust Application

The `src/lib.rs` demonstrates the patterns that `syn` and `quote!` produce. A typical proc macro entry point: `parse_macro_input!(input as DeriveInput)`, extract `&ast.ident` for the type name, match `ast.data` for `Data::Struct(ref data)`, iterate `data.fields` to get field names and types, then `quote! { impl #trait_name for #name { ... #(#field_impls)* ... } }`. The `#()*` syntax in `quote!` is repetition over a `Vec<TokenStream>`.

## OCaml Approach

OCaml's `ppxlib` provides `Ast_pattern` for parsing and `Ast_builder` for generation — the direct equivalents of `syn` and `quote`. `Ast_pattern.(pstr (pstr_type __ __))` matches type declarations; `Ast_builder.Default.str` builds string AST nodes. The functional style of OCaml makes AST traversal through pattern matching more natural, but the verbosity of explicit AST construction matches `quote!`'s more concise interpolation.

## Key Differences

1. **Ergonomics**: `quote!`'s `#var` interpolation is concise; OCaml's `Ast_builder` requires explicit node construction (`Ast_builder.Default.eapply`).
2. **Type safety**: `syn`'s typed AST ensures you're working with valid Rust constructs; OCaml's `Parsetree` is also typed but more verbose.
3. **Hygiene**: `quote!` generates hygienic identifiers using `proc_macro2::Span::call_site()`; OCaml PPX inherits OCaml's lack of macro hygiene.
4. **Generics**: `syn`'s `generics.split_for_impl()` handles the complex case of generic impl blocks; OCaml requires manual handling of type parameters.

## Exercises

1. **Field counter**: Write the body of a derive macro (using the patterns from this example) that generates `impl FieldCount for T { fn field_count() -> usize { N } }` where N is the number of fields in the struct.
2. **Type stringifier**: Using `syn` patterns, generate `impl TypeName for T { fn type_name() -> &'static str { "T" } }` and also `fn field_types() -> Vec<&'static str>` listing each field's type name as a string.
3. **Serde skeleton**: Study `serde_derive`'s source for its `Serialize` derive. Identify which `syn` types it uses to extract field names and types, and how `quote!` generates the `serialize_struct` call. Write a simplified version that generates JSON-string serialization (without serde's runtime).
