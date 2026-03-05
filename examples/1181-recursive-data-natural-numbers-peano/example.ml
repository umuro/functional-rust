(* Recursive Data — Natural Numbers (Peano) *)
(* Peano natural numbers as a recursive type *)

type nat = Zero | Succ of nat

let rec to_int = function
  | Zero -> 0
  | Succ n -> 1 + to_int n

let rec of_int = function
  | 0 -> Zero
  | n -> Succ (of_int (n - 1))

let rec add a b = match a with
  | Zero -> b
  | Succ a' -> Succ (add a' b)

let rec mul a b = match a with
  | Zero -> Zero
  | Succ a' -> add b (mul a' b)

let three = of_int 3
let four = of_int 4
let () = Printf.printf "3 + 4 = %d\n" (to_int (add three four))
let () = Printf.printf "3 * 4 = %d\n" (to_int (mul three four))
