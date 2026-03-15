(* Example 081: Newtype Pattern *)
(* OCaml module types → Rust tuple structs for type safety *)

(* Approach 1: Abstract types via modules *)
module UserId : sig
  type t
  val create : int -> t
  val value : t -> int
  val to_string : t -> string
end = struct
  type t = int
  let create x = if x > 0 then x else failwith "invalid user id"
  let value x = x
  let to_string = string_of_int
end

module OrderId : sig
  type t
  val create : int -> t
  val value : t -> int
  val to_string : t -> string
end = struct
  type t = int
  let create x = if x > 0 then x else failwith "invalid order id"
  let value x = x
  let to_string = string_of_int
end

(* Approach 2: Private type abbreviation *)
module Email : sig
  type t = private string
  val create : string -> t option
  val to_string : t -> string
end = struct
  type t = string
  let create s = if String.contains s '@' then Some s else None
  let to_string s = s
end

(* Approach 3: Record wrapper *)
type celsius = { celsius_value : float }
type fahrenheit = { fahrenheit_value : float }

let celsius_of_float v = { celsius_value = v }
let fahrenheit_of_float v = { fahrenheit_value = v }

let to_fahrenheit c =
  { fahrenheit_value = c.celsius_value *. 9.0 /. 5.0 +. 32.0 }

let to_celsius f =
  { celsius_value = (f.fahrenheit_value -. 32.0) *. 5.0 /. 9.0 }

(* Cannot accidentally mix: to_fahrenheit expects celsius *)

(* Tests *)
let () =
  let uid = UserId.create 42 in
  let oid = OrderId.create 100 in
  assert (UserId.value uid = 42);
  assert (OrderId.value oid = 100);
  (* UserId and OrderId are incompatible types *)

  (match Email.create "user@example.com" with
   | Some e -> assert (Email.to_string e = "user@example.com")
   | None -> assert false);
  assert (Email.create "invalid" = None);

  let c = celsius_of_float 100.0 in
  let f = to_fahrenheit c in
  assert (abs_float (f.fahrenheit_value -. 212.0) < 0.01);
  let c2 = to_celsius f in
  assert (abs_float (c2.celsius_value -. 100.0) < 0.01);

  Printf.printf "✓ All tests passed\n"
