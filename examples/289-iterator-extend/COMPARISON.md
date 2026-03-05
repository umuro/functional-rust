# OCaml vs Rust: extend()

## Pattern 1: Append to Collection

### OCaml
```ocaml
let base = [1; 2; 3] in
let extension = [4; 5; 6] in
let combined = base @ extension  (* creates new list *)
```

### Rust
```rust
let mut base = vec![1, 2, 3];
base.extend([4, 5, 6]);  // in-place, reuses allocation
```

## Pattern 2: String Building

### OCaml
```ocaml
let buf = Buffer.create 16 in
Buffer.add_string buf "hello";
Buffer.add_string buf " world";
Buffer.contents buf
```

### Rust
```rust
let mut s = String::from("Hello");
s.extend(", world!".chars());
// or: s.push_str(", world!");
```

## Pattern 3: Incremental Building

### Rust
```rust
let mut result = Vec::new();
for batch in batches {
    result.extend(batch);  // efficient - reuses allocation
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| List append | `@` creates new list | `extend` modifies in-place |
| Allocation | Always new | Reuses when possible |
| String | `Buffer.add_string` | `extend(chars)` or `push_str` |
| Maps | `fold_left` + insert | `map.extend(pairs)` |
| vs chain+collect | Equivalent result | `extend` avoids new allocation |
