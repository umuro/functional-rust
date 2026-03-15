(* 084: From/Into — OCaml conversion functions *)

(* Approach 1: Simple conversions *)
type celsius = Celsius of float
type fahrenheit = Fahrenheit of float

let fahrenheit_of_celsius (Celsius c) = Fahrenheit (c *. 9.0 /. 5.0 +. 32.0)
let celsius_of_fahrenheit (Fahrenheit f) = Celsius ((f -. 32.0) *. 5.0 /. 9.0)

(* Approach 2: String conversions *)
type color = Red | Green | Blue

let color_of_string = function
  | "red" -> Ok Red
  | "green" -> Ok Green
  | "blue" -> Ok Blue
  | s -> Error (Printf.sprintf "Unknown color: %s" s)

let string_of_color = function
  | Red -> "red"
  | Green -> "green"
  | Blue -> "blue"

(* Approach 3: Complex record conversion *)
type raw_user = { raw_name: string; raw_age: string; raw_email: string }
type user = { name: string; age: int; email: string }

let user_of_raw raw =
  match int_of_string_opt raw.raw_age with
  | None -> Error "Invalid age"
  | Some age -> Ok { name = raw.raw_name; age; email = raw.raw_email }

(* Tests *)
let () =
  let (Fahrenheit f) = fahrenheit_of_celsius (Celsius 100.0) in
  assert (abs_float (f -. 212.0) < 0.001);
  let (Celsius c) = celsius_of_fahrenheit (Fahrenheit 32.0) in
  assert (abs_float c < 0.001);
  assert (color_of_string "red" = Ok Red);
  assert (Result.is_error (color_of_string "purple"));
  assert (string_of_color Blue = "blue");
  let raw = { raw_name = "Alice"; raw_age = "30"; raw_email = "a@b.com" } in
  assert (user_of_raw raw = Ok { name = "Alice"; age = 30; email = "a@b.com" });
  let bad = { raw_name = "Bob"; raw_age = "xyz"; raw_email = "b@c.com" } in
  assert (Result.is_error (user_of_raw bad));
  Printf.printf "✓ All tests passed\n"
