(* 1015: Validation Errors — Accumulating All Errors *)
(* Not short-circuiting: collect ALL validation failures *)

type field_error = { field: string; message: string }

(* Approach 1: Validate each field, collect errors *)
let validate_name name =
  if String.length name = 0 then [{ field = "name"; message = "required" }]
  else if String.length name > 50 then [{ field = "name"; message = "too long" }]
  else []

let validate_age age =
  if age < 0 then [{ field = "age"; message = "negative" }]
  else if age > 150 then [{ field = "age"; message = "unreasonable" }]
  else []

let validate_email email =
  let errors = ref [] in
  if String.length email = 0 then
    errors := { field = "email"; message = "required" } :: !errors;
  if not (String.contains email '@') then
    errors := { field = "email"; message = "missing @" } :: !errors;
  List.rev !errors

let validate_form name age email =
  let errors = validate_name name @ validate_age age @ validate_email email in
  if errors = [] then Ok (name, age, email)
  else Error errors

(* Approach 2: Applicative-style validation *)
let validate_field field pred value msg =
  if pred value then [] else [{ field; message = msg }]

let validate_form_app name age email =
  let errors =
    validate_field "name" (fun s -> String.length s > 0) name "required"
    @ validate_field "name" (fun s -> String.length s <= 50) name "too long"
    @ validate_field "age" (fun n -> n >= 0) age "negative"
    @ validate_field "age" (fun n -> n <= 150) age "unreasonable"
    @ validate_field "email" (fun s -> String.contains s '@') email "missing @"
  in
  if errors = [] then Ok (name, age, email)
  else Error errors

let test_all_errors () =
  (* Multiple errors accumulated *)
  (match validate_form "" (-5) "bademail" with
   | Error errs ->
     assert (List.length errs >= 3);
     assert (List.exists (fun e -> e.field = "name") errs);
     assert (List.exists (fun e -> e.field = "age") errs);
     assert (List.exists (fun e -> e.field = "email") errs);
   | Ok _ -> assert false);
  Printf.printf "  Approach 1 (accumulate all): passed\n"

let test_valid () =
  assert (validate_form "Alice" 30 "a@b.com" = Ok ("Alice", 30, "a@b.com"));
  Printf.printf "  Valid input: passed\n"

let test_applicative () =
  (match validate_form_app "" (-1) "bad" with
   | Error errs -> assert (List.length errs >= 3)
   | Ok _ -> assert false);
  assert (validate_form_app "Bob" 25 "b@c.com" = Ok ("Bob", 25, "b@c.com"));
  Printf.printf "  Approach 2 (applicative): passed\n"

let () =
  Printf.printf "Testing validation errors:\n";
  test_all_errors ();
  test_valid ();
  test_applicative ();
  Printf.printf "✓ All tests passed\n"
