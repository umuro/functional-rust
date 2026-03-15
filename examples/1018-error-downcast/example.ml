(* 1018: Error Downcast *)
(* OCaml: pattern matching on exception types *)

(* In OCaml, exceptions are already pattern-matchable *)
exception DatabaseError of string
exception AuthError of string
exception NetworkError of string

(* Approach 1: Direct pattern matching on exceptions *)
let handle_error f =
  try Ok (f ())
  with
  | DatabaseError msg -> Error (Printf.sprintf "DB: %s" msg)
  | AuthError msg -> Error (Printf.sprintf "Auth: %s" msg)
  | NetworkError msg -> Error (Printf.sprintf "Net: %s" msg)
  | exn -> Error (Printf.sprintf "Unknown: %s" (Printexc.to_string exn))

(* Approach 2: Using a general exception container *)
type any_error = ..  (* extensible variant type *)
type any_error += DB of string | Auth of string | Net of string

let classify_error (e : any_error) =
  match e with
  | DB msg -> Printf.sprintf "database: %s" msg
  | Auth msg -> Printf.sprintf "auth: %s" msg
  | Net msg -> Printf.sprintf "network: %s" msg
  | _ -> "unknown error"

(* Approach 3: GADT-style typed errors *)
type _ error_kind =
  | DbKind : string error_kind
  | AuthKind : string error_kind

type packed_error = Pack : 'a error_kind * 'a -> packed_error

let describe_packed (Pack (kind, value)) =
  match kind with
  | DbKind -> Printf.sprintf "DB error: %s" value
  | AuthKind -> Printf.sprintf "Auth error: %s" value

let test_exceptions () =
  let r = handle_error (fun () -> raise (DatabaseError "timeout")) in
  assert (r = Error "DB: timeout");
  let r = handle_error (fun () -> 42) in
  assert (r = Ok 42);
  Printf.printf "  Approach 1 (exception matching): passed\n"

let test_extensible () =
  assert (classify_error (DB "conn failed") = "database: conn failed");
  assert (classify_error (Auth "bad token") = "auth: bad token");
  Printf.printf "  Approach 2 (extensible variants): passed\n"

let test_gadt () =
  let e = Pack (DbKind, "query failed") in
  assert (describe_packed e = "DB error: query failed");
  let e = Pack (AuthKind, "expired") in
  assert (describe_packed e = "Auth error: expired");
  Printf.printf "  Approach 3 (GADT packed): passed\n"

let () =
  Printf.printf "Testing error downcast:\n";
  test_exceptions ();
  test_extensible ();
  test_gadt ();
  Printf.printf "✓ All tests passed\n"
