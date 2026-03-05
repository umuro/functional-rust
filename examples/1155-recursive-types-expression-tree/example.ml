(* Recursive Types — Expression Tree *)
(* Define and evaluate a recursive expression type *)

type expr =
  | Lit of float
  | Var of string
  | Add of expr * expr
  | Mul of expr * expr
  | Neg of expr

let rec eval env = function
  | Lit n -> n
  | Var x -> List.assoc x env
  | Add (a, b) -> eval env a +. eval env b
  | Mul (a, b) -> eval env a *. eval env b
  | Neg e -> -.(eval env e)

let rec to_string = function
  | Lit n -> Printf.sprintf "%.0f" n
  | Var x -> x
  | Add (a, b) -> Printf.sprintf "(%s + %s)" (to_string a) (to_string b)
  | Mul (a, b) -> Printf.sprintf "(%s * %s)" (to_string a) (to_string b)
  | Neg e -> Printf.sprintf "(-%s)" (to_string e)

let e = Add (Mul (Var "x", Lit 2.0), Lit 3.0)
let () = Printf.printf "%s = %.0f\n" (to_string e) (eval [("x", 5.0)] e)
