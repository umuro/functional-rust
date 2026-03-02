let twice f x = f (f x)

let double x = 2 * x
let square x = x * x

let quad   = twice double
let fourth = twice square

let () =
  Printf.printf "quad 3   = %d\n" (quad 3);
  Printf.printf "fourth 2 = %d\n" (fourth 2)
