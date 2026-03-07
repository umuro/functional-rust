let twice f x = f (f x)

let double x = 2 * x
let square x = x * x

let quad   = twice double   (* applies double twice *)
let fourth = twice square   (* applies square twice *)

let () =
  Printf.printf "quad 3   = %d\n" (quad 3);    (* 12 *)
  Printf.printf "fourth 2 = %d\n" (fourth 2)   (* 16 *)