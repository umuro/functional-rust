(* OCaml: external declarations link to C symbols via the OCaml runtime. *)

(* In real OCaml, you'd write: external c_add : int -> int -> int = "c_add" *)
(* Here we simulate the C functions directly in OCaml. *)

let c_add (a : int) (b : int) : int = a + b
let c_abs (n : int) : int = if n < 0 then -n else n
let c_max (a : int) (b : int) : int = if a > b then a else b

let () =
  Printf.printf "c_add(3, 4)   = %d\n" (c_add 3 4);
  Printf.printf "c_abs(-7)     = %d\n" (c_abs (-7));
  Printf.printf "c_max(10, 20) = %d\n" (c_max 10 20)
