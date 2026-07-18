(* At Level *)
(* OCaml 99 Problems #33 *)

type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree

let node x l r = Node (x, l, r)
let leaf = Leaf

let rec at_level tree level =
  match tree with
  | Leaf -> []
  | Node (x, _, _) when level = 1 -> [x]
  | Node (_, l, r) -> at_level l (level - 1) @ at_level r (level - 1)

(* Tests *)
let () =
  let sample = node 1 (node 2 leaf leaf) (node 3 (node 4 leaf leaf) leaf) in
  assert (at_level sample 1 = [1]);
  assert (at_level sample 2 = [2; 3]);
  assert (at_level sample 3 = [4]);
  assert (at_level sample 4 = []);
  print_endline "✓ OCaml tests passed"
