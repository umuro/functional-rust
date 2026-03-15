(* 1003: Custom Error Types
   OCaml models errors as variant types. Pattern matching on error variants
   replaces Rust's Display impl. We use Result for fallible operations. *)

(* Approach 1: Simple error variant *)
type validation_error =
  | NegativeAge of int
  | UnreasonableAge of int
  | EmptyName

let string_of_validation_error = function
  | NegativeAge n    -> Printf.sprintf "negative age: %d" n
  | UnreasonableAge n -> Printf.sprintf "unreasonable age: %d" n
  | EmptyName        -> "name cannot be empty"

let validate_age age =
  if age < 0 then Error (NegativeAge age)
  else if age > 150 then Error (UnreasonableAge age)
  else Ok age

let validate_name name =
  if name = "" then Error EmptyName
  else Ok name

(* Approach 2: Structured error with context fields *)
type detailed_error = {
  field: string;
  message: string;
}

let string_of_detailed_error e =
  Printf.sprintf "field '%s': %s" e.field e.message

let validate_field field value =
  if value = "" then
    Error { field; message = "cannot be empty" }
  else
    Ok ()

let () =
  assert (validate_age 25 = Ok 25);
  assert (validate_age (-5) = Error (NegativeAge (-5)));
  assert (validate_age 200 = Error (UnreasonableAge 200));
  assert (string_of_validation_error (NegativeAge (-1)) = "negative age: -1");
  assert (string_of_validation_error EmptyName = "name cannot be empty");
  assert (validate_name "Alice" = Ok "Alice");
  assert (validate_name "" = Error EmptyName);

  (match validate_field "email" "" with
   | Error e -> assert (string_of_detailed_error e = "field 'email': cannot be empty")
   | Ok () -> assert false);

  Printf.printf "validate_age 25: %s\n"
    (match validate_age 25 with Ok n -> string_of_int n | Error e -> string_of_validation_error e)
