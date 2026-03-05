(* Let Bindings and Scope *)
(* Understanding let expressions and lexical scope *)

(* let..in creates a local scope *)
let area_of_ring ~inner ~outer =
  let pi = Float.pi in
  let sq r = r *. r in
  pi *. (sq outer -. sq inner)

(* Shadowing, not mutation *)
let x = 5
let x = x + 1  (* new binding, old x is shadowed *)
let () = Printf.printf "x = %d\n" x  (* 6 *)

(* Nested let..in *)
let hypotenuse a b =
  let a2 = a *. a in
  let b2 = b *. b in
  sqrt (a2 +. b2)

let () = Printf.printf "Ring area: %.2f\n" (area_of_ring ~inner:3.0 ~outer:5.0)
let () = Printf.printf "Hypotenuse: %.2f\n" (hypotenuse 3.0 4.0)
