# OCaml vs Rust: Sierpinski Triangle

## Side-by-Side Code

### OCaml
```ocaml
let sierpinski n =
  let rec go n =
    if n = 0 then ["*"]
    else
      let prev = go (n - 1) in
      let width = 1 lsl n - 1 in
      let pad s = String.make ((width - String.length s) / 2) ' ' ^ s in
      let top = List.map pad prev in
      let bottom = List.map (fun s -> s ^ " " ^ s) prev in
      top @ bottom
  in
  List.iter print_endline (go n)
```

### Rust (idiomatic)
```rust
pub fn sierpinski(n: u32) -> Vec<String> {
    if n == 0 {
        return vec!["*".to_string()];
    }
    let prev = sierpinski(n - 1);
    let width = (1 << n) - 1;

    let top: Vec<String> = prev.iter()
        .map(|s| {
            let pad = (width - s.len()) / 2;
            format!("{}{}", " ".repeat(pad), s)
        })
        .collect();

    let bottom: Vec<String> = prev.iter()
        .map(|s| format!("{} {}", s, s))
        .collect();

    [top, bottom].concat()
}
```

### Rust (functional/iterative fold)
```rust
pub fn sierpinski_iter(n: u32) -> Vec<String> {
    (0..n).fold(vec!["*".to_string()], |prev, _| {
        let width = prev.last().map_or(1, |s| s.len() * 2 + 1);
        let top: Vec<String> = prev.iter()
            .map(|s| {
                let pad = (width - s.len()) / 2;
                format!("{}{}", " ".repeat(pad), s)
            })
            .collect();
        let bottom: Vec<String> = prev.iter()
            .map(|s| format!("{} {}", s, s))
            .collect();
        [top, bottom].concat()
    })
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Return type | `string list` (implicitly, via `go`) | `Vec<String>` |
| Line padding | `String.make n ' ' ^ s` | `format!("{}{}", " ".repeat(pad), s)` |
| Line duplication | `s ^ " " ^ s` | `format!("{} {}", s, s)` |
| List concat | `top @ bottom` | `[top, bottom].concat()` |
| Bit shift | `1 lsl n` | `1 << n` |

## Key Insights

1. **Structural recursion translates directly:** The recursive decomposition (base case → pad top → duplicate bottom → concatenate) maps 1:1 from OCaml to Rust, showing that functional algorithms transfer cleanly across languages
2. **String building patterns:** OCaml uses `^` for concatenation; Rust's `format!` macro is more flexible and avoids intermediate allocations when building complex strings
3. **List vs Vec for recursive generation:** OCaml's linked list has O(1) prepend and O(n) append (`@`); Rust's Vec has O(1) push and O(n) concat — similar asymptotic cost, different cache behavior
4. **Inner function vs top-level:** OCaml uses `let rec go` as an inner helper; Rust uses the public function itself as the recursion — Rust doesn't need the indirection since there's no separate printing concern
5. **Fold as iteration:** The `(0..n).fold(...)` version shows how recursion over a counter naturally becomes fold over a range — Rust's range iterators make this particularly clean

## When to Use Each Style

**Use idiomatic Rust when:** The recursive structure makes the algorithm clearest — fractal generation is inherently self-similar, and the recursive version communicates that directly.

**Use iterative Rust when:** You want to avoid stack depth concerns for large orders, or when composing with other iterator operations. The fold version keeps the same logic but avoids function call overhead.
