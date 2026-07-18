(* Binary Tree *)
(* OCaml 99 Problems #29 *)

type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree

let node x l r = Node (x, l, r)
let leaf = Leaf

let rec size = function
  | Leaf -> 0
  | Node (_, l, r) -> 1 + size l + size r

let rec depth = function
  | Leaf -> 0
  | Node (_, l, r) -> 1 + max (depth l) (depth r)

let rec mem x = function
  | Leaf -> false
  | Node (v, l, r) -> v = x || mem x l || mem x r

(* Tests *)
let () =
  let sample = node 1 (node 2 leaf leaf) (node 3 (node 4 leaf leaf) leaf) in
  assert (size sample = 4);
  assert (size leaf = 0);
  assert (depth sample = 3);
  assert (depth leaf = 0);
  assert (mem 4 sample = true);
  assert (mem 1 sample = true);
  assert (mem 5 sample = false);
  print_endline "✓ OCaml tests passed"
