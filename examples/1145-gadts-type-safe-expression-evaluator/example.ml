(* GADTs — Type-Safe Expression Evaluator *)
(* Generalized algebraic data types for type safety *)

type _ expr =
  | Int : int -> int expr
  | Bool : bool -> bool expr
  | Add : int expr * int expr -> int expr
  | If : bool expr * 'a expr * 'a expr -> 'a expr
  | Eq : int expr * int expr -> bool expr

let rec eval : type a. a expr -> a = function
  | Int n -> n
  | Bool b -> b
  | Add (a, b) -> eval a + eval b
  | If (cond, t, f) -> if eval cond then eval t else eval f
  | Eq (a, b) -> eval a = eval b

let result = eval (If (Eq (Add (Int 2, Int 3), Int 5), Int 1, Int 0))
let () = Printf.printf "Result: %d\n" result
