(* 1004: Error Conversion *)
(* OCaml: manual error wrapping vs Rust's From trait *)

type io_error = FileNotFound of string | PermissionDenied of string
type parse_error = InvalidFormat of string | OutOfRange of int

type app_error =
  | IoError of io_error
  | ParseError of parse_error

let string_of_io_error = function
  | FileNotFound s -> Printf.sprintf "file not found: %s" s
  | PermissionDenied s -> Printf.sprintf "permission denied: %s" s

let string_of_parse_error = function
  | InvalidFormat s -> Printf.sprintf "invalid format: %s" s
  | OutOfRange n -> Printf.sprintf "out of range: %d" n

let string_of_app_error = function
  | IoError e -> Printf.sprintf "IO: %s" (string_of_io_error e)
  | ParseError e -> Printf.sprintf "Parse: %s" (string_of_parse_error e)

(* Approach 1: Manual wrapping *)
let read_config_exn path =
  if path = "/missing" then Error (IoError (FileNotFound path))
  else Ok "42"

let parse_value_exn s =
  match int_of_string_opt s with
  | None -> Error (ParseError (InvalidFormat s))
  | Some n when n < 0 -> Error (ParseError (OutOfRange n))
  | Some n -> Ok n

(* Approach 2: Helper to lift sub-errors *)
let lift_io f x =
  match f x with
  | Error e -> Error (IoError e)
  | Ok v -> Ok v

let lift_parse f x =
  match f x with
  | Error e -> Error (ParseError e)
  | Ok v -> Ok v

let read_raw path =
  if path = "/missing" then Error (FileNotFound path)
  else Ok "42"

let parse_raw s =
  match int_of_string_opt s with
  | None -> Error (InvalidFormat s)
  | Some n -> Ok n

let load_config path =
  match lift_io read_raw path with
  | Error e -> Error e
  | Ok s -> lift_parse parse_raw s

let test_manual () =
  assert (read_config_exn "/ok" = Ok "42");
  (match read_config_exn "/missing" with
   | Error (IoError (FileNotFound _)) -> ()
   | _ -> assert false);
  assert (parse_value_exn "42" = Ok 42);
  (match parse_value_exn "abc" with
   | Error (ParseError (InvalidFormat _)) -> ()
   | _ -> assert false);
  Printf.printf "  Approach 1 (manual wrapping): passed\n"

let test_lift () =
  assert (load_config "/ok" = Ok 42);
  (match load_config "/missing" with
   | Error (IoError (FileNotFound _)) -> ()
   | _ -> assert false);
  Printf.printf "  Approach 2 (lift helpers): passed\n"

let () =
  Printf.printf "Testing error conversion:\n";
  test_manual ();
  test_lift ();
  Printf.printf "✓ All tests passed\n"
