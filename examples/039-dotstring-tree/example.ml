(* Dotstring Tree *)
(* OCaml 99 Problems #39 *)

type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree

let node x l r = Node (x, l, r)
let leaf = Leaf

let rec to_dotstring = function
  | Leaf -> "."
  | Node (c, l, r) -> String.make 1 c ^ to_dotstring l ^ to_dotstring r

(* Tests *)
let () =
  let t = node 'a' (node 'b' leaf leaf) (node 'c' leaf leaf) in
  assert (to_dotstring t = "ab..c..");
  assert (to_dotstring leaf = ".");
  let deeper = node 'a' (node 'b' (node 'd' leaf leaf) leaf) leaf in
  assert (to_dotstring deeper = "abd....");
  print_endline "✓ OCaml tests passed"
