(* Complete Tree *)
(* OCaml 99 Problems #34 *)

type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree

let rec count_nodes = function
  | Leaf -> 0
  | Node (_, l, r) -> 1 + count_nodes l + count_nodes r

let rec complete_binary_tree n =
  if n = 0 then Leaf
  else
    let remaining = n - 1 in
    let l = remaining / 2 + (remaining mod 2) in
    Node ('x', complete_binary_tree l, complete_binary_tree (remaining - l))

(* Tests *)
let () =
  for n = 0 to 19 do
    assert (count_nodes (complete_binary_tree n) = n)
  done;
  assert (complete_binary_tree 0 = Leaf);
  print_endline "✓ OCaml tests passed"
