(* 1018: Error Downcast
   Rust downcasts Box<dyn Error> to concrete types via TypeId.
   OCaml achieves the same with extensible variants + pattern matching,
   or by tagging errors in a unified variant type.

   We show both: a tagged union approach and an extensible exception approach. *)

(* Approach 1: Tagged union — safe, type-checked "downcast" *)
type error_kind =
  | DatabaseErr of string
  | AuthErr of string
  | NetworkErr of string

let string_of_error_kind = function
  | DatabaseErr s -> Printf.sprintf "database error: %s" s
  | AuthErr s     -> Printf.sprintf "auth error: %s" s
  | NetworkErr s  -> Printf.sprintf "network error: %s" s

let classify_error = function
  | DatabaseErr _ -> "database"
  | AuthErr _     -> "auth"
  | NetworkErr _  -> "network"

let is_database_error = function
  | DatabaseErr _ -> true
  | _             -> false

let handle_error = function
  | DatabaseErr s -> Printf.sprintf "Handling DB: %s" s
  | _             -> "unhandled error"

(* Approach 2: OCaml's extensible exception type — works like Box<dyn Any> *)
exception Db_exn of string
exception Auth_exn of string
exception Net_exn of string

(* "Downcast" by catching the specific exception type *)
let classify_exn exn =
  match exn with
  | Db_exn _   -> "database"
  | Auth_exn _ -> "auth"
  | Net_exn _  -> "network"
  | _          -> "unknown"

let might_fail_db () = raise (Db_exn "timeout")
let might_fail_auth () = raise (Auth_exn "expired token")

let () =
  (* Tagged union approach *)
  let err = DatabaseErr "test" in
  assert (classify_error err = "database");
  assert (handle_error err = "Handling DB: test");

  let err2 = AuthErr "bad" in
  assert (classify_error err2 = "auth");

  assert (is_database_error (DatabaseErr "x") = true);
  assert (is_database_error (AuthErr "x") = false);

  assert (handle_error (AuthErr "nope") = "unhandled error");

  (* Extensible exception approach *)
  (try might_fail_db ()
   with exn ->
     assert (classify_exn exn = "database");
     (match exn with
      | Db_exn s -> assert (s = "timeout")
      | _ -> assert false));

  (try might_fail_auth ()
   with exn ->
     assert (classify_exn exn = "auth"));

  (* Unknown exception *)
  assert (classify_exn Exit = "unknown");

  Printf.printf "Downcast tests passed\n"
