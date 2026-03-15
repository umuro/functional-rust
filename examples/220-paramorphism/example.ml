(* Example 220: Paramorphism — Cata with Access to Original Subtree *)

(* para : ('f ('a * fix) -> 'a) -> fix -> 'a
   Like cata, but the algebra also sees the original subtree (not just the result). *)

type 'a list_f = NilF | ConsF of int * 'a

let map_f f = function NilF -> NilF | ConsF (x, a) -> ConsF (x, f a)

type fix_list = FixL of fix_list list_f
let unfix (FixL f) = f

let rec cata alg (FixL f) = alg (map_f (cata alg) f)

(* para: each position gets (result, original_subtree) *)
let rec para alg (FixL f as original) =
  let paired = map_f (fun child -> (para alg child, child)) f in
  alg paired

(* Approach 1: Factorial — para sees (n-1)! AND the original list from n-1 down *)
let nil = FixL NilF
let cons x xs = FixL (ConsF (x, xs))

let to_list fl = cata (function NilF -> [] | ConsF (x, acc) -> x :: acc) fl

(* Approach 2: tails — needs the original subtree *)
(* tails [1;2;3] = [[1;2;3]; [2;3]; [3]; []] *)
let tails_alg = function
  | NilF -> [[]]
  | ConsF (_, (rest_tails, original_tail)) ->
    to_list original_tail :: rest_tails
(* We need original_tail (the fix structure) to convert it to a list *)

let tails fl = (to_list fl) :: para tails_alg fl

(* Approach 3: Sliding window — needs access to remainder *)
let sliding_window_alg n = function
  | NilF -> []
  | ConsF (x, (rest_windows, original_tail)) ->
    let remainder = x :: to_list original_tail in
    if List.length remainder >= n then
      (List.filteri (fun i _ -> i < n) remainder) :: rest_windows
    else
      rest_windows

let sliding_window n fl = para (sliding_window_alg n) fl

(* Approach 4: Drop while — needs to know "am I still dropping?" *)
(* Actually simpler: suffix extraction *)
let drop_while_alg pred = function
  | NilF -> []
  | ConsF (x, (rest, original_tail)) ->
    if pred x then rest
    else x :: to_list original_tail

let drop_while pred fl = para (drop_while_alg pred) fl

(* === Tests === *)
let () =
  let xs = cons 1 (cons 2 (cons 3 nil)) in

  (* tails *)
  let t = tails xs in
  assert (t = [[1; 2; 3]; [2; 3]; [3]; []]);

  (* sliding window of size 2 *)
  let w = sliding_window 2 xs in
  assert (w = [[1; 2]; [2; 3]]);

  (* sliding window of size 3 *)
  let w3 = sliding_window 3 xs in
  assert (w3 = [[1; 2; 3]]);

  (* drop_while *)
  let xs2 = cons 1 (cons 2 (cons 3 (cons 1 nil))) in
  let d = drop_while (fun x -> x < 3) xs2 in
  assert (d = [3; 1]);

  let d2 = drop_while (fun _ -> false) xs in
  assert (d2 = [1; 2; 3]);

  print_endline "✓ All tests passed"
