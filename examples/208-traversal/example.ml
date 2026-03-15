(* Example 208: Traversal — Focus on Zero or More Targets *)

(* A traversal focuses on 0-to-many values inside a structure *)
type ('s, 'a) traversal = {
  over   : ('a -> 'a) -> 's -> 's;    (* modify all focused values *)
  to_list : 's -> 'a list;             (* collect all focused values *)
}

(* Approach 1: Traversal over list elements *)
let each_traversal : ('a list, 'a) traversal = {
  over = List.map;
  to_list = Fun.id;
}

(* Approach 2: Traversal into a tree structure *)
type 'a tree = Leaf of 'a | Branch of 'a tree * 'a tree

let rec tree_over f = function
  | Leaf x -> Leaf (f x)
  | Branch (l, r) -> Branch (tree_over f l, tree_over f r)

let rec tree_to_list = function
  | Leaf x -> [x]
  | Branch (l, r) -> tree_to_list l @ tree_to_list r

let each_leaf : ('a tree, 'a) traversal = {
  over = tree_over;
  to_list = tree_to_list;
}

(* Approach 3: Traversal combinators *)
let length_of t s = List.length (t.to_list s)
let sum_of t s = List.fold_left ( + ) 0 (t.to_list s)
let all_of t pred s = List.for_all pred (t.to_list s)
let any_of t pred s = List.exists pred (t.to_list s)
let find_of t pred s = List.find_opt pred (t.to_list s)

(* Filtered traversal *)
let filtered pred (t : ('s, 'a) traversal) : ('s, 'a) traversal = {
  over = (fun f s -> t.over (fun a -> if pred a then f a else a) s);
  to_list = (fun s -> List.filter pred (t.to_list s));
}

(* === Tests === *)
let () =
  (* List traversal *)
  let xs = [1; 2; 3; 4; 5] in
  assert (each_traversal.to_list xs = [1; 2; 3; 4; 5]);
  assert (each_traversal.over (fun x -> x * 2) xs = [2; 4; 6; 8; 10]);
  assert (length_of each_traversal xs = 5);
  assert (sum_of each_traversal xs = 15);

  (* Tree traversal *)
  let tree = Branch (Branch (Leaf 1, Leaf 2), Leaf 3) in
  assert (each_leaf.to_list tree = [1; 2; 3]);
  let tree2 = each_leaf.over (fun x -> x + 10) tree in
  assert (each_leaf.to_list tree2 = [11; 12; 13]);
  assert (sum_of each_leaf tree = 6);

  (* Filtered traversal *)
  let evens = filtered (fun x -> x mod 2 = 0) each_traversal in
  assert (evens.to_list xs = [2; 4]);
  let xs2 = evens.over (fun x -> x * 10) xs in
  assert (xs2 = [1; 20; 3; 40; 5]);

  (* Combinators *)
  assert (all_of each_traversal (fun x -> x > 0) xs);
  assert (not (all_of each_traversal (fun x -> x > 3) xs));
  assert (any_of each_traversal (fun x -> x = 3) xs);
  assert (find_of each_traversal (fun x -> x > 3) xs = Some 4);

  print_endline "✓ All tests passed"
