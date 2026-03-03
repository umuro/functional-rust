(* Interpreter — Simple Lambda Calculus *)

type expr =
  | Int of int | Var of string
  | Lam of string * expr | App of expr * expr
  | Add of expr * expr

type value = VInt of int | VClosure of string * expr * env
and env = (string * value) list

(* Version 1: Direct recursive interpreter *)
let rec eval env = function
  | Int n -> VInt n
  | Var x -> List.assoc x env
  | Lam (x, body) -> VClosure (x, body, env)
  | App (f, arg) ->
    let fv = eval env f in
    let av = eval env arg in
    (match fv with
     | VClosure (x, body, cenv) -> eval ((x, av) :: cenv) body
     | _ -> failwith "not a function")
  | Add (a, b) ->
    (match eval env a, eval env b with
     | VInt x, VInt y -> VInt (x + y)
     | _ -> failwith "type error")

let () =
  let e = App (Lam ("x", Add (Var "x", Int 1)), Int 41) in
  match eval [] e with VInt n -> assert (n = 42) | _ -> assert false
