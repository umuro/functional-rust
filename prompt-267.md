Convert this OCaml example to idiomatic Rust.

Directory: examples/267-zipper-list-cursor/

## OCaml source
```ocaml
type 'a zipper = { left: 'a list; focus: 'a; right: 'a list }

let of_list = function
  | [] -> failwith "empty"
  | h :: t -> { left = []; focus = h; right = t }

let go_right z = match z.right with
  | [] -> None
  | h :: t -> Some { left = z.focus :: z.left; focus = h; right = t }

let go_left z = match z.left with
  | [] -> None
  | h :: t -> Some { left = t; focus = h; right = z.focus :: z.right }

let update f z = { z with focus = f z.focus }
let to_list z = List.rev z.left @ [z.focus] @ z.right

let () =
  let z = of_list [1;2;3;4;5] in
  let z = Option.get (go_right z) in
  let z = Option.get (go_right z) in
  let z = update (fun x -> x * 10) z in
  List.iter (Printf.printf "%d ") (to_list z)
```

## Topic
Zipper data structure for O(1) local list navigation — functional cursor pattern.

Read CLAUDE.md in this directory — it defines all quality standards, file structure, and self-verification steps. Follow it exactly.

When done, report:
DONE — 267-zipper-list-cursor — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]
