(* Counting elements at compile time in OCaml *)
(* OCaml uses type-level tricks for this *)

(* Phantom types for compile-time length tracking *)
type zero = Zero
type 'a succ = Succ of 'a

type ('a, 'len) vec =
  | Nil : ('a, zero) vec
  | Cons : 'a * ('a, 'n) vec -> ('a, 'n succ) vec

let vec3 = Cons (1, Cons (2, Cons (3, Nil)))
(* Type: (int, zero succ succ succ) vec *)

(* Count at "compile time" via type system *)
let rec length : type a n. (a, n) vec -> int = function
  | Nil -> 0
  | Cons (_, rest) -> 1 + length rest

(* Alternative: compile-time constant via modules *)
let n_items = 5  (* would be a compile-time constant in macro system *)
let items = Array.make n_items 0

let () =
  Printf.printf "vec3 length: %d\n" (length vec3);
  Printf.printf "items capacity: %d\n" n_items
