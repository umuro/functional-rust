(* Example 216: Fix Point — Unrolling Recursion from a Functor *)

(* The key insight: separate the SHAPE of data from RECURSION itself.
   type 'a list_f = NilF | ConsF of int * 'a    (* one layer, non-recursive *)
   type fix_list = fix_list list_f               (* recursion via fix point *)
*)

(* Approach 1: Fix point for lists *)
type 'a list_f = NilF | ConsF of int * 'a

let map_list_f f = function
  | NilF -> NilF
  | ConsF (x, rest) -> ConsF (x, f rest)

type fix_list = FixL of fix_list list_f

let unfix_l (FixL f) = f

(* Build lists *)
let nil = FixL NilF
let cons x xs = FixL (ConsF (x, xs))

(* cata for lists *)
let rec cata_list alg (FixL f) =
  alg (map_list_f (cata_list alg) f)

(* Approach 2: Fix point for binary trees *)
type 'a tree_f = LeafF of int | BranchF of 'a * 'a

let map_tree_f f = function
  | LeafF n -> LeafF n
  | BranchF (l, r) -> BranchF (f l, f r)

type fix_tree = FixT of fix_tree tree_f

let unfix_t (FixT f) = f

let leaf n = FixT (LeafF n)
let branch l r = FixT (BranchF (l, r))

let rec cata_tree alg (FixT f) =
  alg (map_tree_f (cata_tree alg) f)

(* Approach 3: Generic fix point (using polymorphic variant or functor) *)
(* In OCaml, we can use a functor module *)
module type FUNCTOR = sig
  type 'a t
  val map : ('a -> 'b) -> 'a t -> 'b t
end

module Fix (F : FUNCTOR) = struct
  type t = In of t F.t
  let out (In f) = f
  let rec cata alg (In f) =
    alg (F.map (cata alg) f)
end

(* === Tests === *)
let () =
  (* List fix point *)
  let xs = cons 1 (cons 2 (cons 3 nil)) in

  let sum_alg = function NilF -> 0 | ConsF (x, acc) -> x + acc in
  assert (cata_list sum_alg xs = 6);

  let length_alg = function NilF -> 0 | ConsF (_, acc) -> 1 + acc in
  assert (cata_list length_alg xs = 3);

  let to_list_alg = function NilF -> [] | ConsF (x, acc) -> x :: acc in
  assert (cata_list to_list_alg xs = [1; 2; 3]);

  (* Tree fix point *)
  let tree = branch (branch (leaf 1) (leaf 2)) (leaf 3) in

  let sum_tree = function LeafF n -> n | BranchF (l, r) -> l + r in
  assert (cata_tree sum_tree tree = 6);

  let depth_tree = function LeafF _ -> 0 | BranchF (l, r) -> 1 + max l r in
  assert (cata_tree depth_tree tree = 2);

  let count_tree = function LeafF _ -> 1 | BranchF (l, r) -> l + r in
  assert (cata_tree count_tree tree = 3);

  print_endline "✓ All tests passed"
