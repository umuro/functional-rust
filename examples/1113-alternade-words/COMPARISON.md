# OCaml vs Rust: Alternade Words

## Side-by-Side Code

### OCaml

```ocaml
module StrSet = Set.Make(String)

let get_alternade set s =
  let s0 = String.init (succ (String.length s) lsr 1) (fun i -> s.[i + i])
  and s1 = String.init (String.length s lsr 1) (fun i -> s.[i + succ i]) in
  if StrSet.mem s0 set && StrSet.mem s1 set
  then Some (Printf.sprintf "%s | %s %s" s s0 s1) else None

let () =
  let set = seq_lines stdin |> Seq.filter (min_len 3) |> StrSet.of_seq in
  StrSet.to_seq set |> Seq.filter (min_len 6)
  |> Seq.filter_map (get_alternade set) |> Seq.iter print_endline
```

### Rust (idiomatic — iterator stride)

```rust
pub fn split_alternade(word: &str) -> (String, String) {
    let even: String = word.chars().step_by(2).collect();
    let odd: String  = word.chars().skip(1).step_by(2).collect();
    (even, odd)
}

pub fn find_alternades(words: &[&str]) -> Vec<String> {
    use std::collections::HashSet;
    let word_set: HashSet<&str> =
        words.iter().copied().filter(|w| w.len() >= 3).collect();

    let mut results: Vec<String> = words
        .iter().copied()
        .filter(|w| w.len() >= 6)
        .filter_map(|word| {
            let (even, odd) = split_alternade(word);
            if word_set.contains(even.as_str()) && word_set.contains(odd.as_str()) {
                Some(format!("{word} | {even} {odd}"))
            } else {
                None
            }
        })
        .collect();
    results.sort();
    results
}
```

### Rust (explicit index loop — closer to OCaml's String.init style)

```rust
pub fn split_alternade_indexed(word: &str) -> (String, String) {
    let chars: Vec<char> = word.chars().collect();
    let n = chars.len();
    let even: String = (0..n).step_by(2).map(|i| chars[i]).collect();
    let odd:  String = (1..n).step_by(2).map(|i| chars[i]).collect();
    (even, odd)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Split function | `val get_alternade : StrSet.t -> string -> string option` | `fn split_alternade(word: &str) -> (String, String)` |
| Word set | `StrSet.t` (balanced BST, `Set.Make(String)`) | `HashSet<&str>` (hash table, borrows input) |
| String type | `string` (mutable byte array) | `&str` (borrowed) / `String` (owned) |
| Optional result | `'a option` | `Option<String>` |
| Main pipeline | `Seq.filter_map` (lazy) | `.filter_map()` (lazy, pulled eagerly by `.collect()`) |

## Key Insights

1. **`step_by` vs `String.init`:** Rust's `chars().step_by(2)` and `.skip(1).step_by(2)` replace OCaml's `String.init (ceil_len) (fun i -> s.[i*2])` arithmetic. The iterator form is cleaner and avoids index arithmetic bugs.

2. **HashSet vs BST Set:** OCaml's `Set.Make(String)` is a persistent balanced BST giving O(log n) membership; Rust's `HashSet<&str>` gives O(1) average. For dictionary lookup over thousands of words, this matters.

3. **Zero-copy string keys:** Rust's `HashSet<&str>` stores borrowed pointers into the original `&[&str]` slice — no heap copies. OCaml's `StrSet` stores full string copies in the BST nodes.

4. **Unicode correctness:** OCaml's `s.[i]` indexes bytes — correct only for ASCII. Rust's `chars()` decodes UTF-8 code points, making the alternade split Unicode-safe by default.

5. **Lazy vs eager pipeline:** OCaml's `Seq` is truly lazy (co-routine style). Rust's iterator chain is also lazy but driven eagerly by `.collect()`. Both avoid materializing intermediate collections.

## When to Use Each Style

**Use idiomatic Rust (iterator stride):** Always — it's the most readable, avoids index arithmetic, and is Unicode-correct.

**Use indexed Rust:** When you need random access later in the same function, or when the logic mirrors a known mathematical index formula directly from a spec.
