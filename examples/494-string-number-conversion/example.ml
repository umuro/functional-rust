(* 494: Number <-> String conversion in OCaml *)
(* OCaml uses string_of_int, int_of_string, Printf.sprintf, Scanf.sscanf *)

(* int → string *)
let int_to_string n = string_of_int n

(* string → int option (safe parse) *)
let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None   -> Error ("not an integer: " ^ s)

(* int → hex string — Printf.sprintf with %x *)
let to_hex n = Printf.sprintf "%x" n

(* hex string → int *)
let from_hex s =
  match int_of_string_opt ("0x" ^ s) with
  | Some n -> Ok n
  | None   -> Error ("not a hex integer: " ^ s)

(* float → formatted string *)
let float_to_fixed2 f = Printf.sprintf "%.2f" f

(* string → float option *)
let parse_float s =
  match float_of_string_opt s with
  | Some f -> Ok f
  | None   -> Error ("not a float: " ^ s)

let () =
  (* int to_string *)
  assert (int_to_string 42 = "42");
  assert (int_to_string (-7) = "-7");
  print_endline "int_to_string: ok";

  (* parse int *)
  assert (parse_int "42" = Ok 42);
  assert (Result.is_error (parse_int "abc"));
  print_endline "parse_int: ok";

  (* hex *)
  assert (to_hex 255 = "ff");
  assert (from_hex "ff" = Ok 255);
  print_endline "hex: ok";

  (* float *)
  assert (float_to_fixed2 3.14159 = "3.14");
  assert (Result.is_ok (parse_float "3.14"));
  print_endline "float: ok";

  (* negative *)
  assert (int_to_string (-7) = "-7");
  print_endline "negative: ok";

  print_endline "All assertions passed."
