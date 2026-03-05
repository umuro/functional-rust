## Core Insight

Higher-order functions abstract iteration patterns. OCaml uses `List.map`, `List.filter`, `List.fold_left` on linked lists. Rust uses `.iter().map()`, `.filter()`, `.fold()` on iterator chains.

## OCaml Approach
- `List.map f lst` applies `f` to every element
- `List.filter pred lst` keeps elements where `pred` is true
- `List.fold_left f acc lst` reduces left-to-right
- Functions are curried by default — partial application is free

## Rust Approach
- `.iter().map(|x| ...)` with closures
- `.iter().filter(|x| ...)` — note double reference `&&x`
- `.iter().fold(init, |acc, x| ...)` — accumulator first
- Closures capture environment; `Fn`, `FnMut`, `FnOnce` traits

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Map | `List.map f lst` | `.iter().map(\|x\| f(x))` |
| Filter | `List.filter p lst` | `.iter().filter(\|x\| p(x))` |
| Fold | `List.fold_left f a lst` | `.iter().fold(a, \|acc, x\| ...)` |
| Currying | Native | Closures returning closures |
| Evaluation | Eager (list) | Lazy (iterator chain) |
