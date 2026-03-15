(* 1028: BTreeSet — Union, Intersection, Difference *)
(* OCaml's Set module provides sorted set operations *)

module IntSet = Set.Make(Int)

(* Approach 1: Basic set operations *)
let basic_ops () =
  let a = IntSet.of_list [1; 2; 3; 4; 5] in
  let b = IntSet.of_list [3; 4; 5; 6; 7] in
  let union = IntSet.union a b in
  let inter = IntSet.inter a b in
  let diff = IntSet.diff a b in
  assert (IntSet.elements union = [1; 2; 3; 4; 5; 6; 7]);
  assert (IntSet.elements inter = [3; 4; 5]);
  assert (IntSet.elements diff = [1; 2])

(* Approach 2: Subset and disjoint checks *)
let subset_checks () =
  let small = IntSet.of_list [2; 3] in
  let big = IntSet.of_list [1; 2; 3; 4] in
  let other = IntSet.of_list [5; 6] in
  assert (IntSet.subset small big);
  assert (not (IntSet.subset big small));
  assert (IntSet.disjoint small other)

(* Approach 3: Fold-based operations *)
let fold_ops () =
  let s = IntSet.of_list [1; 2; 3; 4; 5] in
  let sum = IntSet.fold (fun x acc -> x + acc) s 0 in
  assert (sum = 15);
  let evens = IntSet.filter (fun x -> x mod 2 = 0) s in
  assert (IntSet.elements evens = [2; 4])

let () =
  basic_ops ();
  subset_checks ();
  fold_ops ();
  Printf.printf "✓ All tests passed\n"
