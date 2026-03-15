(* 1028: BTreeSet — Union, Intersection, Difference
   OCaml's Set functor provides a sorted, immutable set with all set operations.
   Set.union, Set.inter, Set.diff, Set.subset, Set.disjoint (4.12+). *)

module IntSet = Set.Make(Int)

let of_list lst =
  List.fold_left (fun acc x -> IntSet.add x acc) IntSet.empty lst

let to_sorted_list s = IntSet.elements s  (* always sorted *)

(* Basic set operations *)
let basic_ops () =
  let a = of_list [1; 2; 3; 4; 5] in
  let b = of_list [3; 4; 5; 6; 7] in

  assert (to_sorted_list (IntSet.union a b) = [1; 2; 3; 4; 5; 6; 7]);
  assert (to_sorted_list (IntSet.inter a b) = [3; 4; 5]);
  assert (to_sorted_list (IntSet.diff  a b) = [1; 2]);

  (* Symmetric difference: elements in either but not both *)
  let sym_diff = IntSet.union (IntSet.diff a b) (IntSet.diff b a) in
  assert (to_sorted_list sym_diff = [1; 2; 6; 7])

(* Subset and disjoint checks *)
let subset_checks () =
  let small = of_list [2; 3] in
  let big   = of_list [1; 2; 3; 4] in
  let other = of_list [5; 6] in

  assert (IntSet.subset small big);
  assert (not (IntSet.subset big small));

  (* superset: small ⊆ big *)
  assert (IntSet.subset small big);

  (* disjoint — available in OCaml 4.12+ *)
  assert (IntSet.disjoint small other)

(* Iterator-based operations *)
let iter_ops () =
  let s = of_list [1; 2; 3; 4; 5] in
  let sum = IntSet.fold (+) s 0 in
  assert (sum = 15);

  let evens = IntSet.filter (fun x -> x mod 2 = 0) s in
  assert (to_sorted_list evens = [2; 4]);

  (* Range query *)
  let range = IntSet.filter (fun x -> x >= 2 && x <= 4) s in
  assert (to_sorted_list range = [2; 3; 4])

let () =
  basic_ops ();
  subset_checks ();
  iter_ops ();

  (* Operator-style via union/inter *)
  let a = of_list [1; 2; 3] in
  let b = of_list [2; 3; 4] in
  assert (to_sorted_list (IntSet.union a b) = [1; 2; 3; 4]);
  assert (to_sorted_list (IntSet.inter a b) = [2; 3]);

  Printf.printf "BTreeSet (Set) tests passed\n"
