(* 1021: Error Propagation Depth
   5-level error propagation using Result.bind / let* syntax.
   Each layer produces a specific error variant; composition threads them through. *)

type app_error =
  | ConfigMissing of string
  | ParseFailed of string
  | ValidationFailed of string
  | ServiceUnavailable of string
  | Timeout

let string_of_app_error = function
  | ConfigMissing s         -> Printf.sprintf "config missing: %s" s
  | ParseFailed s           -> Printf.sprintf "parse failed: %s" s
  | ValidationFailed s      -> Printf.sprintf "validation: %s" s
  | ServiceUnavailable s    -> Printf.sprintf "service unavailable: %s" s
  | Timeout                 -> "timeout"

(* Level 1: Config layer *)
let read_config key =
  if key = "missing" then Error (ConfigMissing key)
  else Ok "8080"

(* Level 2: Parse layer *)
let parse_port s =
  match int_of_string_opt s with
  | None   -> Error (ParseFailed s)
  | Some n ->
    if n >= 0 && n <= 65535 then Ok n
    else Error (ParseFailed (Printf.sprintf "out of range: %d" n))

(* Level 3: Validation layer *)
let validate_port port =
  if port = 0 then Error (ValidationFailed (Printf.sprintf "port %d invalid" port))
  else Ok port

(* Level 4: Connection layer *)
let connect _host port =
  if port = 9999 then Error (ServiceUnavailable "connection refused")
  else Ok (Printf.sprintf "connected:%d" port)

(* Level 5: Application layer — chains all with let* *)
let ( let* ) = Result.bind

let start_service key host =
  let* raw   = read_config key     in   (* Level 1 *)
  let* port  = parse_port raw      in   (* Level 2 *)
  let* valid = validate_port port  in   (* Level 3 *)
  let* conn  = connect host valid  in   (* Level 4 *)
  Ok conn                               (* Level 5 success *)

let () =
  assert (start_service "port" "localhost" = Ok "connected:8080");

  (match start_service "missing" "localhost" with
   | Error (ConfigMissing _) -> ()
   | _ -> assert false);

  (match parse_port "abc" with
   | Error (ParseFailed _) -> ()
   | _ -> assert false);

  (match validate_port 0 with
   | Error (ValidationFailed _) -> ()
   | _ -> assert false);

  (match connect "host" 9999 with
   | Error (ServiceUnavailable _) -> ()
   | _ -> assert false);

  assert (string_of_app_error (ConfigMissing "db_url") = "config missing: db_url");
  assert (string_of_app_error Timeout = "timeout");

  (* Each ? passes error unchanged *)
  (match (let* _ = read_config "missing" in Ok ()) with
   | Error (ConfigMissing _) -> ()
   | _ -> assert false);

  (* All layers independently happy *)
  assert (Result.is_ok (read_config "ok"));
  assert (Result.is_ok (parse_port "8080"));
  assert (Result.is_ok (validate_port 80));
  assert (Result.is_ok (connect "localhost" 80));

  Printf.printf "start_service: %s\n"
    (match start_service "port" "localhost" with
     | Ok s -> s
     | Error e -> string_of_app_error e)
