(* 1004: Error Conversion
   Rust's From trait auto-converts sub-errors via ?. In OCaml we use
   Result.map_error to wrap low-level errors into a unified app error type.
   Composition replaces automatic conversion. *)

type io_error =
  | FileNotFound of string
  | PermissionDenied of string

let string_of_io_error = function
  | FileNotFound p -> Printf.sprintf "file not found: %s" p
  | PermissionDenied p -> Printf.sprintf "permission denied: %s" p

type parse_error = ParseFailed of string

let string_of_parse_error (ParseFailed s) = Printf.sprintf "parse error: %s" s

(* Unified app error — wraps subsystem errors *)
type app_error =
  | Io of io_error
  | Parse of parse_error

let string_of_app_error = function
  | Io e    -> Printf.sprintf "IO: %s" (string_of_io_error e)
  | Parse e -> Printf.sprintf "Parse: %s" (string_of_parse_error e)

(* Low-level IO function *)
let read_config path =
  if path = "/missing" then Error (FileNotFound path)
  else Ok "42"

(* Parse a string as int — returns parse_error *)
let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error (ParseFailed (Printf.sprintf "not an int: %s" s))

(* Application layer: wraps sub-errors into app_error using Result.map_error *)
let load_config path =
  Result.map_error (fun e -> Io e) (read_config path)
  |> Result.bind (fun content ->
       Result.map_error (fun e -> Parse e) (parse_int content))

let () =
  assert (load_config "/ok" = Ok 42);

  (match load_config "/missing" with
   | Error (Io (FileNotFound _)) -> ()
   | _ -> assert false);

  (match load_config "/bad" with
   | _ -> ());  (* /bad reads "42" which is valid *)

  (* Manual from-style conversions *)
  let io_as_app = Result.map_error (fun e -> Io e) (read_config "/missing") in
  assert (Result.is_error io_as_app);

  Printf.printf "load_config /ok: %s\n"
    (match load_config "/ok" with
     | Ok n -> string_of_int n
     | Error e -> string_of_app_error e)
