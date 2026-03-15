(* 1015: Validation Errors — Accumulating All Errors
   Unlike Result.bind which short-circuits, validation collects ALL errors.
   We return a list of field errors. No short-circuiting — every field is checked. *)

type field_error = {
  field: string;
  message: string;
}

let string_of_field_error e =
  Printf.sprintf "%s: %s" e.field e.message

let validate_name name =
  let errors = ref [] in
  if name = "" then
    errors := { field = "name"; message = "required" } :: !errors;
  if String.length name > 50 then
    errors := { field = "name"; message = "too long" } :: !errors;
  List.rev !errors

let validate_age age =
  let errors = ref [] in
  if age < 0 then
    errors := { field = "age"; message = "negative" } :: !errors;
  if age > 150 then
    errors := { field = "age"; message = "unreasonable" } :: !errors;
  List.rev !errors

let validate_email email =
  let errors = ref [] in
  if email = "" then
    errors := { field = "email"; message = "required" } :: !errors;
  if not (String.contains email '@') then
    errors := { field = "email"; message = "missing @" } :: !errors;
  List.rev !errors

type valid_form = {
  name: string;
  age: int;
  email: string;
}

(* Approach 1: Collect errors from each validator — all fields checked *)
let validate_form name age email =
  let errors =
    validate_name name
    @ validate_age age
    @ validate_email email
  in
  if errors = [] then Ok { name; age; email }
  else Error errors

(* Approach 2: Functional with a check list *)
let run_checks field value checks =
  List.filter_map (fun (pred, msg) ->
    if not (pred value) then Some { field; message = msg }
    else None
  ) checks

let validate_form_functional name age email =
  let name_checks = [
    ((fun s -> s <> ""), "required");
    ((fun s -> String.length s <= 50), "too long");
  ] in
  let age_checks = [
    ((fun n -> n >= 0), "negative");
    ((fun n -> n <= 150), "unreasonable");
  ] in
  let errors =
    run_checks "name" name name_checks
    @ run_checks "age" age age_checks
    @ validate_email email
  in
  if errors = [] then Ok { name; age; email }
  else Error errors

let () =
  (* All errors are collected — no short-circuit *)
  (match validate_form "" (-5) "bademail" with
   | Error errors ->
     assert (List.length errors >= 3);
     assert (List.exists (fun e -> e.field = "name") errors);
     assert (List.exists (fun e -> e.field = "age") errors);
     assert (List.exists (fun e -> e.field = "email") errors)
   | _ -> assert false);

  (match validate_form "Alice" 30 "a@b.com" with
   | Ok form ->
     assert (form.name = "Alice");
     assert (form.age = 30)
   | _ -> assert false);

  (match validate_form "Alice" 30 "no-at" with
   | Error errors ->
     assert (List.length errors = 1);
     assert ((List.hd errors).field = "email")
   | _ -> assert false);

  (match validate_form_functional "" (-1) "bad" with
   | Error errors -> assert (List.length errors >= 3)
   | _ -> assert false);

  (* empty + negative age + empty email + missing @ = at least 4 errors *)
  (match validate_form "" (-5) "" with
   | Error errors -> assert (List.length errors >= 4)
   | _ -> assert false);

  Printf.printf "All validation error tests passed\n"
