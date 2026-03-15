(* 1003: Custom Error Types *)
(* OCaml exception vs Rust enum-based errors *)

(* Approach 1: Built-in exceptions *)
exception Invalid_age of string
exception Parse_error of string

let validate_age_exn age =
  if age < 0 then raise (Invalid_age "age cannot be negative")
  else if age > 150 then raise (Invalid_age "age unreasonably large")
  else age

let test_approach1 () =
  assert (validate_age_exn 25 = 25);
  (try
     let _ = validate_age_exn (-1) in
     assert false
   with Invalid_age msg -> assert (msg = "age cannot be negative"));
  (try
     let _ = validate_age_exn 200 in
     assert false
   with Invalid_age msg -> assert (msg = "age unreasonably large"));
  Printf.printf "  Approach 1 (exceptions): passed\n"

(* Approach 2: Result type with custom error variant *)
type validation_error =
  | NegativeAge of int
  | UnreasonableAge of int
  | EmptyName

let string_of_validation_error = function
  | NegativeAge n -> Printf.sprintf "negative age: %d" n
  | UnreasonableAge n -> Printf.sprintf "unreasonable age: %d" n
  | EmptyName -> "name cannot be empty"

let validate_age age =
  if age < 0 then Error (NegativeAge age)
  else if age > 150 then Error (UnreasonableAge age)
  else Ok age

let validate_name name =
  if String.length name = 0 then Error EmptyName
  else Ok name

let test_approach2 () =
  assert (validate_age 25 = Ok 25);
  assert (validate_age (-5) = Error (NegativeAge (-5)));
  assert (validate_age 200 = Error (UnreasonableAge 200));
  assert (validate_name "Alice" = Ok "Alice");
  assert (validate_name "" = Error EmptyName);
  (match validate_age (-1) with
   | Error e -> assert (string_of_validation_error e = "negative age: -1")
   | Ok _ -> assert false);
  Printf.printf "  Approach 2 (result type): passed\n"

(* Approach 3: Polymorphic variants for lightweight errors *)
let validate_age_poly age =
  if age < 0 then Error (`NegativeAge age)
  else if age > 150 then Error (`UnreasonableAge age)
  else Ok age

let test_approach3 () =
  assert (validate_age_poly 30 = Ok 30);
  (match validate_age_poly (-1) with
   | Error (`NegativeAge n) -> assert (n = -1)
   | _ -> assert false);
  Printf.printf "  Approach 3 (polymorphic variants): passed\n"

let () =
  Printf.printf "Testing custom error types:\n";
  test_approach1 ();
  test_approach2 ();
  test_approach3 ();
  Printf.printf "✓ All tests passed\n"
