(* Cartesian Closed Categories in OCaml *)

(* Terminal object *)
type terminal = unit

(* Product *)
type ('a, 'b) product = 'a * 'b

(* Exponential *)
type ('a, 'b) exponential = 'a -> 'b

(* curry *)
let curry f a b = f (a, b)

(* uncurry *)
let uncurry f (a, b) = f a b

(* eval: (B^A × A) → B *)
let eval (f, a) = f a

(* Product universal property *)
let product_universal f g x = (f x, g x)

(* Diagonal *)
let diagonal x = (x, x)

let () =
  (* Curry example *)
  let add (a, b) = a + b in
  let curried = curry add in
  let add_5 = curried 5 in
  Printf.printf "5 + 3 = %d\n" (add_5 3);
  
  (* Universal property *)
  let h = product_universal (fun x -> x + 1) (fun x -> x * 2) in
  let (a, b) = h 5 in
  Printf.printf "product_universal: (%d, %d)\n" a b
