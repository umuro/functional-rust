(* OCaml: recursive tree operations (sync, as Lwt analogy) *)

type tree = Leaf | Node of { value: int; left: tree; right: tree }

let leaf = Leaf
let node v l r = Node { value = v; left = l; right = r }

let rec sum_tree = function
  | Leaf -> 0
  | Node { value; left; right } -> value + sum_tree left + sum_tree right

let rec depth_tree = function
  | Leaf -> 0
  | Node { left; right; _ } -> 1 + max (depth_tree left) (depth_tree right)

let sample =
  node 1
    (node 2 (node 4 leaf leaf) (node 5 leaf leaf))
    (node 3 (node 6 leaf leaf) leaf)

let () =
  Printf.printf "Sum: %d\n" (sum_tree sample);
  Printf.printf "Depth: %d\n" (depth_tree sample)
