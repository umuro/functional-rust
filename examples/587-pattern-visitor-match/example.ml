(* Visitor via match in OCaml *)
type expr =
  | Lit    of float
  | Add    of expr * expr
  | Sub    of expr * expr
  | Mul    of expr * expr
  | Div    of expr * expr

(* Each "visitor" is just a recursive function *)
let rec eval = function
  | Lit n      -> n
  | Add(l,r)   -> eval l +. eval r
  | Sub(l,r)   -> eval l -. eval r
  | Mul(l,r)   -> eval l *. eval r
  | Div(l,r)   -> eval l /. eval r

let rec count_ops = function
  | Lit _      -> 0
  | Add(l,r)|Sub(l,r)|Mul(l,r)|Div(l,r) -> 1 + count_ops l + count_ops r

let rec pretty = function
  | Lit n      -> string_of_float n
  | Add(l,r)   -> Printf.sprintf "(%s+%s)" (pretty l) (pretty r)
  | Sub(l,r)   -> Printf.sprintf "(%s-%s)" (pretty l) (pretty r)
  | Mul(l,r)   -> Printf.sprintf "(%s*%s)" (pretty l) (pretty r)
  | Div(l,r)   -> Printf.sprintf "(%s/%s)" (pretty l) (pretty r)

let () =
  let e = Add(Mul(Lit 3., Lit 4.), Sub(Lit 10., Lit 2.)) in
  Printf.printf "%s = %.1f (ops=%d)\n" (pretty e) (eval e) (count_ops e)
