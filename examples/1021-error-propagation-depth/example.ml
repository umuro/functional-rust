(* 1021: Error Propagation Depth *)
(* 5-level error propagation through layers *)

type app_error =
  | ConfigMissing of string
  | ParseFailed of string
  | ValidationFailed of string
  | ServiceUnavailable of string
  | Timeout

let string_of_error = function
  | ConfigMissing s -> Printf.sprintf "config missing: %s" s
  | ParseFailed s -> Printf.sprintf "parse failed: %s" s
  | ValidationFailed s -> Printf.sprintf "validation: %s" s
  | ServiceUnavailable s -> Printf.sprintf "service unavailable: %s" s
  | Timeout -> "timeout"

let ( let* ) = Result.bind

(* Level 1: Config layer *)
let read_config key =
  if key = "missing" then Error (ConfigMissing key)
  else Ok "8080"

(* Level 2: Parse layer *)
let parse_port s =
  match int_of_string_opt s with
  | None -> Error (ParseFailed s)
  | Some n -> Ok n

(* Level 3: Validation layer *)
let validate_port port =
  if port < 1 || port > 65535 then
    Error (ValidationFailed (Printf.sprintf "port %d out of range" port))
  else Ok port

(* Level 4: Connection layer *)
let connect _host port =
  if port = 9999 then Error (ServiceUnavailable "connection refused")
  else Ok (Printf.sprintf "connected:%d" port)

(* Level 5: Application layer — chains all 4 *)
let start_service key host =
  let* raw = read_config key in
  let* port = parse_port raw in
  let* valid_port = validate_port port in
  let* conn = connect host valid_port in
  Ok conn

let test_success () =
  assert (start_service "port" "localhost" = Ok "connected:8080");
  Printf.printf "  Success path: passed\n"

let test_config_error () =
  (match start_service "missing" "localhost" with
   | Error (ConfigMissing _) -> ()
   | _ -> assert false);
  Printf.printf "  Config error (level 1): passed\n"

let test_validation_error () =
  (* We'd need a way to make parse return out-of-range...
     Let's test validate directly *)
  assert (validate_port 0 |> Result.is_error);
  assert (validate_port 70000 |> Result.is_error);
  assert (validate_port 8080 = Ok 8080);
  Printf.printf "  Validation error (level 3): passed\n"

let test_all_levels () =
  (* Each level can fail independently *)
  let results = [
    start_service "missing" "localhost";  (* level 1 fail *)
    start_service "port" "localhost";     (* success *)
  ] in
  assert (List.length (List.filter Result.is_ok results) = 1);
  assert (List.length (List.filter Result.is_error results) = 1);
  Printf.printf "  All levels: passed\n"

let () =
  Printf.printf "Testing 5-level error propagation:\n";
  test_success ();
  test_config_error ();
  test_validation_error ();
  test_all_levels ();
  Printf.printf "✓ All tests passed\n"
