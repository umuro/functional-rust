(* 081: Newtype Pattern *)

(* Approach 1: Simple newtypes *)
type meters = Meters of float
type seconds = Seconds of float
type meters_per_second = MPS of float

let speed (Meters d) (Seconds t) =
  if t = 0.0 then None else Some (MPS (d /. t))

let value_of_meters (Meters m) = m
let value_of_mps (MPS v) = v

(* Approach 2: Distinct ID types *)
type user_id = UserId of int
type order_id = OrderId of int

(* These won't compile: type mismatch! *)
(* let bad = UserId 1 = OrderId 1 *)

let find_user (UserId id) = Printf.sprintf "User #%d" id
let find_order (OrderId id) = Printf.sprintf "Order #%d" id

(* Approach 3: Newtype with operations *)
type celsius = Celsius of float
type fahrenheit = Fahrenheit of float

let to_fahrenheit (Celsius c) = Fahrenheit (c *. 9.0 /. 5.0 +. 32.0)
let to_celsius (Fahrenheit f) = Celsius ((f -. 32.0) *. 5.0 /. 9.0)

(* Tests *)
let () =
  (match speed (Meters 100.0) (Seconds 10.0) with
   | Some (MPS v) -> assert (abs_float (v -. 10.0) < 0.001)
   | None -> assert false);
  assert (speed (Meters 100.0) (Seconds 0.0) = None);
  assert (find_user (UserId 42) = "User #42");
  assert (find_order (OrderId 7) = "Order #7");
  let (Fahrenheit f) = to_fahrenheit (Celsius 100.0) in
  assert (abs_float (f -. 212.0) < 0.001);
  let (Celsius c) = to_celsius (Fahrenheit 32.0) in
  assert (abs_float c < 0.001);
  Printf.printf "✓ All tests passed\n"
