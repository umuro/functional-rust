(* Example 084: From/Into Traits *)
(* OCaml coercion → Rust explicit conversions *)

(* Approach 1: Explicit conversion functions *)
type celsius = { c : float }
type fahrenheit = { f : float }

let celsius_of_float v = { c = v }
let fahrenheit_of_float v = { f = v }
let fahrenheit_of_celsius c = { f = c.c *. 9.0 /. 5.0 +. 32.0 }
let celsius_of_fahrenheit f = { c = (f.f -. 32.0) *. 5.0 /. 9.0 }

(* Approach 2: String conversions *)
let int_of_string_opt s =
  try Some (int_of_string s) with Failure _ -> None

let string_of_point (x, y) =
  Printf.sprintf "(%d, %d)" x y

let point_of_string s =
  try Scanf.sscanf s "(%d, %d)" (fun x y -> Some (x, y))
  with _ -> None

(* Approach 3: Conversion via module signatures *)
module type Convertible = sig
  type src
  type dst
  val convert : src -> dst
end

module IntToFloat : Convertible with type src = int and type dst = float = struct
  type src = int
  type dst = float
  let convert = float_of_int
end

module StringToChars : Convertible with type src = string and type dst = char list = struct
  type src = string
  type dst = char list
  let convert s = List.init (String.length s) (String.get s)
end

(* Chained conversions *)
let celsius_string_of_fahrenheit_string s =
  match float_of_string_opt s with
  | None -> None
  | Some v ->
    let f = fahrenheit_of_float v in
    let c = celsius_of_fahrenheit f in
    Some (Printf.sprintf "%.1f°C" c.c)

(* Tests *)
let () =
  let c = celsius_of_float 100.0 in
  let f = fahrenheit_of_celsius c in
  assert (abs_float (f.f -. 212.0) < 0.01);

  let f2 = fahrenheit_of_float 32.0 in
  let c2 = celsius_of_fahrenheit f2 in
  assert (abs_float (c2.c -. 0.0) < 0.01);

  assert (int_of_string_opt "42" = Some 42);
  assert (int_of_string_opt "abc" = None);

  assert (string_of_point (3, 4) = "(3, 4)");
  assert (point_of_string "(3, 4)" = Some (3, 4));

  assert (IntToFloat.convert 42 = 42.0);
  assert (StringToChars.convert "hi" = ['h'; 'i']);

  assert (celsius_string_of_fahrenheit_string "212" = Some "100.0°C");
  assert (celsius_string_of_fahrenheit_string "abc" = None);

  Printf.printf "✓ All tests passed\n"
