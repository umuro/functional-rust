(* 1017: Typed Error Hierarchy *)
(* Enum with variants for each subsystem *)

type db_error = ConnectionFailed | QueryFailed of string | NotFound of string
type auth_error = InvalidToken | Expired | Forbidden of string
type api_error = BadRequest of string | RateLimit

type app_error =
  | Db of db_error
  | Auth of auth_error
  | Api of api_error

let string_of_db_error = function
  | ConnectionFailed -> "database connection failed"
  | QueryFailed q -> Printf.sprintf "query failed: %s" q
  | NotFound id -> Printf.sprintf "not found: %s" id

let string_of_auth_error = function
  | InvalidToken -> "invalid token"
  | Expired -> "token expired"
  | Forbidden r -> Printf.sprintf "forbidden: %s" r

let string_of_api_error = function
  | BadRequest msg -> Printf.sprintf "bad request: %s" msg
  | RateLimit -> "rate limited"

let string_of_app_error = function
  | Db e -> Printf.sprintf "[DB] %s" (string_of_db_error e)
  | Auth e -> Printf.sprintf "[Auth] %s" (string_of_auth_error e)
  | Api e -> Printf.sprintf "[API] %s" (string_of_api_error e)

(* Subsystem functions return their own error types *)
let db_find_user id =
  if id = "missing" then Error (NotFound id)
  else Ok (Printf.sprintf "user_%s" id)

let auth_check token =
  if token = "" then Error InvalidToken
  else if token = "expired" then Error Expired
  else Ok "valid"

(* App layer lifts sub-errors *)
let get_user token user_id =
  match auth_check token with
  | Error e -> Error (Auth e)
  | Ok _ ->
    match db_find_user user_id with
    | Error e -> Error (Db e)
    | Ok user -> Ok user

let test_hierarchy () =
  assert (get_user "valid" "123" = Ok "user_123");
  (match get_user "" "123" with
   | Error (Auth InvalidToken) -> ()
   | _ -> assert false);
  (match get_user "valid" "missing" with
   | Error (Db (NotFound _)) -> ()
   | _ -> assert false);
  Printf.printf "  Typed hierarchy: passed\n"

let test_display () =
  let err = Db (QueryFailed "SELECT *") in
  assert (string_of_app_error err = "[DB] query failed: SELECT *");
  let err = Auth Expired in
  assert (string_of_app_error err = "[Auth] token expired");
  Printf.printf "  Display formatting: passed\n"

let () =
  Printf.printf "Testing typed error hierarchy:\n";
  test_hierarchy ();
  test_display ();
  Printf.printf "✓ All tests passed\n"
