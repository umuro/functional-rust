(* Phantom Types — Type-Safe Units *)
(* The phantom type parameter 'a exists only at the type level *)

type meters
type seconds
type 'a quantity = Q of float

let meters x : meters quantity = Q x
let seconds x : seconds quantity = Q x

let add (Q a : 'a quantity) (Q b : 'a quantity) : 'a quantity = Q (a +. b)
let scale k (Q a : 'a quantity) : 'a quantity = Q (k *. a)
let value (Q x) = x

let () =
  let d1 = meters 100.0 in
  let d2 = meters 50.0 in
  let total = add d1 d2 in
  Printf.printf "Distance: %.1f m\n" (value total);
  (* add d1 (seconds 5.0) would be a type error! *)
  let doubled = scale 2.0 (seconds 3.0) in
  Printf.printf "Time: %.1f s\n" (value doubled);
  Printf.printf "Phantom type tests passed!\n"
