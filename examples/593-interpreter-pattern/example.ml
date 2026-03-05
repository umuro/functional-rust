(* Mini-language interpreter in OCaml *)
type expr =
  | Lit   of float
  | Var   of string
  | Add   of expr * expr
  | Mul   of expr * expr
  | Let   of string * expr * expr
  | If    of expr * expr * expr

type env = (string * float) list

let rec eval env = function
  | Lit n          -> n
  | Var x          -> List.assoc x env
  | Add(l,r)       -> eval env l +. eval env r
  | Mul(l,r)       -> eval env l *. eval env r
  | Let(x,e,body)  -> let v = eval env e in eval ((x,v)::env) body
  | If(cond,t,f)   -> if eval env cond <> 0.0 then eval env t else eval env f

let () =
  (* let x = 3 in let y = 4 in x*x + y*y *)
  let e = Let("x",Lit 3., Let("y",Lit 4.,
              Add(Mul(Var"x",Var"x"), Mul(Var"y",Var"y")))) in
  Printf.printf "result = %.1f\n" (eval [] e)
