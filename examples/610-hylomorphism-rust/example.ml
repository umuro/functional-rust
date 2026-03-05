(* Hylomorphism in OCaml *)

(* Factorial as hylo *)
(* Coalgebra: unfold 5 -> [5;4;3;2;1] *)
(* Algebra: fold [5;4;3;2;1] -> 120 *)
let factorial n =
  let unfold_step k = if k <= 1 then None else Some (k, k-1) in
  let seeds = let rec go k = if k<=1 then [1] else k :: go (k-1) in go n in
  List.fold_left ( * ) 1 seeds

(* Merge sort as hylomorphism *)
let rec merge xs ys = match (xs, ys) with
  | ([], _) -> ys | (_, []) -> xs
  | (x::xt, y::yt) -> if x<=y then x::merge xt ys else y::merge xs yt

let split xs =
  let rec go a b = function
    | [] -> (a, b)
    | x :: rest -> go b (x::a) rest
  in go [] [] xs

let merge_sort xs =
  let rec hylo = function
    | [] | [_] as xs -> xs
    | xs -> let (a,b) = split xs in merge (hylo a) (hylo b)
  in hylo xs

let () =
  Printf.printf "5! = %d\n" (factorial 5);
  Printf.printf "sorted: %s\n"
    (String.concat "," (List.map string_of_int (merge_sort [3;1;4;1;5;9;2;6])))
