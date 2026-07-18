(* Internal Nodes *)
(* OCaml 99 Problems #32 *)

type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree

let node x l r = Node (x, l, r)
let leaf = Leaf

let rec collect_internal = function
  | Leaf -> []
  | Node (_, Leaf, Leaf) -> []
  | Node (x, l, r) -> x :: collect_internal l @ collect_internal r

(* Tests *)
let () =
  let sample = node 1 (node 2 leaf leaf) (node 3 (node 4 leaf leaf) leaf) in
  assert (collect_internal sample = [1; 3]);
  assert (collect_internal (node 9 leaf leaf) = []);
  assert (collect_internal leaf = []);
  print_endline "✓ OCaml tests passed"
