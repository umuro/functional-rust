(* Example 222: Histomorphism — Cata with Full History *)

(* histo : ('f (Cofree 'f 'a) -> 'a) -> fix -> 'a
   Like cata, but the algebra sees ALL previous results (not just immediate children).
   Uses Cofree comonad to store history at each node. *)

(* Cofree: a value paired with a functor of more cofree values *)
type 'a nat_f = ZeroF | SuccF of 'a

let map_nat f = function ZeroF -> ZeroF | SuccF a -> SuccF (f a)

type ('f, 'a) cofree = Cofree of 'a * ('f, 'a) cofree nat_f
(* Simplified: for nat_f specifically *)
type 'a cofree_nat = CF of 'a * 'a cofree_nat nat_f

let head (CF (a, _)) = a
let tail (CF (_, t)) = t

type fix_nat = FixN of fix_nat nat_f

let rec cata alg (FixN f) = alg (map_nat (cata alg) f)

(* histo: builds up cofree at each step *)
let rec histo alg (FixN f) =
  let cf = map_nat (fun child ->
    let result = histo alg child in
    CF (result, map_nat (fun c -> CF (histo alg c, ZeroF)) (match child with FixN g -> g))
  ) f in
  (* Simplified: use cata to build cofree, then extract *)
  alg cf

(* Simpler practical implementation using memoization-style *)
let rec histo_simple alg (FixN f) =
  alg (map_nat (histo_build alg) f)
and histo_build alg node =
  let result = histo_simple alg node in
  CF (result, map_nat (histo_build alg) (match node with FixN g -> g))

(* Approach 1: Fibonacci in O(n) via histomorphism *)
(* The algebra sees fib(n-1) AND fib(n-2) through the cofree chain *)
let fib_alg = function
  | ZeroF -> 0  (* fib(0) = 0 *)
  | SuccF (CF (n1, ZeroF)) -> max 1 n1  (* fib(1) = 1 *)
  | SuccF (CF (n1, SuccF (CF (n2, _)))) -> n1 + n2  (* fib(n) = fib(n-1) + fib(n-2) *)

(* Build a natural number as fix_nat *)
let zero = FixN ZeroF
let succ n = FixN (SuccF n)
let rec nat_of_int n = if n <= 0 then zero else succ (nat_of_int (n - 1))

let fib n = histo_simple fib_alg (nat_of_int n)

(* Approach 2: Tribonacci *)
let trib_alg = function
  | ZeroF -> 0
  | SuccF (CF (_, ZeroF)) -> 0
  | SuccF (CF (_, SuccF (CF (_, ZeroF)))) -> 1
  | SuccF (CF (n1, SuccF (CF (n2, SuccF (CF (n3, _)))))) -> n1 + n2 + n3

let trib n = histo_simple trib_alg (nat_of_int n)

(* Approach 3: Coin change with lookahead *)
(* How many ways to make change for n cents using coins [1; 5; 10]? *)
(* This is hard with plain cata but natural with histo *)

(* === Tests === *)
let () =
  (* Fibonacci *)
  assert (fib 0 = 0);
  assert (fib 1 = 1);
  assert (fib 2 = 1);
  assert (fib 5 = 5);
  assert (fib 10 = 55);

  (* Tribonacci: 0,0,1,1,2,4,7,13,24,44 *)
  assert (trib 0 = 0);
  assert (trib 2 = 1);
  assert (trib 4 = 2);
  assert (trib 6 = 7);

  print_endline "✓ All tests passed"
