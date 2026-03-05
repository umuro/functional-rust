# OCaml vs Rust: Parallel Letter Frequency

## Side-by-Side Code

### OCaml
```ocaml
module CMap = Map.Make(Char)

let letter_freq s =
  String.fold_left (fun m c ->
    let c = Char.lowercase_ascii c in
    if c >= 'a' && c <= 'z' then
      CMap.update c (function None -> Some 1 | Some n -> Some (n+1)) m
    else m
  ) CMap.empty s

let merge_maps =
  CMap.union (fun _ a b -> Some (a + b))

let parallel_frequency texts =
  texts |> List.map letter_freq |> List.fold_left merge_maps CMap.empty
```

### Rust (idiomatic)
```rust
fn letter_freq(s: &str) -> HashMap<char, usize> {
    s.chars().fold(HashMap::new(), |mut map, c| {
        let c = c.to_ascii_lowercase();
        if c.is_ascii_lowercase() {
            *map.entry(c).or_insert(0) += 1;
        }
        map
    })
}

fn merge_maps(mut a: HashMap<char, usize>, b: &HashMap<char, usize>) -> HashMap<char, usize> {
    for (&ch, &count) in b {
        *a.entry(ch).or_insert(0) += count;
    }
    a
}

fn parallel_frequency(texts: &[&str]) -> HashMap<char, usize> {
    texts.iter().map(|t| letter_freq(t)).fold(HashMap::new(), |acc, f| merge_maps(acc, &f))
}
```

### Rust (functional/recursive)
```rust
fn parallel_frequency_recursive(texts: &[&str]) -> HashMap<char, usize> {
    match texts {
        [] => HashMap::new(),
        [single] => letter_freq(single),
        [head, rest @ ..] => {
            let head_freq = letter_freq(head);
            let rest_freq = parallel_frequency_recursive(rest);
            merge_maps(head_freq, &rest_freq)
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Frequency map | `char CMap.t` (via `Map.Make(Char)`) | `HashMap<char, usize>` |
| Count type | `int` | `usize` |
| Text list | `string list` | `&[&str]` (slice of string slices) |
| Update function | `CMap.update : key -> (val option -> val option) -> map -> map` | `map.entry(key).or_insert(default)` returns `&mut V` |
| Merge function | `CMap.union : (key -> val -> val -> val option) -> map -> map -> map` | Manual iteration + entry API |

## Key Insights

1. **Functor vs generic:** OCaml needs `Map.Make(Char)` to create a char-keyed map module; Rust's `HashMap<K, V>` is generic out of the box — no functor ceremony needed
2. **Immutable vs mutable maps:** OCaml's `CMap.update` returns a new map each time (persistent data structure); Rust mutates the HashMap in place, which is more cache-friendly
3. **Entry API elegance:** Rust's `entry().or_insert()` pattern replaces OCaml's `function None -> Some 1 | Some n -> Some (n+1)` — less boilerplate for the common "upsert" pattern
4. **Union vs manual merge:** OCaml's `CMap.union` is a single elegant call with a merge function; Rust has no built-in HashMap merge, requiring explicit iteration
5. **Pipeline preservation:** Both languages express map-reduce cleanly — OCaml with `|>` pipeline, Rust with `.iter().map().fold()` method chain

## When to Use Each Style

**Use idiomatic Rust when:** You want maximum performance — in-place mutation avoids allocation overhead of creating new maps on every insert. This is the natural Rust approach for frequency counting.

**Use recursive Rust when:** Teaching the map-reduce concept explicitly — the recursive decomposition `[head, rest @ ..]` makes the "divide and conquer" structure visible, matching OCaml's mental model of list processing.
