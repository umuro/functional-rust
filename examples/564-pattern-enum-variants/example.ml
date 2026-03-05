(* Algebraic expression eval in OCaml *)
type expr =
  | Num of float
  | Add of expr * expr
  | Mul of expr * expr
  | Neg of expr
  | Var of string

let rec eval env = function
  | Num n         -> n
  | Add (l, r)    -> eval env l +. eval env r
  | Mul (l, r)    -> eval env l *. eval env r
  | Neg e         -> -. (eval env e)
  | Var s         -> List.assoc s env

let rec show = function
  | Num n      -> string_of_float n
  | Add (l, r) -> Printf.sprintf "(%s+%s)" (show l) (show r)
  | Mul (l, r) -> Printf.sprintf "(%s*%s)" (show l) (show r)
  | Neg e      -> Printf.sprintf "(-%s)" (show e)
  | Var s      -> s

let () =
  let env = [("x",3.0);("y",4.0)] in
  let e = Add(Mul(Var"x",Var"x"), Mul(Var"y",Var"y")) in
  Printf.printf "%s = %.1f\n" (show e) (eval env e)
