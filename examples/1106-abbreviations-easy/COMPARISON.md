# OCaml vs Rust: Abbreviations, Easy

## Side-by-Side Code

### OCaml

```ocaml
let char_is_uppercase c =
  match c with
  | 'A'..'Z' -> true
  | _ -> false

let get_abbr s =
  let seq = String.to_seq s in
  let seq = Seq.filter char_is_uppercase seq in
  String.of_seq seq

(* Build table: (min_abbr, FULL_UPPERCASE) *)
let cmds =
  List.map (fun s ->
    get_abbr s,
    String.uppercase_ascii s
  ) (Str.split (Str.regexp "[ \r\n]+") raw_cmds)

(* Lookup a user word *)
let lookup ucmd cmds =
  let n = String.length ucmd in
  let find_abbr (abbr, cmd) =
    let na = String.length abbr in
    let nc = String.length cmd in
    if n < na || nc < n then false
    else String.sub cmd 0 n = String.uppercase_ascii ucmd
  in
  match List.find_opt find_abbr cmds with
  | Some (_, found) -> found
  | None -> "*error*"
```

### Rust (idiomatic)

```rust
pub fn get_abbr(s: &str) -> String {
    s.chars().filter(|c| c.is_uppercase()).collect()
}

pub fn lookup<'a>(word: &str, commands: &[(&str, &'a str)]) -> &'a str {
    let n = word.len();
    let word_upper = word.to_uppercase();
    commands
        .iter()
        .find(|(abbr, full)| {
            let na = abbr.len();
            let nc = full.len();
            n >= na && n <= nc && full[..n] == word_upper
        })
        .map(|(_, full)| *full)
        .unwrap_or("*error*")
}

pub fn build_table(raw: &str) -> Vec<(String, String)> {
    raw.split_whitespace()
        .map(|s| (get_abbr(s), s.to_uppercase()))
        .collect()
}
```

### Rust (pipeline-focused with explicit borrow step)

```rust
pub fn resolve_all(user_input: &str, raw_commands: &str) -> Vec<String> {
    let table = build_table(raw_commands);
    // Borrow as (&str, &str) to avoid cloning in the lookup loop
    let table_refs: Vec<(&str, &str)> = table
        .iter()
        .map(|(a, c)| (a.as_str(), c.as_str()))
        .collect();
    user_input
        .split_whitespace()
        .map(|word| lookup(word, &table_refs).to_string())
        .collect()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Extract abbreviation | `val get_abbr : string -> string` | `fn get_abbr(s: &str) -> String` |
| Lookup a word | `val lookup : string -> (string * string) list -> string` | `fn lookup<'a>(word: &str, commands: &[(&str, &'a str)]) -> &'a str` |
| Build table | `List.map` over `Str.split` result | `fn build_table(raw: &str) -> Vec<(String, String)>` |
| Command entry | `string * string` tuple | `(String, String)` owned or `(&str, &str)` borrowed |
| Optional match | `'a option` / `match ... with Some/None` | `Option<T>` / `.map().unwrap_or()` |

## Key Insights

1. **`chars().filter().collect()` mirrors `Seq.filter`:** OCaml and Rust both use lazy
   sequences to filter characters by predicate. The pipeline is identical conceptually;
   only the syntax differs. Rust makes the collection step explicit with `.collect()`.

2. **Lifetime `'a` replaces the GC:** In OCaml the returned string is always a fresh
   allocation and the GC tracks it. In Rust, returning `&'a str` borrows from the table
   slice, which must outlive the return value. The lifetime annotation makes this
   contract explicit and zero-cost — no allocation occurs in the lookup path.

3. **`Iterator::find` vs `List.find_opt`:** Both return an optional match. OCaml
   uses `match ... with Some/None`; Rust chains `.map(|(_, full)| *full).unwrap_or("*error*")`
   — the combinator form eliminates the match expression entirely.

4. **`split_whitespace` handles messy data gracefully:** OCaml needs `Str.regexp "[ \r\n]+"`;
   Rust's `str::split_whitespace` skips any Unicode whitespace by default, making the
   table parser one method call instead of a regex import.

5. **Two-phase borrow for lookup:** `build_table` returns owned `Vec<(String, String)>`.
   Before passing to `lookup`, we re-borrow as `Vec<(&str, &str)>`. This is idiomatic Rust:
   own data at the boundary, borrow for computation, avoiding redundant allocations.

## When to Use Each Style

**Use idiomatic Rust (iterator chain) when:** processing structured text data into lookup
tables — the single-pass iterator pipeline is clear, allocation-minimal, and composes well
with the rest of the standard library.

**Use lifetime-annotated borrows when:** returning string references from a function that
searches a collection — it avoids allocating a new `String` per lookup and lets the caller
control memory lifetime explicitly.
