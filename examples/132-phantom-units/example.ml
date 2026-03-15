(* Example 132: Units of Measure via Phantom Types *)

(* Approach 1: Phantom type units *)
type meters
type seconds
type kilograms

type 'unit quantity = { value : float }

let meters v : meters quantity = { value = v }
let seconds v : seconds quantity = { value = v }
let kilograms v : kilograms quantity = { value = v }

let add (a : 'u quantity) (b : 'u quantity) : 'u quantity =
  { value = a.value +. b.value }

let scale (a : 'u quantity) (s : float) : 'u quantity =
  { value = a.value *. s }

let get_value (q : 'u quantity) = q.value

(* Approach 2: Module-based units *)
module type UNIT = sig
  type t
  val name : string
end

module Meters : UNIT = struct type t = meters let name = "m" end
module Seconds : UNIT = struct type t = seconds let name = "s" end

module Quantity (U : UNIT) = struct
  type t = float
  let create v = v
  let add a b = a +. b
  let to_string v = Printf.sprintf "%.2f %s" v U.name
end

module M = Quantity(Meters)
module S = Quantity(Seconds)

(* Approach 3: Speed = meters / seconds *)
type speed
let compute_speed (d : meters quantity) (t : seconds quantity) : speed quantity =
  { value = d.value /. t.value }

(* Tests *)
let () =
  let d1 = meters 100.0 in
  let d2 = meters 50.0 in
  let total = add d1 d2 in
  assert (get_value total = 150.0);

  let t = seconds 10.0 in
  let s = compute_speed total t in
  assert (get_value s = 15.0);

  let scaled = scale d1 2.0 in
  assert (get_value scaled = 200.0);

  (* Module-based *)
  let m1 = M.create 5.0 in
  let m2 = M.create 3.0 in
  assert (M.add m1 m2 = 8.0);

  Printf.printf "✓ All tests passed\n"
