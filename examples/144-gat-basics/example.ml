(* GADTs (Generalized Algebraic Data Types): each constructor can
   fix the type parameter differently. The type carries proof information. *)

(* Classic: typed expression tree *)
type _ expr =
  | Int  : int  -> int expr
  | Bool : bool -> bool expr
  | Add  : int expr * int expr -> int expr
  | If   : bool expr * 'a expr * 'a expr -> 'a expr
  | Eq   : int expr * int expr -> bool expr

(* Type-safe eval: the return type matches the GADT parameter *)
let rec eval : type a. a expr -> a = function
  | Int n          -> n
  | Bool b         -> b
  | Add (l, r)     -> eval l + eval r
  | If (c, t, e)   -> if eval c then eval t else eval e
  | Eq (l, r)      -> eval l = eval r

let () =
  let e1 = Add (Int 3, Int 4) in
  Printf.printf "3 + 4 = %d\n" (eval e1);

  let e2 = If (Bool true, Int 10, Int 20) in
  Printf.printf "if true then 10 else 20 = %d\n" (eval e2);

  let e3 = If (Eq (Int 1, Int 1), Bool true, Bool false) in
  Printf.printf "if 1=1 then true else false = %b\n" (eval e3);

  let e4 = If (Eq (Add (Int 2, Int 3), Int 5), Int 42, Int 0) in
  Printf.printf "if 2+3=5 then 42 else 0 = %d\n" (eval e4)
