📖 **[View on hightechmind.io →](https://hightechmind.io/rust/495-string-template-pattern)**

---

# String Template Pattern
**Difficulty:** ⭐  
**Category:** Functional Programming  


A `{{variable}}` template engine implemented with a streaming parser that processes the template in a single left-to-right pass, avoiding repeated full-string scans from naive `replace` loops.

## Problem Statement

`format!` requires the template to be known at compile time. For user-defined or config-driven templates — email subjects, report headers, webhook payloads — the template is a runtime string. A simple but naive implementation calls `string.replace("{{key}}", value)` for each variable: this scans the entire string N times (one per variable) and allocates N intermediate strings. The efficient approach is a **single streaming pass**: scan for `{{`, extract the key, look it up, emit the value, and advance past `}}` — O(template_length) with at most one allocation for the result.

## Learning Outcomes

- Implement a streaming template parser using `find` + manual index arithmetic
- Emit literal text and substituted values into a pre-allocated `String`
- Handle missing variables gracefully (keep placeholder vs. empty string)
- Compare the streaming approach against the naive `replace` loop
- Use `String::with_capacity` to pre-allocate based on template size

## Rust Application

`render_fn` implements the single-pass approach:

```rust
fn render_fn<F: Fn(&str) -> Option<String>>(template: &str, lookup: F) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(start) = rest.find("{{") {
        out.push_str(&rest[..start]);   // emit literal prefix
        rest = &rest[start + 2..];
        if let Some(end) = rest.find("}}") {
            let key = &rest[..end];
            out.push_str(&lookup(key).unwrap_or_else(|| format!("{{{{{}}}}}", key)));
            rest = &rest[end + 2..];
        } else {
            out.push_str("{{");  // unclosed — preserve as-is
        }
    }
    out.push_str(rest);   // trailing literal
    out
}
```

The naive `render` function with `replace` serves as a baseline for correctness testing.

## OCaml Approach

```ocaml
let render template vars =
  let buf = Buffer.create (String.length template) in
  let rec loop i =
    match String.index_from_opt template i '{' with
    | None -> Buffer.add_substring buf template i (String.length template - i)
    | Some j when j + 1 < String.length template && template.[j+1] = '{' ->
        Buffer.add_substring buf template i (j - i);
        (match String.index_from_opt template (j+2) '}' with
         | Some k when k + 1 < String.length template && template.[k+1] = '}' ->
             let key = String.sub template (j+2) (k - j - 2) in
             Buffer.add_string buf (Option.value ~default:key (List.assoc_opt key vars));
             loop (k+2)
         | _ -> Buffer.add_string buf "{{"; loop (j+2))
    | Some j -> Buffer.add_char buf template.[j]; loop (j+1)
  in
  loop 0; Buffer.contents buf
```

## Key Differences

1. **Single pass**: Rust's streaming approach using `find` + slice advancement is O(N); the naive `replace` loop is O(N×M) where M is the number of variables.
2. **Slice advancement**: Rust's `rest = &rest[end + 2..]` advances without copying; OCaml uses index arithmetic with `String.index_from_opt`.
3. **`with_capacity`**: Rust pre-allocates the output buffer at `template.len()` bytes; OCaml's `Buffer.create` takes a hint.
4. **Closure lookup**: Rust's `render_fn` takes a generic `F: Fn(&str) -> Option<String>`, enabling HashMap, function, or closure-based lookup; OCaml uses an association list or `Hashtbl`.

## Exercises

1. **Recursive templates**: Extend `render_fn` to support nested templates — substitute values that themselves contain `{{key}}` placeholders, with a depth limit to prevent infinite loops.
2. **Benchmark render strategies**: Use `criterion` to compare the naive `replace` loop vs. `render_fn` for a 1KB template with 20 variables.
3. **Typed templates**: Design a `Template<Args>` type (using a struct with named fields) that validates at compile time that all required variables are provided, similar to `format_args!`.
