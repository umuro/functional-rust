Convert this OCaml example to idiomatic Rust.

Directory: examples/260-functor-comparable-set/

## OCaml source
```ocaml
module type COMPARABLE = sig
  type t
  val compare : t -> t -> int
end

module MakeSet (C : COMPARABLE) = struct
  type t = C.t list
  let empty = []
  let mem x = List.exists (fun y -> C.compare x y = 0)
  let add x s = if mem x s then s else x :: s
  let to_list s = List.sort C.compare s
end

module IntSet = MakeSet(Int)
module StringSet = MakeSet(String)

let () =
  let s = IntSet.(empty |> add 3 |> add 1 |> add 3 |> add 2) in
  List.iter (Printf.printf "%d ") (IntSet.to_list s);
  print_newline ()
```

## Topic
Creating a custom Set using the Map.Make functor pattern — Comparable Set
Difficulty: Advanced | Category: Functors and modules

Read CLAUDE.md in this directory — it defines all quality standards, file structure, and self-verification steps. Follow it exactly.

When done, report:
DONE — 260-functor-comparable-set — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]
