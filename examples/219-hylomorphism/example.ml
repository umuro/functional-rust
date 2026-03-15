(* Example 219: Hylomorphism — Ana then Cata, Fused *)

(* hylo : ('f b -> b) -> (a -> 'f a) -> a -> b
   Unfold a seed, then fold the result. No intermediate structure built! *)

type 'a list_f = NilF | ConsF of int * 'a

let map_f f = function NilF -> NilF | ConsF (x, a) -> ConsF (x, f a)

(* The general hylo: ana then cata, fused into one pass *)
let rec hylo alg coalg seed =
  alg (map_f (hylo alg coalg) (coalg seed))

(* Approach 1: Factorial via hylo *)
(* Coalgebra: n -> ConsF(n, n-1) or NilF when n=0
   Algebra:   NilF -> 1, ConsF(n, acc) -> n * acc *)
let fact_coalg n =
  if n <= 0 then NilF
  else ConsF (n, n - 1)

let fact_alg = function
  | NilF -> 1
  | ConsF (n, acc) -> n * acc

let factorial n = hylo fact_alg fact_coalg n

(* Approach 2: Sum of range [1..n] via hylo *)
let range_coalg n =
  if n <= 0 then NilF
  else ConsF (n, n - 1)

let sum_alg = function
  | NilF -> 0
  | ConsF (x, acc) -> x + acc

let sum_range n = hylo sum_alg range_coalg n

(* Approach 3: Merge sort via tree hylo *)
type 'a tree_f = LeafF of int | BranchF of 'a * 'a

let map_tf f = function
  | LeafF n -> LeafF n
  | BranchF (l, r) -> BranchF (f l, f r)

let rec hylo_tree alg coalg seed =
  alg (map_tf (hylo_tree alg coalg) (coalg seed))

(* Split a list into halves *)
let split_coalg = function
  | [] -> LeafF 0  (* shouldn't happen *)
  | [x] -> LeafF x
  | xs ->
    let mid = List.length xs / 2 in
    let rec take n = function
      | [] -> []
      | x :: rest -> if n <= 0 then [] else x :: take (n-1) rest
    in
    let rec drop n = function
      | [] -> []
      | _ :: rest as xs -> if n <= 0 then xs else drop (n-1) rest
    in
    BranchF (take mid xs, drop mid xs)

(* Merge two sorted lists *)
let rec merge xs ys = match xs, ys with
  | [], ys -> ys
  | xs, [] -> xs
  | (x :: xt), (y :: yt) ->
    if x <= y then x :: merge xt ys
    else y :: merge xs yt

let merge_alg = function
  | LeafF n -> [n]
  | BranchF (l, r) -> merge l r

let merge_sort xs = hylo_tree merge_alg split_coalg xs

(* === Tests === *)
let () =
  assert (factorial 0 = 1);
  assert (factorial 1 = 1);
  assert (factorial 5 = 120);
  assert (factorial 10 = 3628800);

  assert (sum_range 0 = 0);
  assert (sum_range 10 = 55);
  assert (sum_range 100 = 5050);

  assert (merge_sort [] = []);
  assert (merge_sort [1] = [1]);
  assert (merge_sort [3; 1; 4; 1; 5; 9; 2; 6] = [1; 1; 2; 3; 4; 5; 6; 9]);
  assert (merge_sort [5; 4; 3; 2; 1] = [1; 2; 3; 4; 5]);

  print_endline "✓ All tests passed"
