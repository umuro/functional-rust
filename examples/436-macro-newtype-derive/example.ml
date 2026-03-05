(* Newtype derive in OCaml *)

(* Phantom type wrapper with all needed operations *)
type meters = Meters of float

(* Manually implement all operations *)
let add_m (Meters a) (Meters b) = Meters (a +. b)
let sub_m (Meters a) (Meters b) = Meters (a -. b)
let mul_m (Meters a) s = Meters (a *. s)
let cmp_m (Meters a) (Meters b) = compare a b
let show_m (Meters a) = Printf.sprintf "%.2fm" a
let float_of_m (Meters a) = a

(* Simulate derive for newtypes *)
module Meters = struct
  type t = Meters of float
  let create v = Meters v
  let value (Meters v) = v
  let add (Meters a) (Meters b) = Meters (a +. b)
  let sub (Meters a) (Meters b) = Meters (a -. b)
  let to_string (Meters v) = Printf.sprintf "%.2fm" v
  let compare (Meters a) (Meters b) = compare a b
end

let () =
  let d1 = Meters.create 5.0 in
  let d2 = Meters.create 3.0 in
  Printf.printf "d1 = %s\n" (Meters.to_string d1);
  Printf.printf "d1 + d2 = %s\n" (Meters.to_string (Meters.add d1 d2));
  Printf.printf "d1 > d2: %b\n" (Meters.compare d1 d2 > 0)
