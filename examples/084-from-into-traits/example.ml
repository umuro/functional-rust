(* 084: From / Into / TryFrom / TryInto
   OCaml expresses conversions via explicit functions or module coercions *)

(* --- Approach 1: Temperature conversion (From/Into analogue) --- *)

type celsius    = { c : float }
type fahrenheit = { f : float }

(* Explicit conversion functions — idiomatic OCaml *)
let celsius_of_fahrenheit { f } = { c = (f -. 32.0) *. 5.0 /. 9.0 }
let fahrenheit_of_celsius { c } = { f = c *. 9.0 /. 5.0 +. 32.0 }

(* --- Approach 2: TryFrom analogue — parsing that can fail --- *)

type color = Red | Green | Blue

let color_of_string = function
  | "red"   -> Ok Red
  | "green" -> Ok Green
  | "blue"  -> Ok Blue
  | s       -> Error (Printf.sprintf "Unknown color: %s" s)

let string_of_color = function
  | Red   -> "red"
  | Green -> "green"
  | Blue  -> "blue"

(* --- Approach 3: Validated construction — TryFrom for complex types --- *)

type raw_user = { raw_name: string; raw_age: string; raw_email: string }
type user     = { name: string; age: int; email: string }

let user_of_raw { raw_name; raw_age; raw_email } =
  match int_of_string_opt raw_age with
  | None     -> Error "Invalid age"
  | Some age -> Ok { name = raw_name; age; email = raw_email }

(* Generic conversion pipeline — pipe through multiple converters *)
let ( |>> ) x f = Result.bind x f

let () =
  (* temperature *)
  let f = fahrenheit_of_celsius { c = 100.0 } in
  Printf.printf "100°C = %.1f°F\n" f.f;
  let c = celsius_of_fahrenheit { f = 32.0 } in
  Printf.printf "32°F = %.1f°C\n" c.c;

  (* color parsing *)
  Printf.printf "color_of_string \"red\" = %s\n"
    (match color_of_string "red" with Ok c -> string_of_color c | Error e -> e);
  Printf.printf "color_of_string \"purple\" = %s\n"
    (match color_of_string "purple" with Ok _ -> "Ok" | Error e -> "Error: " ^ e);

  (* user parsing *)
  let raw = { raw_name = "Alice"; raw_age = "30"; raw_email = "a@b.com" } in
  Printf.printf "valid user age = %s\n"
    (match user_of_raw raw with Ok u -> string_of_int u.age | Error e -> e);

  let bad = { raw_name = "Bob"; raw_age = "xyz"; raw_email = "b@c.com" } in
  Printf.printf "invalid age = %s\n"
    (match user_of_raw bad with Ok _ -> "Ok" | Error e -> e);

  (* pipeline example *)
  let result =
    Ok "42"
    |>> (fun s -> match int_of_string_opt s with None -> Error "parse" | Some n -> Ok n)
    |>> (fun n -> if n > 0 then Ok n else Error "not positive")
    |> Result.map (fun n -> n * 2)
  in
  Printf.printf "pipeline \"42\" -> %s\n"
    (match result with Ok v -> string_of_int v | Error e -> "Error: " ^ e)
