(* Idiomatic OCaml — applies f twice to x *)
let twice f x = f (f x)

(* Named functions *)
let double x = 2 * x
let square x = x * x

(* Partial application: twice returns a function when given one argument *)
let quad   = twice double   (* applies double twice *)
let fourth = twice square   (* applies square twice *)

let () =
  assert (twice double 3  = 12);
  assert (twice square 2  = 16);
  assert (quad 3          = 12);
  assert (fourth 2        = 16);
  assert (twice (fun x -> x + 1) 5 = 7);
  print_endline "ok"
