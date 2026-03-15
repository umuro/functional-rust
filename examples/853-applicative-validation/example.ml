(* Example 054: Applicative Validation *)
(* Accumulate ALL errors instead of short-circuiting on first *)

type ('a, 'e) validated =
  | Valid of 'a
  | Invalid of 'e list

let pure x = Valid x

let map f = function
  | Valid x -> Valid (f x)
  | Invalid es -> Invalid es

let apply vf vx = match vf, vx with
  | Valid f, Valid x -> Valid (f x)
  | Invalid e1, Invalid e2 -> Invalid (e1 @ e2)  (* accumulate! *)
  | Invalid e, _ | _, Invalid e -> Invalid e

let ( <*> ) = apply

(* Approach 1: Validate a user record *)
type user = { name: string; age: int; email: string }

let validate_name s =
  if String.length s > 0 then Valid s
  else Invalid ["Name cannot be empty"]

let validate_age n =
  if n >= 0 && n <= 150 then Valid n
  else Invalid ["Age must be between 0 and 150"]

let validate_email s =
  if String.contains s '@' then Valid s
  else Invalid ["Email must contain @"]

let make_user name age email = { name; age; email }

let validate_user name age email =
  pure make_user <*> validate_name name <*> validate_age age <*> validate_email email

(* Approach 2: Combine with lift *)
let lift2 f a b = pure f <*> a <*> b
let lift3 f a b c = pure f <*> a <*> b <*> c

(* Approach 3: Collect errors from a list of validations *)
let collect_errors validations =
  List.fold_left (fun acc v ->
    match acc, v with
    | Valid xs, Valid x -> Valid (xs @ [x])
    | Invalid e1, Invalid e2 -> Invalid (e1 @ e2)
    | Invalid e, _ | _, Invalid e -> Invalid e
  ) (Valid []) validations

let () =
  (* Valid user *)
  let u = validate_user "Alice" 30 "alice@example.com" in
  (match u with Valid u -> assert (u.name = "Alice") | _ -> assert false);

  (* Single error *)
  let u2 = validate_user "" 30 "alice@example.com" in
  assert (u2 = Invalid ["Name cannot be empty"]);

  (* Multiple errors accumulated *)
  let u3 = validate_user "" (-5) "bad" in
  assert (u3 = Invalid [
    "Name cannot be empty";
    "Age must be between 0 and 150";
    "Email must contain @"
  ]);

  (* collect_errors *)
  let errs = collect_errors [Valid 1; Invalid ["e1"]; Valid 3; Invalid ["e2"]] in
  assert (errs = Invalid ["e1"; "e2"]);

  Printf.printf "✓ All tests passed\n"
