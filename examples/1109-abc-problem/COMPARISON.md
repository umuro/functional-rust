# OCaml vs Rust: ABC Problem

## Side-by-Side Code

### OCaml
```ocaml
let blocks = [
  ('B','O');('X','K');('D','Q');('C','P');('N','A');('G','T');('R','E');('T','G');
  ('Q','D');('F','S');('J','W');('H','U');('V','I');('A','N');('O','B');('E','R');
  ('F','S');('L','Y');('P','C');('Z','M');
]

let find_letter blocks c =
  let found, remaining =
    List.partition (fun (c1, c2) -> c1 = c || c2 = c) blocks
  in
  match found with
  | _ :: res -> Some (res @ remaining)
  | _ -> None

let can_make_word w =
  let n = String.length w in
  let rec aux i _blocks =
    if i >= n then true else
      match find_letter _blocks w.[i] with
      | None -> false
      | Some rem_blocks -> aux (succ i) rem_blocks
  in
  aux 0 blocks
```

### Rust (idiomatic — index-based backtracking)
```rust
pub fn can_make_word(word: &str) -> bool {
    let letters: Vec<char> = word.to_uppercase().chars().collect();
    let available: Vec<usize> = (0..BLOCKS.len()).collect();
    can_spell(&letters, &available)
}

fn can_spell(letters: &[char], available: &[usize]) -> bool {
    match letters {
        [] => true,
        [c, rest @ ..] => available.iter().enumerate().any(|(pos, &idx)| {
            block_has(BLOCKS[idx], *c) && {
                let remaining: Vec<usize> = available[..pos]
                    .iter()
                    .chain(&available[pos + 1..])
                    .copied()
                    .collect();
                can_spell(rest, &remaining)
            }
        }),
    }
}
```

### Rust (functional — partition mirrors OCaml)
```rust
pub fn can_make_word_functional(word: &str) -> bool {
    let letters: Vec<char> = word.to_uppercase().chars().collect();
    can_spell_functional(&letters, BLOCKS.to_vec())
}

fn can_spell_functional(letters: &[char], blocks: Vec<(char, char)>) -> bool {
    match letters {
        [] => true,
        [c, rest @ ..] => {
            let (matching, non_matching): (Vec<_>, Vec<_>) =
                blocks.into_iter().partition(|&b| block_has(b, *c));
            matching.iter().enumerate().any(|(i, _)| {
                let remaining: Vec<_> = matching[..i]
                    .iter()
                    .chain(&matching[i + 1..])
                    .chain(&non_matching)
                    .copied()
                    .collect();
                can_spell_functional(rest, remaining)
            })
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Block set | `(char * char) list` | `&[(char, char)]` (static slice) |
| Word parameter | `string` | `&str` |
| Return type | `bool` | `bool` |
| Available pool | `(char * char) list` (rebuilt each step) | `Vec<usize>` (indices into static array) |
| Letter extraction | `w.[i]` (char by index) | `.chars().collect()` then slice patterns |

## Key Insights

1. **Greedy vs correct backtracking:** The OCaml `find_letter` picks only the *first* matching block (head of the `found` list), which happens to work for the Rosetta Code block set but is not generally correct. The Rust versions use `.any()` over all matching blocks, retrying every candidate before giving up — true backtracking.

2. **Index-based removal is idiomatic:** Rather than cloning the block tuple into a new `Vec` on each step, the idiomatic Rust solution stores *indices* into the static `BLOCKS` array. Removing index `pos` is a slice split + chain: `available[..pos].iter().chain(&available[pos+1..])`. No heap data is copied; only `usize` values are.

3. **Slice patterns replace recursive list deconstruction:** OCaml's `| _ :: rest -> ...` becomes `[c, rest @ ..] => ...` in Rust. Both destructure the head from the tail, but Rust's version works directly on `&[char]` slices rather than linked lists, enabling cache-friendly access.

4. **`partition` maps 1-to-1:** `List.partition` in OCaml and `Iterator::partition` in Rust have identical semantics: they split a collection into two groups based on a predicate. The functional Rust version is a near-literal translation, just with owned `Vec<(char,char)>` instead of OCaml's persistent lists.

5. **Case normalisation upfront vs per-character:** OCaml typically applies `Char.uppercase_ascii` at each character comparison site. Rust normalises the whole word once with `.to_uppercase()` and stores the result in a `Vec<char>`, avoiding repeated Unicode processing during the recursive search.

## When to Use Each Style

**Use idiomatic Rust (index-based) when:** you want minimal allocations and are working with a fixed global collection. The `Vec<usize>` pool is cheaper to clone than a `Vec<(char,char)>` because `usize` is smaller.

**Use functional Rust (partition-based) when:** translating OCaml code directly for pedagogical clarity, or when the available pool itself changes between invocations and cannot be referred to by a static index.
