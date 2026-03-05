# OCaml vs Rust: Abbreviations, Simple

## Side-by-Side Code

### OCaml

```ocaml
(* Parse table: split on whitespace, pair each word with the next token if it is
   an integer (= min length), otherwise use the word's own length as the minimum. *)
let parse_commands table =
  let tokens = String.split_on_char ' ' table
               |> List.filter (fun s -> s <> "") in
  let rec loop acc = function
    | [] -> List.rev acc
    | w :: n :: rest when int_of_string_opt n <> None ->
        loop ((String.lowercase_ascii w, int_of_string n) :: acc) rest
    | w :: rest ->
        let name = String.lowercase_ascii w in
        loop ((name, String.length name) :: acc) rest
  in
  loop [] tokens

(* Lookup: find first command where len(word) >= min AND cmd starts with word *)
let abbreviate word commands =
  let lower = String.lowercase_ascii word in
  let n = String.length lower in
  match List.find_opt (fun (cmd, min_len) ->
    n >= min_len &&
    String.length cmd >= n &&
    String.sub cmd 0 n = lower
  ) commands with
  | Some (cmd, _) -> String.uppercase_ascii cmd
  | None -> "*error*"
```

### Rust (idiomatic)

```rust
pub fn parse_commands(table: &str) -> Vec<(String, usize)> {
    let tokens: Vec<&str> = table.split_whitespace().collect();
    let mut commands = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let name = tokens[i].to_lowercase();
        let (min_len, advance_by) = if i + 1 < tokens.len() {
            match tokens[i + 1].parse::<usize>() {
                Ok(n) => (n, 2),
                Err(_) => (name.len(), 1),
            }
        } else {
            (name.len(), 1)
        };
        commands.push((name, min_len));
        i += advance_by;
    }
    commands
}

pub fn abbreviate(word: &str, commands: &[(String, usize)]) -> String {
    let lower = word.to_lowercase();
    let len = lower.len();
    commands
        .iter()
        .find(|(cmd, min)| len >= *min && cmd.starts_with(lower.as_str()))
        .map(|(cmd, _)| cmd.to_uppercase())
        .unwrap_or_else(|| "*error*".to_string())
}
```

### Rust (functional/recursive table parsing)

```rust
// Alternative: fold-based parser — accumulates (command, min) pairs by
// inspecting each token and peeking at the next.
pub fn parse_commands_fold(table: &str) -> Vec<(String, usize)> {
    let tokens: Vec<&str> = table.split_whitespace().collect();
    tokens
        .iter()
        .enumerate()
        .filter_map(|(i, tok)| {
            // Skip tokens that are pure numbers (they were consumed by the previous command)
            if tok.parse::<usize>().is_ok() {
                return None;
            }
            let name = tok.to_lowercase();
            let min_len = tokens
                .get(i + 1)
                .and_then(|next| next.parse::<usize>().ok())
                .unwrap_or(name.len());
            Some((name, min_len))
        })
        .collect()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Table entry | `(string * int)` | `(String, usize)` |
| Command table | `(string * int) list` | `&[(String, usize)]` |
| Lookup result | `string` (with `"*error*"`) | `String` (with `"*error*"`) |
| Parse function | `string -> (string * int) list` | `fn(&str) -> Vec<(String, usize)>` |
| Lookup function | `string -> (string * int) list -> string` | `fn(&str, &[(String, usize)]) -> String` |

## Key Insights

1. **Parse once, query many times:** Rust separates `parse_commands` (one allocation) from `abbreviate`
   (zero allocation beyond the result string). OCaml functional style favours the same separation but
   less explicitly.

2. **`starts_with` vs manual `String.sub`:** Rust's `str::starts_with` handles the boundary check
   (`cmd.len() >= word.len()`) implicitly — if `word` is longer than `cmd`, `starts_with` returns
   `false`. OCaml requires explicit bounds checking with `String.sub`.

3. **The "no number = exact match" encoding:** Storing `min_len = name.len()` for unnumbered commands
   is the same trick in both languages: the prefix check `len >= min` collapses to `len >= cmd.len()`,
   which combined with `cmd.starts_with(lower)` forces an exact match.

4. **`find` + `map` + `unwrap_or_else` vs `match List.find_opt`:** Rust's combinator chain is
   functionally identical to OCaml's `match ... with Some/None` but reads left-to-right without
   indentation nesting.

5. **Case discipline:** Both approaches lowercase the input and the command for comparison, then
   uppercase the matched command for display output. Rust's `.to_uppercase()` returns an `String`
   (not `&str`) because Unicode uppercasing can expand the byte length.

## When to Use Each Style

**Use idiomatic Rust when:** you have a fixed table processed at startup — parse once into a typed
`Vec`, then do O(n) lookups with iterator combinators.

**Use recursive/fold Rust when:** you want a purely functional pipeline without a mutable loop index
— the `filter_map` + `enumerate` approach achieves the same result with no `while` loop.
