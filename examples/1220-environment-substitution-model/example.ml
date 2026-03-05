(* Environment — Substitution Model *)
(* Simple interpreter with environment-based evaluation *)

type expr =
  | Var of string | Num of int
  | Let of string * expr * expr
  | Fun of string * expr | App of expr * expr
  | Add of expr * expr

type value = VNum of int | VFun of string * expr * env
and env = (string * value) list

let rec eval env = function
  | Num n -> VNum n
  | Var x -> List.assoc x env
  | Add (a, b) ->
    (match (eval env a, eval env b) with
     | (VNum a, VNum b) -> VNum (a + b) | _ -> failwith "type error")
  | Let (x, e1, e2) -> eval ((x, eval env e1) :: env) e2
  | Fun (x, body) -> VFun (x, body, env)
  | App (f, arg) ->
    (match eval env f with
     | VFun (x, body, cenv) -> eval ((x, eval env arg) :: cenv) body
     | _ -> failwith "not a function")

let prog = Let ("double", Fun ("x", Add (Var "x", Var "x")),
                App (Var "double", Num 21))
let () = match eval [] prog with
  | VNum n -> Printf.printf "Result: %d\n" n | _ -> ()
