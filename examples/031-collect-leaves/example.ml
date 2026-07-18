(* Collect Leaves *)
(* OCaml 99 Problems #31 *)

type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree

let node x l r = Node (x, l, r)
let leaf = Leaf

let rec collect_leaves = function
  | Leaf -> []
  | Node (x, Leaf, Leaf) -> [x]
  | Node (_, l, r) -> collect_leaves l @ collect_leaves r

(* Tests *)
let () =
  let sample = node 1 (node 2 leaf leaf) (node 3 (node 4 leaf leaf) leaf) in
  assert (collect_leaves sample = [2; 4]);
  assert (collect_leaves (node 9 leaf leaf) = [9]);
  assert (collect_leaves leaf = []);
  print_endline "✓ OCaml tests passed"
