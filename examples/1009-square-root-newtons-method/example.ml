(* Square Root (Newton's Method) *)
(* Iterative approximation with recursion — Newton's method *)

let square_root n =
  let radicand = float_of_int n in
  let rec aux guess =
    let next = 0.5 *. (guess +. radicand /. guess) in
    if abs_float (next -. guess) < 0.0001 then int_of_float next
    else aux next
  in
  aux (radicand /. 2.0)
