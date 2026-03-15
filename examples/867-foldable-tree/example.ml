(* Example 068: Foldable for Binary Tree *)
(* Left/right fold, collect all values, various aggregations *)

type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

(* Approach 1: In-order fold (left, value, right) *)
let rec fold_inorder f acc = function
  | Leaf -> acc
  | Node (l, v, r) ->
    let acc = fold_inorder f acc l in
    let acc = f acc v in
    fold_inorder f acc r

(* Approach 2: Pre-order and post-order *)
let rec fold_preorder f acc = function
  | Leaf -> acc
  | Node (l, v, r) ->
    let acc = f acc v in
    let acc = fold_preorder f acc l in
    fold_preorder f acc r

let rec fold_postorder f acc = function
  | Leaf -> acc
  | Node (l, v, r) ->
    let acc = fold_postorder f acc l in
    let acc = fold_postorder f acc r in
    f acc v

(* Approach 3: Derived operations *)
let to_list_inorder tree = List.rev (fold_inorder (fun acc x -> x :: acc) [] tree)
let sum tree = fold_inorder (+) 0 tree
let max_val tree = fold_inorder (fun acc x -> max acc x) min_int tree
let all pred tree = fold_inorder (fun acc x -> acc && pred x) true tree
let any pred tree = fold_inorder (fun acc x -> acc || pred x) false tree
let count pred tree = fold_inorder (fun acc x -> if pred x then acc + 1 else acc) 0 tree

let () =
  let tree = Node (Node (Leaf, 1, Leaf), 2, Node (Node (Leaf, 3, Leaf), 4, Node (Leaf, 5, Leaf))) in

  assert (to_list_inorder tree = [1; 2; 3; 4; 5]);
  assert (sum tree = 15);
  assert (max_val tree = 5);
  assert (all (fun x -> x > 0) tree = true);
  assert (all (fun x -> x > 2) tree = false);
  assert (any (fun x -> x = 3) tree = true);
  assert (count (fun x -> x mod 2 = 0) tree = 2);

  (* Pre-order: root first *)
  let pre = List.rev (fold_preorder (fun acc x -> x :: acc) [] tree) in
  assert (pre = [2; 1; 4; 3; 5]);

  (* Post-order: root last *)
  let post = List.rev (fold_postorder (fun acc x -> x :: acc) [] tree) in
  assert (post = [1; 3; 5; 4; 2]);

  Printf.printf "✓ All tests passed\n"
