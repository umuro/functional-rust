(* Higher-Order — Function as Return Value *)
(* Functions that return functions *)

let make_adder n = fun x -> x + n
let make_multiplier n = fun x -> x * n

let make_validator ~min ~max =
  fun x -> x >= min && x <= max

let is_valid_age = make_validator ~min:0 ~max:150
let is_valid_score = make_validator ~min:0 ~max:100

(* Function factory *)
let make_counter () =
  let n = ref 0 in
  fun () -> incr n; !n

let c1 = make_counter ()
let c2 = make_counter ()
let () =
  Printf.printf "c1: %d %d %d\n" (c1 ()) (c1 ()) (c1 ());
  Printf.printf "c2: %d %d\n" (c2 ()) (c2 ())
