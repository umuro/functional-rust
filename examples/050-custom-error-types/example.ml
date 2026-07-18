(* Custom Error Types *)
(* OCaml 99 Problems #50 *)

(* OCaml has no Error trait; errors are plain variant types with a to_string helper. *)
type validation_error = NegativeAge of int | UnreasonableAge of int | EmptyName

let error_to_string = function
  | NegativeAge n -> Printf.sprintf "negative age: %d" n
  | UnreasonableAge n -> Printf.sprintf "unreasonable age: %d" n
  | EmptyName -> "name cannot be empty"

let validate_age age =
  if age < 0 then Error (NegativeAge age)
  else if age > 150 then Error (UnreasonableAge age)
  else Ok age

let validate_name name = if name = "" then Error EmptyName else Ok name

(* Tests *)
let () =
  assert (validate_age 30 = Ok 30);
  assert (validate_age (-1) = Error (NegativeAge (-1)));
  assert (validate_age 200 = Error (UnreasonableAge 200));
  assert (validate_name "Ada" = Ok "Ada");
  assert (validate_name "" = Error EmptyName);
  assert (error_to_string (NegativeAge (-5)) = "negative age: -5");
  assert (error_to_string (UnreasonableAge 200) = "unreasonable age: 200");
  assert (error_to_string EmptyName = "name cannot be empty");
  print_endline "✓ OCaml tests passed"
