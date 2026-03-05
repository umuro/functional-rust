(* Example 215: Recursion Schemes — Separating What From How *)

(* === The problem: every function duplicates the same traversal === *)

type expr =
  | Lit of int
  | Add of expr * expr
  | Mul of expr * expr

let rec eval = function
  | Lit n      -> n
  | Add (a, b) -> eval a + eval b
  | Mul (a, b) -> eval a * eval b

let rec show = function
  | Lit n      -> string_of_int n
  | Add (a, b) -> "(" ^ show a ^ " + " ^ show b ^ ")"
  | Mul (a, b) -> "(" ^ show a ^ " * " ^ show b ^ ")"

let rec depth = function
  | Lit _      -> 0
  | Add (a, b) | Mul (a, b) -> 1 + max (depth a) (depth b)

(* === The solution: base functor + catamorphism === *)

(* Base functor: recursive positions become type variable 'a *)
type 'a expr_f =
  | LitF of int
  | AddF of 'a * 'a
  | MulF of 'a * 'a

(* Functorial map: apply f to every recursive position *)
let fmap f = function
  | LitF n      -> LitF n
  | AddF (a, b) -> AddF (f a, f b)
  | MulF (a, b) -> MulF (f a, f b)

(* Project: peel off one layer of expr *)
let project = function
  | Lit n      -> LitF n
  | Add (a, b) -> AddF (a, b)
  | Mul (a, b) -> MulF (a, b)

(* Catamorphism: the ONE place recursion lives *)
let rec cata alg e = alg (fmap (cata alg) (project e))

(* Algebras: pure logic, zero recursion *)
let eval_alg = function
  | LitF n      -> n
  | AddF (a, b) -> a + b
  | MulF (a, b) -> a * b

let show_alg = function
  | LitF n      -> string_of_int n
  | AddF (a, b) -> "(" ^ a ^ " + " ^ b ^ ")"
  | MulF (a, b) -> "(" ^ a ^ " * " ^ b ^ ")"

let depth_alg = function
  | LitF _      -> 0
  | AddF (a, b) | MulF (a, b) -> 1 + max a b

let count_alg = function
  | LitF _      -> 1
  | AddF (a, b) | MulF (a, b) -> 1 + a + b

(* (2 + 3) * 4 *)
let sample = Mul (Add (Lit 2, Lit 3), Lit 4)

let () =
  (* Direct approach *)
  assert (eval sample = 20);
  assert (show sample = "((2 + 3) * 4)");
  assert (depth sample = 2);
  (* Catamorphism approach — same results *)
  assert (cata eval_alg  sample = 20);
  assert (cata show_alg  sample = "((2 + 3) * 4)");
  assert (cata depth_alg sample = 2);
  assert (cata count_alg sample = 5);
  (* Edge cases *)
  assert (eval (Lit 7) = 7);
  assert (cata eval_alg (Lit 42) = 42);
  assert (cata count_alg (Lit 0) = 1);
  print_endline "ok"
