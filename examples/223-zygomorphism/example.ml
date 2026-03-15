(* Example 223: Zygomorphism — Two Mutually Dependent Folds *)

(* zygo : ('f 'b -> 'b) -> ('f ('a, 'b) -> 'a) -> fix -> 'a
   Run two folds simultaneously: the main fold sees both results,
   the helper fold runs independently. *)

type 'a expr_f =
  | LitF of int
  | AddF of 'a * 'a
  | MulF of 'a * 'a
  | NegF of 'a

let map_f f = function
  | LitF n -> LitF n
  | AddF (a, b) -> AddF (f a, f b)
  | MulF (a, b) -> MulF (f a, f b)
  | NegF a -> NegF (f a)

type fix = Fix of fix expr_f

let rec cata alg (Fix f) = alg (map_f (cata alg) f)

(* zygo: helper fold computes 'b, main fold sees both 'a and 'b *)
let rec zygo helper_alg main_alg (Fix f) =
  let paired = map_f (fun child ->
    let a = zygo helper_alg main_alg child in
    let b = cata helper_alg child in
    (a, b)
  ) f in
  main_alg paired

(* More efficient: compute both in one pass *)
let rec zygo_eff helper main (Fix f) =
  let paired = map_f (fun child ->
    let (a, b) = zygo_both helper main child in
    (a, b)
  ) f in
  main paired
and zygo_both helper main (Fix f) =
  let paired = map_f (fun child -> zygo_both helper main child) f in
  let b_layer = map_f snd paired in
  let ab_layer = paired in
  (main ab_layer, helper b_layer)

(* Approach 1: "Is this expression safe?" depends on evaluation *)
(* Helper: evaluate. Main: check if safe (no division by zero, etc.) *)
let eval_helper = function
  | LitF n -> n
  | AddF (a, b) -> a + b
  | MulF (a, b) -> a * b
  | NegF a -> -a

(* "Safe" means no multiplication by a value that could overflow *)
let safe_main = function
  | LitF _ -> true
  | AddF ((a, _), (b, _)) -> a && b
  | MulF ((a, va), (b, vb)) -> a && b && abs va < 1000 && abs vb < 1000
  | NegF (a, _) -> a

(* Approach 2: Pretty print with precedence *)
(* Helper: compute precedence. Main: add parens only when needed. *)
let prec_helper = function
  | LitF _ -> 100
  | AddF _ -> 1
  | MulF _ -> 2
  | NegF _ -> 3

let show_main = function
  | LitF n -> string_of_int n
  | AddF ((a, pa), (b, pb)) ->
    let la = if pa < 1 then "(" ^ a ^ ")" else a in
    let rb = if pb < 1 then "(" ^ b ^ ")" else b in
    la ^ " + " ^ rb
  | MulF ((a, pa), (b, pb)) ->
    let la = if pa < 2 then "(" ^ a ^ ")" else a in
    let rb = if pb < 2 then "(" ^ b ^ ")" else b in
    la ^ " * " ^ rb
  | NegF (a, _) -> "-" ^ a

(* Approach 3: Count and sum simultaneously *)
let count_helper = function
  | LitF _ -> 1
  | AddF (a, b) | MulF (a, b) -> a + b
  | NegF a -> a

let avg_main = function
  | LitF n -> float_of_int n
  | AddF ((a, ca), (b, cb)) -> (a *. float_of_int ca +. b *. float_of_int cb) /. float_of_int (ca + cb)
  | MulF ((_, _), (_, _)) -> 0.0 (* simplified *)
  | NegF (a, _) -> -. a

(* Builders *)
let lit n = Fix (LitF n)
let add a b = Fix (AddF (a, b))
let mul a b = Fix (MulF (a, b))
let neg a = Fix (NegF a)

(* === Tests === *)
let () =
  let e = add (lit 3) (mul (lit 4) (lit 5)) in

  (* Safety check *)
  let safe = zygo_eff eval_helper safe_main e in
  assert safe;

  let unsafe_e = mul (lit 99999) (lit 99999) in
  let not_safe = zygo_eff eval_helper safe_main unsafe_e in
  assert (not not_safe);

  (* Show with precedence *)
  let e2 = mul (add (lit 1) (lit 2)) (lit 3) in
  let shown = zygo_eff prec_helper show_main e2 in
  assert (shown = "(1 + 2) * 3");

  let e3 = add (lit 1) (mul (lit 2) (lit 3)) in
  let shown3 = zygo_eff prec_helper show_main e3 in
  assert (shown3 = "1 + 2 * 3");

  print_endline "✓ All tests passed"
