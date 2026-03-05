(* Example 108: Rc<T> — OCaml Default Sharing → Rust Explicit Rc *)

(* In OCaml, all heap values are GC-managed. Multiple bindings to the
   same value is the default — no annotation needed. *)

(* Approach 1: Shared tree nodes — implicit in OCaml *)
type tree = Leaf | Node of tree * int * tree

let rec tree_sum = function
  | Leaf -> 0
  | Node (l, v, r) -> tree_sum l + v + tree_sum r

let shared_tree_demo () =
  let shared = Node (Leaf, 42, Leaf) in
  (* Both trees reference the same `shared` node — GC tracks liveness *)
  let tree1 = Node (shared, 1, Leaf) in
  let tree2 = Node (Leaf, 2, shared) in
  (tree_sum tree1, tree_sum tree2)

(* Approach 2: Shared-tail cons list *)
(* In OCaml, list tails are implicitly shared when you pattern-match *)
let make_shared_lists () =
  let tail = [3; 2; 1] in
  let list_a = 10 :: tail in
  let list_b = 20 :: tail in
  (list_a, list_b)

let () =
  let (s1, s2) = shared_tree_demo () in
  Printf.printf "tree1 sum = %d, tree2 sum = %d\n" s1 s2;
  assert (s1 = 43);
  assert (s2 = 44);

  let (a, b) = make_shared_lists () in
  assert (a = [10; 3; 2; 1]);
  assert (b = [20; 3; 2; 1]);

  print_endline "ok"
