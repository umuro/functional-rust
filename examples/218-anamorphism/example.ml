(* Example 218: Anamorphism — Unfold to Build Recursive Structures *)

(* ana : ('a -> 'f 'a) -> 'a -> fix
   The dual of cata: builds UP a structure from a seed. *)

type 'a list_f = NilF | ConsF of int * 'a

let map_lf f = function NilF -> NilF | ConsF (x, a) -> ConsF (x, f a)

type fix_list = FixL of fix_list list_f

let rec ana coalg seed =
  FixL (map_lf (ana coalg) (coalg seed))

let rec cata alg (FixL f) =
  alg (map_lf (cata alg) f)

(* Approach 1: Build a range [lo..hi] *)
let range_coalg (lo, hi) =
  if lo > hi then NilF
  else ConsF (lo, (lo + 1, hi))

let range lo hi = ana range_coalg (lo, hi)

(* Approach 2: Build countdown *)
let countdown_coalg n =
  if n <= 0 then NilF
  else ConsF (n, n - 1)

let countdown n = ana countdown_coalg n

(* Approach 3: Collatz sequence *)
let collatz_coalg n =
  if n <= 1 then ConsF (1, 0)  (* terminal *)
  else if n = 0 then NilF
  else if n mod 2 = 0 then ConsF (n, n / 2)
  else ConsF (n, 3 * n + 1)

let collatz n = ana collatz_coalg n

(* Convert fix_list to OCaml list *)
let to_list fl = cata (function NilF -> [] | ConsF (x, acc) -> x :: acc) fl

(* Tree anamorphism *)
type 'a tree_f = LeafF of int | BranchF of 'a * 'a

let map_tf f = function LeafF n -> LeafF n | BranchF (l, r) -> BranchF (f l, f r)

type fix_tree = FixT of fix_tree tree_f

let rec ana_tree coalg seed =
  FixT (map_tf (ana_tree coalg) (coalg seed))

let rec cata_tree alg (FixT f) = alg (map_tf (cata_tree alg) f)

(* Build a balanced binary tree of depth d *)
let balanced_coalg (d, start) =
  if d <= 0 then LeafF start
  else BranchF ((d - 1, start), (d - 1, start + (1 lsl (d - 1))))

let balanced_tree d = ana_tree balanced_coalg (d, 1)

let tree_to_list t = cata_tree (function LeafF n -> [n] | BranchF (l, r) -> l @ r) t

(* === Tests === *)
let () =
  assert (to_list (range 1 5) = [1; 2; 3; 4; 5]);
  assert (to_list (range 3 3) = [3]);
  assert (to_list (range 5 3) = []);

  assert (to_list (countdown 5) = [5; 4; 3; 2; 1]);
  assert (to_list (countdown 0) = []);

  let c = to_list (collatz 6) in
  assert (c = [6; 3; 10; 5; 16; 8; 4; 2; 1]);

  let t = balanced_tree 2 in
  assert (tree_to_list t = [1; 2; 3; 4]);

  let t3 = balanced_tree 3 in
  assert (List.length (tree_to_list t3) = 8);

  print_endline "✓ All tests passed"
