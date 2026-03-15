(* Example 214: Fold — Read-Only Traversal for Aggregating *)

(* A Fold is a read-only traversal: it can extract multiple values
   but cannot modify them. Think of it as a generalized "toList". *)

type ('s, 'a) fold = {
  fold_map : 'b. ('a -> 'b) -> ('b -> 'b -> 'b) -> 'b -> 's -> 'b;
}

(* Simpler encoding for practical use *)
type ('s, 'a) fold_simple = {
  to_list : 's -> 'a list;
}

(* Approach 1: Basic fold combinators *)
let to_list_of f s = f.to_list s
let length_of f s = List.length (f.to_list s)
let sum_of f s = List.fold_left ( + ) 0 (f.to_list s)
let sum_of_float f s = List.fold_left ( +. ) 0.0 (f.to_list s)
let product_of f s = List.fold_left ( * ) 1 (f.to_list s)
let any_of f pred s = List.exists pred (f.to_list s)
let all_of f pred s = List.for_all pred (f.to_list s)
let find_of f pred s = List.find_opt pred (f.to_list s)
let max_of f s = match f.to_list s with [] -> None | xs -> Some (List.fold_left max (List.hd xs) xs)
let min_of f s = match f.to_list s with [] -> None | xs -> Some (List.fold_left min (List.hd xs) xs)

(* Approach 2: Fold for different structures *)
let list_fold : ('a list, 'a) fold_simple = { to_list = Fun.id }

type 'a tree = Leaf of 'a | Branch of 'a tree * 'a tree

let rec tree_to_list = function
  | Leaf x -> [x]
  | Branch (l, r) -> tree_to_list l @ tree_to_list r

let tree_fold : ('a tree, 'a) fold_simple = { to_list = tree_to_list }

(* Fold from a lens (lens is also a fold with exactly 1 element) *)
let lens_to_fold get = { to_list = (fun s -> [get s]) }

type person = { name : string; scores : int list }

let scores_fold : (person, int) fold_simple = { to_list = (fun p -> p.scores) }

(* Approach 3: Composing folds *)
let compose_fold (outer : ('s, 'a) fold_simple) (inner : ('a, 'b) fold_simple) : ('s, 'b) fold_simple = {
  to_list = (fun s -> List.concat_map inner.to_list (outer.to_list s));
}

type team = { members : person list }

let members_fold : (team, person) fold_simple = { to_list = (fun t -> t.members) }

let team_scores = compose_fold members_fold scores_fold

(* === Tests === *)
let () =
  (* List fold *)
  let xs = [3; 1; 4; 1; 5] in
  assert (to_list_of list_fold xs = [3; 1; 4; 1; 5]);
  assert (length_of list_fold xs = 5);
  assert (sum_of list_fold xs = 14);
  assert (product_of list_fold xs = 60);
  assert (max_of list_fold xs = Some 5);
  assert (min_of list_fold xs = Some 1);

  (* Tree fold *)
  let tree = Branch (Branch (Leaf 10, Leaf 20), Leaf 30) in
  assert (sum_of tree_fold tree = 60);
  assert (length_of tree_fold tree = 3);

  (* Person scores fold *)
  let alice = { name = "Alice"; scores = [90; 85; 95] } in
  assert (sum_of scores_fold alice = 270);
  assert (all_of scores_fold (fun s -> s >= 80) alice);
  assert (max_of scores_fold alice = Some 95);

  (* Composed fold: team → all scores *)
  let team = { members = [
    { name = "Alice"; scores = [90; 85] };
    { name = "Bob"; scores = [70; 95; 80] };
  ] } in
  assert (to_list_of team_scores team = [90; 85; 70; 95; 80]);
  assert (sum_of team_scores team = 420);
  assert (length_of team_scores team = 5);

  (* Empty cases *)
  assert (sum_of list_fold [] = 0);
  assert (max_of list_fold [] = None);

  print_endline "✓ All tests passed"
