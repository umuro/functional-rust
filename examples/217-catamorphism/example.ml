(* Example 217: Catamorphism — Fold ANY Recursive Structure *)

(* cata : ('f 'a -> 'a) -> fix -> 'a
   "Give me what to do at each layer, I'll handle the recursion." *)

type 'a expr_f =
  | LitF of int
  | AddF of 'a * 'a
  | MulF of 'a * 'a
  | NegF of 'a
  | IfZeroF of 'a * 'a * 'a  (* condition, then, else *)

let map_f f = function
  | LitF n -> LitF n
  | AddF (a, b) -> AddF (f a, f b)
  | MulF (a, b) -> MulF (f a, f b)
  | NegF a -> NegF (f a)
  | IfZeroF (c, t, e) -> IfZeroF (f c, f t, f e)

type fix = Fix of fix expr_f
let unfix (Fix f) = f

let rec cata alg (Fix f) = alg (map_f (cata alg) f)

(* Approach 1: Multiple algebras — each just one layer *)
let eval_alg = function
  | LitF n -> n
  | AddF (a, b) -> a + b
  | MulF (a, b) -> a * b
  | NegF a -> -a
  | IfZeroF (c, t, e) -> if c = 0 then t else e

let eval = cata eval_alg

let show_alg = function
  | LitF n -> string_of_int n
  | AddF (a, b) -> "(" ^ a ^ " + " ^ b ^ ")"
  | MulF (a, b) -> "(" ^ a ^ " * " ^ b ^ ")"
  | NegF a -> "(-" ^ a ^ ")"
  | IfZeroF (c, t, e) -> "(if0 " ^ c ^ " then " ^ t ^ " else " ^ e ^ ")"

let show = cata show_alg

(* Approach 2: Constant propagation — transform structure *)
let opt_alg = function
  | AddF (Fix (LitF 0), b) -> b
  | AddF (a, Fix (LitF 0)) -> a
  | MulF (Fix (LitF 0), _) | MulF (_, Fix (LitF 0)) -> Fix (LitF 0)
  | MulF (Fix (LitF 1), b) -> b
  | MulF (a, Fix (LitF 1)) -> a
  | NegF (Fix (NegF a)) -> a
  | other -> Fix other

let optimize = cata opt_alg

(* Approach 3: Collecting free variables (with named vars) *)
type 'a expr_v =
  | VLitF of int
  | VVarF of string
  | VAddF of 'a * 'a

let map_v f = function
  | VLitF n -> VLitF n
  | VVarF s -> VVarF s
  | VAddF (a, b) -> VAddF (f a, f b)

type fix_v = FixV of fix_v expr_v

let rec cata_v alg (FixV f) = alg (map_v (cata_v alg) f)

let free_vars_alg = function
  | VLitF _ -> []
  | VVarF s -> [s]
  | VAddF (a, b) -> a @ b

let free_vars = cata_v free_vars_alg

(* Builders *)
let lit n = Fix (LitF n)
let add a b = Fix (AddF (a, b))
let mul a b = Fix (MulF (a, b))
let neg a = Fix (NegF a)
let ifzero c t e = Fix (IfZeroF (c, t, e))

let vlit n = FixV (VLitF n)
let vvar s = FixV (VVarF s)
let vadd a b = FixV (VAddF (a, b))

(* === Tests === *)
let () =
  let e = add (lit 1) (mul (lit 2) (neg (lit 3))) in
  assert (eval e = -5);
  assert (show e = "(1 + (2 * (-3)))");

  let e2 = ifzero (lit 0) (lit 42) (lit 99) in
  assert (eval e2 = 42);

  let e3 = ifzero (lit 1) (lit 42) (lit 99) in
  assert (eval e3 = 99);

  (* Optimization *)
  let e4 = add (lit 0) (mul (lit 1) (lit 5)) in
  let opt = optimize e4 in
  assert (eval opt = 5);

  let e5 = neg (neg (lit 42)) in
  let opt5 = optimize e5 in
  assert (eval opt5 = 42);

  (* Free variables *)
  let ve = vadd (vvar "x") (vadd (vlit 1) (vvar "y")) in
  assert (free_vars ve = ["x"; "y"]);

  print_endline "✓ All tests passed"
