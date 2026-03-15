(* 054: Applicative Validation *)
(* Collect all validation errors instead of stopping at the first *)

type 'a validated = Valid of 'a | Invalid of string list

(* Approach 1: Individual validators *)
let validate_name name =
  if String.length name = 0 then Invalid ["Name cannot be empty"]
  else if String.length name > 50 then Invalid ["Name too long"]
  else Valid name

let validate_age age =
  if age < 0 then Invalid ["Age cannot be negative"]
  else if age > 150 then Invalid ["Age unrealistic"]
  else Valid age

let validate_email email =
  if not (String.contains email '@') then Invalid ["Email must contain @"]
  else Valid email

(* Approach 2: Combine validators — applicative style *)
let map_validated f = function
  | Valid x -> Valid (f x)
  | Invalid errs -> Invalid errs

let apply_validated fv xv =
  match fv, xv with
  | Valid f, Valid x -> Valid (f x)
  | Valid _, Invalid errs -> Invalid errs
  | Invalid errs, Valid _ -> Invalid errs
  | Invalid e1, Invalid e2 -> Invalid (e1 @ e2)

type person = { name: string; age: int; email: string }

let validate_person name age email =
  let mk_person n a e = { name = n; age = a; email = e } in
  Valid mk_person
  |> fun fv -> apply_validated fv (validate_name name)
  |> fun fv -> apply_validated fv (validate_age age)
  |> fun fv -> apply_validated fv (validate_email email)

(* Tests *)
let () =
  (match validate_person "Alice" 30 "alice@example.com" with
   | Valid p -> assert (p.name = "Alice" && p.age = 30)
   | Invalid _ -> assert false);
  (match validate_person "" (-5) "bad" with
   | Valid _ -> assert false
   | Invalid errs -> assert (List.length errs = 3));
  (match validate_person "Bob" 25 "bad" with
   | Valid _ -> assert false
   | Invalid errs -> assert (List.length errs = 1));
  Printf.printf "✓ All tests passed\n"
