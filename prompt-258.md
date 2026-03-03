Convert this OCaml example to idiomatic Rust.

Directory: examples/258-monadic-option-chaining/

## OCaml source
```ocaml
let ( >>= ) opt f = match opt with
  | None -> None
  | Some x -> f x

let ( >>| ) opt f = match opt with
  | None -> None
  | Some x -> Some (f x)

let safe_div x y = if y = 0 then None else Some (x / y)
let safe_head = function [] -> None | h :: _ -> Some h

let compute lst =
  safe_head lst >>= fun x ->
  safe_div 100 x >>| fun r ->
  r * 2

let () =
  let show = function None -> "None" | Some x -> string_of_int x in
  Printf.printf "%s\n" (show (compute [5; 3; 1]));
  Printf.printf "%s\n" (show (compute [0; 1]));
  Printf.printf "%s\n" (show (compute []))
```

## Topic
Option monad bind (>>=) for safe chaining without nested matches
Difficulty: Intermediate | Category: Monadic patterns

Read CLAUDE.md in this directory — it defines all quality standards, file structure, and self-verification steps. Follow it exactly.

When done, report:
DONE — 258-monadic-option-chaining — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]
