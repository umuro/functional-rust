[json-pretty-print on hightechmind.io](https://hightechmind.io/posts/functional-rust/json-pretty-print)

---

## Problem Statement

Implement a recursive JSON pretty-printer that produces indented, human-readable output from a `JsonValue` tree. Arrays and objects expand across multiple lines with consistent 2-space indentation per level. Handle string escaping (quotes, backslashes, newlines, tabs) and integer vs float formatting for numbers.

## Learning Outcomes

- Implement `pretty_print(json: &JsonValue, indent: usize) -> String` recursively with depth tracking
- Use `" ".repeat(indent * 2)` for current-level padding and `(indent + 1) * 2` for child padding
- Implement `escape_string` for JSON-safe string output: `\"`, `\\`, `\n`, `\t`, `\r`
- Format `Number` variants as integers when `fract() == 0.0 && is_finite()`
- Produce compact (no trailing newline, proper comma placement) multi-line output

## Rust Application

```rust
fn escape_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"'  => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c    => out.push(c),
        }
    }
    out
}

fn pretty_print(j: &JsonValue, indent: usize) -> String {
    let pad  = " ".repeat(indent * 2);
    let pad2 = " ".repeat((indent + 1) * 2);
    match j {
        JsonValue::Null     => "null".to_string(),
        JsonValue::Bool(b)  => b.to_string(),
        JsonValue::Number(n) =>
            if n.fract() == 0.0 && n.is_finite() { format!("{}", *n as i64) }
            else { format!("{}", n) },
        JsonValue::Str(s)   => format!("\"{}\"", escape_string(s)),
        JsonValue::Array(items) => {
            if items.is_empty() { return "[]".to_string(); }
            let inner: Vec<String> = items.iter()
                .map(|item| format!("{}{}", pad2, pretty_print(item, indent + 1)))
                .collect();
            format!("[\n{}\n{}]", inner.join(",\n"), pad)
        }
        JsonValue::Object(pairs) => {
            if pairs.is_empty() { return "{}".to_string(); }
            let inner: Vec<String> = pairs.iter()
                .map(|(k, v)| format!("{}\"{}\": {}", pad2, escape_string(k), pretty_print(v, indent + 1)))
                .collect();
            format!("{{\n{}\n{}}}", inner.join(",\n"), pad)
        }
    }
}
```

Commas go between elements, not after the last one — `.join(",\n")` handles this correctly. Empty arrays and objects use compact form `[]` / `{}` to avoid unnecessary blank lines.

The closing `]` or `}` is indented at the current level (`pad`), while the children are at `pad2` (one level deeper). This produces:

```json
{
  "name": "Alice",
  "scores": [
    1,
    2,
    3
  ]
}
```

## OCaml Approach

```ocaml
let rec pretty_print ?(indent=0) j =
  let pad  = String.make (indent * 2) ' ' in
  let pad2 = String.make ((indent + 1) * 2) ' ' in
  match j with
  | Null -> "null"
  | Bool b -> string_of_bool b
  | Number n ->
    if Float.is_integer n then string_of_int (int_of_float n)
    else string_of_float n
  | Str s -> Printf.sprintf "\"%s\"" (escape_string s)
  | Array [] -> "[]"
  | Array items ->
    let inner = List.map (fun item ->
      pad2 ^ pretty_print ~indent:(indent+1) item) items in
    Printf.sprintf "[\n%s\n%s]" (String.concat ",\n" inner) pad
  | Object [] -> "{}"
  | Object pairs ->
    let inner = List.map (fun (k, v) ->
      Printf.sprintf "%s\"%s\": %s" pad2 k (pretty_print ~indent:(indent+1) v)) pairs in
    Printf.sprintf "{\n%s\n%s}" (String.concat ",\n" inner) pad
```

OCaml's optional argument `?(indent=0)` provides a default value — cleaner than Rust's single required `indent` parameter. The structure is otherwise identical.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Default argument | Separate public wrapper or always pass | `?(indent=0)` optional argument |
| String building | `format!` + `String::with_capacity` | `Printf.sprintf` + `^` concatenation |
| String joining | `.join(",\n")` | `String.concat ",\n"` |
| Escaping | Match per character, push to `String` | Same approach with `Buffer` or char match |

The recursive pretty-printer naturally matches the recursive structure of `JsonValue`. There is no stack depth concern for typical JSON documents; deeply nested structures (depth > 10,000) could overflow the call stack.

## Exercises

1. Add a compact (single-line) serializer `to_json_compact` with no indentation or newlines.
2. Handle Unicode escaping: replace characters outside ASCII printable range with `\uXXXX`.
3. Add a `max_depth` parameter and return `Err` if the JSON nests deeper than the limit.
4. Implement `diff(a: &JsonValue, b: &JsonValue)` that prints which values differ between two JSON trees.
5. Add trailing comma suppression for the last element in arrays and objects (some formatters prefer this style).
