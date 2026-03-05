# OCaml vs Rust: Polonius / Advanced Borrow Checking

## OCaml
```ocaml
(* No borrow checking — hashtbl mutation is direct *)
let get_or_insert tbl key =
  try Hashtbl.find tbl key
  with Not_found ->
    let v = "default_" ^ key in
    Hashtbl.add tbl key v;
    v
```

## Rust (NLL workaround)
```rust
// NLL rejects: borrow in match conflicts with insert
// Workaround: check first, then mutate
pub fn get_or_insert<'a>(
    map: &'a mut HashMap<String, String>,
    key: String,
) -> &'a str {
    if !map.contains_key(&key) {
        map.insert(key.clone(), format!("default_{}", key));
    }
    map.get(&key).unwrap()
}

// Or use entry API
map.entry(key).or_insert_with(|| compute())
```

## Key Differences

1. **OCaml**: No borrow tracking, direct mutation
2. **Rust NLL**: Conservative in some patterns
3. **Rust Polonius**: More precise flow analysis (experimental)
4. **Rust**: Entry API is idiomatic workaround
5. Both: Achieve same functionality, different ergonomics
