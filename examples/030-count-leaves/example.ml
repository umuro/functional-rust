(* Count Leaves *)
(* OCaml 99 Problems #30 *)

type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree

let node x l r = Node (x, l, r)
let leaf = Leaf

let rec count_leaves = function
  | Leaf -> 0
  | Node (_, Leaf, Leaf) -> 1
  | Node (_, l, r) -> count_leaves l + count_leaves r

let rec count_nodes = function
  | Leaf -> 0
  | Node (_, l, r) -> 1 + count_nodes l + count_nodes r

(* Tests *)
let () =
  let sample = node 1 (node 2 leaf leaf) (node 3 (node 4 leaf leaf) leaf) in
  assert (count_leaves sample = 2);
  assert (count_leaves (node 1 leaf leaf) = 1);
  assert (count_leaves leaf = 0);
  assert (count_leaves sample + 2 = count_nodes sample);
  print_endline "✓ OCaml tests passed"
