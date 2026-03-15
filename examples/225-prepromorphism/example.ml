(* Example 225: Prepromorphism — Apply Natural Transformation at Each Step of Cata *)

(* prepro : ('f -> 'f) -> ('f 'a -> 'a) -> fix -> 'a
   Like cata, but applies a natural transformation to each layer BEFORE
   recursing into children. Transforms the structure as it folds. *)

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
let unfix (Fix f) = f

let rec cata alg (Fix f) = alg (map_f (cata alg) f)

(* prepro: apply nat_transform to each layer before recurring *)
let rec prepro (nat : fix expr_f -> fix expr_f) (alg : 'a expr_f -> 'a) (Fix f : fix) : 'a =
  alg (map_f (fun child ->
    (* Transform the child's layer, then recurse *)
    prepro nat alg (Fix (nat (unfix child)))
  ) f)

(* Approach 1: Replace all Mul with Add *)
(* Natural transformation: Mul(a,b) → Add(a,b) *)
let mul_to_add : fix expr_f -> fix expr_f = function
  | MulF (a, b) -> AddF (a, b)
  | other -> other

let eval_alg = function
  | LitF n -> n
  | AddF (a, b) -> a + b
  | MulF (a, b) -> a * b
  | NegF a -> -a

(* eval with mul→add: all multiplications become additions *)
let eval_add_only = prepro mul_to_add eval_alg

(* Approach 2: Double all literals *)
let double_lits : fix expr_f -> fix expr_f = function
  | LitF n -> LitF (n * 2)
  | other -> other

(* At each level, literals get doubled before the algebra sees them.
   Inner literals get doubled more times! *)
let eval_doubling = prepro double_lits eval_alg

(* Approach 3: Remove negations *)
let remove_neg : fix expr_f -> fix expr_f = function
  | NegF a -> unfix a  (* unwrap: skip the negation *)
  | other -> other

let eval_no_neg = prepro remove_neg eval_alg

(* Approach: Identity transform = plain cata *)
let identity_nat x = x
let eval_plain = prepro identity_nat eval_alg

(* Builders *)
let lit n = Fix (LitF n)
let add a b = Fix (AddF (a, b))
let mul a b = Fix (MulF (a, b))
let neg a = Fix (NegF a)

(* === Tests === *)
let () =
  (* Normal eval *)
  let e = mul (add (lit 2) (lit 3)) (lit 4) in
  assert (cata eval_alg e = 20);

  (* prepro with identity = cata *)
  assert (eval_plain e = 20);

  (* Mul→Add: (2+3)*(4) becomes (2+3)+(4) = 9? No!
     prepro transforms at EACH level going down:
     - Top level: Mul(Add(2,3), 4) → Add(Add(2,3), 4)
     - Children processed with same transform
     Result: (2+3) + 4 = 9 *)
  assert (eval_add_only e = 9);

  (* Simple case: mul(2,3) → add(2,3) = 5 *)
  let e2 = mul (lit 2) (lit 3) in
  assert (eval_add_only e2 = 5);

  (* Double literals: deeper literals get doubled more *)
  let e3 = add (lit 1) (lit 2) in
  (* Level 0: Add(Lit 1, Lit 2)
     Children: Lit 1 → doubled to Lit 2, Lit 2 → doubled to Lit 4
     Result: 2 + 4 = 6 *)
  assert (eval_doubling e3 = 6);

  (* Remove negation *)
  let e4 = add (neg (lit 5)) (lit 3) in
  (* neg gets removed, so it's add(5, 3) = 8 *)
  assert (eval_no_neg e4 = 8);

  let e5 = neg (neg (lit 10)) in
  (* First neg removed → neg(10), that neg also removed → 10 *)
  assert (eval_no_neg e5 = 10);

  print_endline "✓ All tests passed"
