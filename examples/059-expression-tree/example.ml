(* Recursive Variant — Expression Tree *)

type expr =
  | Num of float
  | Add of expr * expr
  | Sub of expr * expr
  | Mul of expr * expr
  | Div of expr * expr

(* Implementation 1: Direct recursive eval *)
let rec eval = function
  | Num n         -> n
  | Add (l, r)    -> eval l +. eval r
  | Sub (l, r)    -> eval l -. eval r
  | Mul (l, r)    -> eval l *. eval r
  | Div (l, r)    -> eval l /. eval r

(* Implementation 2: to_string with parentheses *)
let rec to_string = function
  | Num n         -> string_of_float n
  | Add (l, r)    -> Printf.sprintf "(%s + %s)" (to_string l) (to_string r)
  | Sub (l, r)    -> Printf.sprintf "(%s - %s)" (to_string l) (to_string r)
  | Mul (l, r)    -> Printf.sprintf "(%s * %s)" (to_string l) (to_string r)
  | Div (l, r)    -> Printf.sprintf "(%s / %s)" (to_string l) (to_string r)

(* Tests *)
let () =
  (* (1 + 2) * (10 - 4) = 18 *)
  let e = Mul (Add (Num 1., Num 2.), Sub (Num 10., Num 4.)) in
  assert (eval e = 18.);
  assert (eval (Num 42.) = 42.);
  assert (eval (Div (Num 10., Sub (Num 5., Num 3.))) = 5.);
  Printf.printf "%s = %.1f\n" (to_string e) (eval e);
  Printf.printf "All expression-tree tests passed!\n"
