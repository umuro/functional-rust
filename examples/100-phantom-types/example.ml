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
  let doubled = scale 2.0 (seconds 3.0) in
  Printf.printf "Time: %.1f s\n" (value doubled)
