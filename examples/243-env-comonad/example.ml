(* Env comonad (also called CoReader): pairs a value with a read-only environment.
   Dual of the Reader monad.
   extract: get the value (ignoring environment)
   extend: access the environment while computing *)

type ('e, 'a) env = Env of 'e * 'a

let ask     (Env (e, _)) = e
let extract (Env (_, a)) = a

let extend (Env (e, a)) f =
  Env (e, f (Env (e, a)))

(* A simple expression evaluator with environment *)
type expr =
  | Lit of int
  | Var of string
  | Add of expr * expr
  | Mul of expr * expr

type env_map = (string * int) list

let rec eval_expr (Env (env, expr)) =
  match expr with
  | Lit n        -> n
  | Var x        -> List.assoc x env
  | Add (l, r)   ->
    eval_expr (Env (env, l)) + eval_expr (Env (env, r))
  | Mul (l, r)   ->
    eval_expr (Env (env, l)) * eval_expr (Env (env, r))

let () =
  let env = [("x", 10); ("y", 3); ("z", 7)] in

  let e1 = Add (Var "x", Mul (Lit 2, Var "y")) in (* x + 2*y = 16 *)
  let result1 = eval_expr (Env (env, e1)) in
  Printf.printf "x + 2*y = %d\n" result1;

  let e2 = Mul (Add (Var "x", Var "z"), Var "y") in (* (x+z)*y = 51 *)
  let result2 = eval_expr (Env (env, e2)) in
  Printf.printf "(x+z)*y = %d\n" result2;

  (* Extend: derive new computation from environment *)
  let w = Env (env, "x") in
  let w' = extend w (fun (Env (e, k)) -> List.assoc k e * 2) in
  Printf.printf "x * 2 via extend = %d\n" (extract w')
