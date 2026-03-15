(* 1017: Typed Error Hierarchy
   Subsystem-specific error types unified by an app-level variant.
   Pattern matching ensures all subsystems are handled exhaustively. *)

(* Subsystem error types *)
type db_error =
  | DbConnectionFailed
  | DbQueryFailed of string
  | DbNotFound of string

let string_of_db_error = function
  | DbConnectionFailed  -> "database connection failed"
  | DbQueryFailed q     -> Printf.sprintf "query failed: %s" q
  | DbNotFound id       -> Printf.sprintf "not found: %s" id

type auth_error =
  | InvalidToken
  | Expired
  | Forbidden of string

let string_of_auth_error = function
  | InvalidToken  -> "invalid token"
  | Expired       -> "token expired"
  | Forbidden r   -> Printf.sprintf "forbidden: %s" r

type api_error =
  | BadRequest of string
  | RateLimit

let string_of_api_error = function
  | BadRequest msg -> Printf.sprintf "bad request: %s" msg
  | RateLimit      -> "rate limited"

(* Top-level unified error *)
type app_error =
  | Db of db_error
  | Auth of auth_error
  | Api of api_error

let string_of_app_error = function
  | Db e   -> Printf.sprintf "[DB] %s" (string_of_db_error e)
  | Auth e -> Printf.sprintf "[Auth] %s" (string_of_auth_error e)
  | Api e  -> Printf.sprintf "[API] %s" (string_of_api_error e)

(* Subsystem functions *)
let db_find_user id =
  if id = "missing" then Error (DbNotFound id)
  else Ok (Printf.sprintf "user_%s" id)

let auth_check token =
  if token = ""         then Error InvalidToken
  else if token = "expired" then Error Expired
  else Ok ()

(* App layer: wraps sub-errors with Result.map_error *)
let get_user token user_id =
  let ( let* ) = Result.bind in
  let* () = Result.map_error (fun e -> Auth e) (auth_check token) in
  let* user = Result.map_error (fun e -> Db e) (db_find_user user_id) in
  Ok user

let () =
  assert (get_user "valid" "123" = Ok "user_123");

  (match get_user "" "123" with
   | Error (Auth InvalidToken) -> ()
   | _ -> assert false);

  (match get_user "expired" "123" with
   | Error (Auth Expired) -> ()
   | _ -> assert false);

  (match get_user "valid" "missing" with
   | Error (Db (DbNotFound _)) -> ()
   | _ -> assert false);

  let err = Db (DbQueryFailed "SELECT *") in
  assert (string_of_app_error err = "[DB] query failed: SELECT *");

  let err2 = Auth Expired in
  assert (string_of_app_error err2 = "[Auth] token expired");

  (* Exhaustive matching — compiler ensures all subsystems handled *)
  let handle = function
    | Db _   -> "database issue"
    | Auth _ -> "auth issue"
    | Api _  -> "api issue"
  in
  assert (handle (Api RateLimit) = "api issue");

  Printf.printf "get_user valid 123: %s\n"
    (match get_user "valid" "123" with Ok s -> s | Error e -> string_of_app_error e)
