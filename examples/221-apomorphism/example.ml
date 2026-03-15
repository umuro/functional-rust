(* Example 221: Apomorphism — Ana that Can Short-Circuit *)

(* apo : ('a -> 'f (Either fix 'a)) -> 'a -> fix
   Like ana, but at each step you can either:
   - Right seed: continue unfolding
   - Left fix:   inject a pre-built subtree (short-circuit) *)

type 'a list_f = NilF | ConsF of int * 'a

let map_f f = function NilF -> NilF | ConsF (x, a) -> ConsF (x, f a)

type fix_list = FixL of fix_list list_f

let rec ana coalg seed = FixL (map_f (ana coalg) (coalg seed))

(* apo: coalgebra returns Either fix_list seed *)
let rec apo coalg seed =
  let layer = coalg seed in
  FixL (map_f (function
    | Either.Left fix -> fix        (* pre-built, stop *)
    | Either.Right s -> apo coalg s (* continue unfolding *)
  ) layer)

(* Since OCaml doesn't have Either built-in, define it *)
type ('a, 'b) either = Left of 'a | Right of 'b

let rec apo coalg seed =
  let layer = coalg seed in
  FixL (map_f (function
    | Left fix -> fix
    | Right s -> apo coalg s
  ) layer)

(* Approach 1: Insert into sorted list — short-circuit with remainder *)
let insert_coalg x = function
  | FixL NilF -> ConsF (x, Left (FixL NilF))
  | FixL (ConsF (y, rest)) as original ->
    if x <= y then ConsF (x, Left original)  (* short-circuit: keep rest as-is *)
    else ConsF (y, Right rest)                 (* continue searching *)

let insert x lst = apo (insert_coalg x) lst

(* Approach 2: Take n elements — short-circuit after n *)
let take_coalg n = function
  | _, FixL NilF -> NilF
  | 0, _ -> NilF
  | n, FixL (ConsF (x, rest)) ->
    ConsF (x, Right (n - 1, rest))

let take n lst = apo (fun (n, lst) -> take_coalg n (n, lst)) (n, lst)

(* Approach 3: Replace first occurrence *)
let replace_first_coalg (target, replacement) = function
  | FixL NilF -> NilF
  | FixL (ConsF (x, rest)) ->
    if x = target then ConsF (replacement, Left rest)  (* found it, short-circuit *)
    else ConsF (x, Right rest)  (* keep looking *)

let replace_first target replacement lst =
  apo (replace_first_coalg (target, replacement)) lst

(* Helpers *)
let nil = FixL NilF
let cons x xs = FixL (ConsF (x, xs))
let rec to_list (FixL f) = match f with
  | NilF -> []
  | ConsF (x, rest) -> x :: to_list rest

(* === Tests === *)
let () =
  let sorted = cons 1 (cons 3 (cons 5 nil)) in

  (* Insert *)
  assert (to_list (insert 2 sorted) = [1; 2; 3; 5]);
  assert (to_list (insert 0 sorted) = [0; 1; 3; 5]);
  assert (to_list (insert 6 sorted) = [1; 3; 5; 6]);

  (* Take *)
  let xs = cons 1 (cons 2 (cons 3 (cons 4 (cons 5 nil)))) in
  assert (to_list (take 3 xs) = [1; 2; 3]);
  assert (to_list (take 0 xs) = []);
  assert (to_list (take 10 xs) = [1; 2; 3; 4; 5]);

  (* Replace first *)
  let xs2 = cons 1 (cons 2 (cons 3 (cons 2 nil))) in
  assert (to_list (replace_first 2 99 xs2) = [1; 99; 3; 2]);

  print_endline "✓ All tests passed"
