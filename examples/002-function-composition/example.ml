let compose f g x = f (g x)

let double x = 2 * x
let square x = x * x

(* square first, then double *)
let square_then_double = compose double square

let () =
  Printf.printf "square_then_double 3 = %d\n" (square_then_double 3);  (* 18 *)
  Printf.printf "square_then_double 4 = %d\n" (square_then_double 4)   (* 32 *)