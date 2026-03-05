(* Recursive Types — Symbolic Differentiation *)
(* Symbolic math with algebraic data types *)

type expr = X | Const of float | Add of expr * expr
          | Mul of expr * expr | Pow of expr * float

let rec deriv = function
  | X -> Const 1.0
  | Const _ -> Const 0.0
  | Add (a, b) -> Add (deriv a, deriv b)
  | Mul (a, b) -> Add (Mul (deriv a, b), Mul (a, deriv b))
  | Pow (e, n) -> Mul (Mul (Const n, Pow (e, n -. 1.0)), deriv e)

let rec simplify = function
  | Add (Const 0.0, e) | Add (e, Const 0.0) -> simplify e
  | Mul (Const 0.0, _) | Mul (_, Const 0.0) -> Const 0.0
  | Mul (Const 1.0, e) | Mul (e, Const 1.0) -> simplify e
  | Add (a, b) -> Add (simplify a, simplify b)
  | Mul (a, b) -> Mul (simplify a, simplify b)
  | e -> e

let rec to_s = function
  | X -> "x" | Const n -> Printf.sprintf "%.0f" n
  | Add (a,b) -> Printf.sprintf "(%s + %s)" (to_s a) (to_s b)
  | Mul (a,b) -> Printf.sprintf "(%s * %s)" (to_s a) (to_s b)
  | Pow (e,n) -> Printf.sprintf "%s^%.0f" (to_s e) n

(* d/dx (x^2 + 3x) *)
let expr = Add (Pow (X, 2.0), Mul (Const 3.0, X))
let d = deriv expr |> simplify
let () = Printf.printf "d/dx %s = %s\n" (to_s expr) (to_s d)
