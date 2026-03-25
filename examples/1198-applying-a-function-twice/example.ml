let twice f x = f (f x)

let double x = 2 * x
let square x = x * x

(* Partial application — OCaml automatically curries `twice` *)
let quad   = twice double   (* applies double twice *)
let fourth = twice square   (* applies square twice *)

let () =
  assert (quad   3 = 12);
  assert (fourth 2 = 16);
  assert (twice (fun x -> x + 10) 5 = 25);
  Printf.printf "quad 3   = %d\n" (quad 3);
  Printf.printf "fourth 2 = %d\n" (fourth 2);
  print_endline "ok"
