Convert this OCaml example to idiomatic Rust.

Directory: examples/256-memoization-fibonacci/

## OCaml source
```ocaml
let memoize f =
  let cache = Hashtbl.create 16 in
  fun x ->
    match Hashtbl.find_opt cache x with
    | Some v -> v
    | None ->
      let v = f x in
      Hashtbl.add cache x v;
      v

let fib =
  let rec fib' n =
    if n <= 1 then n
    else memo_fib (n - 1) + memo_fib (n - 2)
  and memo_fib = memoize fib'
  in memo_fib

let () = Printf.printf "fib(35) = %d\n" (fib 35)
```

## Topic
Transparent memoization using a hash table wrapper — Fibonacci with Hashtable Cache
Difficulty: Intermediate | Category: Memoization

Read CLAUDE.md in this directory — it defines all quality standards, file structure, and self-verification steps. Follow it exactly.

When done, report:
DONE — 256-memoization-fibonacci — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]
