(* 054: Applicative Validation
   Collect ALL errors instead of stopping at first failure *)

type validation_error =
  | NameEmpty
  | NameTooLong
  | AgeNegative
  | AgeUnrealistic
  | EmailInvalid

type person = {
  name  : string;
  age   : int;
  email : string;
}

(* --- Approach 1: Individual validators returning error lists --- *)

(* We use (_, error list) result where Ok carries the valid value *)
let validate_name name =
  if String.length name = 0 then Error [NameEmpty]
  else if String.length name > 50 then Error [NameTooLong]
  else Ok name

let validate_age age =
  if age < 0 then Error [AgeNegative]
  else if age > 150 then Error [AgeUnrealistic]
  else Ok age

let validate_email email =
  if not (String.contains email '@') then Error [EmailInvalid]
  else Ok email

(* --- Approach 2: Applicative: collect all errors instead of failing fast --- *)

(* Helper: combine two validation results, accumulating all errors *)
let ( <+> ) r1 r2 =
  match r1, r2 with
  | Ok _, Ok _       -> Ok ()      (* both good; values assembled separately *)
  | Ok _, Error e    -> Error e
  | Error e, Ok _    -> Error e
  | Error e1, Error e2 -> Error (e1 @ e2)   (* KEY: accumulate both error lists *)

let validate_person name age email =
  let nr = validate_name name in
  let ar = validate_age age in
  let er = validate_email email in
  (* check all three together, accumulating errors *)
  match nr <+> ar <+> er with
  | Error errs -> Error errs
  | Ok _ ->
    (* safe unwraps: we know all succeeded *)
    Ok {
      name  = Result.get_ok nr;
      age   = Result.get_ok ar;
      email = Result.get_ok er;
    }

(* --- Approach 3: Generic applicative combinator over a list of validators --- *)

let validate_all validators value =
  List.fold_left (fun acc v ->
    match acc, v value with
    | Error es, Error e -> Error (es @ e)
    | Error es, Ok _    -> Error es
    | Ok _, Error e     -> Error e
    | Ok _, Ok x        -> Ok x
  ) (Ok value) validators

let () =
  (* valid *)
  (match validate_person "Alice" 30 "alice@example.com" with
   | Ok p -> Printf.printf "Valid: %s, age %d\n" p.name p.age
   | Error _ -> print_endline "Unexpected error");

  (* all errors collected *)
  (match validate_person "" (-5) "bad" with
   | Error errs -> Printf.printf "Errors collected: %d\n" (List.length errs)
   | Ok _ -> print_endline "Unexpected ok");

  (* partial error *)
  (match validate_person "Bob" 25 "bad" with
   | Error errs -> Printf.printf "Partial errors: %d\n" (List.length errs)
   | Ok _ -> print_endline "Unexpected ok")
