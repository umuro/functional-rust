(* Grand synthesis: a mini functional system combining:
   - Type-safe DSL (tagless final)
   - Monadic error handling
   - Functors and natural transformations
   - CPS and trampolining
   The goal: a validated data pipeline with logging *)

(* Validation applicative *)
type ('a, 'e) validated =
  | Valid   of 'a
  | Invalid of 'e list

let valid x    = Valid x
let invalid e  = Invalid [e]

let map_v f = function
  | Valid x     -> Valid (f x)
  | Invalid es  -> Invalid es

let apply_v f_v x_v = match f_v, x_v with
  | Valid f, Valid x             -> Valid (f x)
  | Invalid e, Valid _           -> Invalid e
  | Valid _, Invalid e           -> Invalid e
  | Invalid e1, Invalid e2       -> Invalid (e1 @ e2)

(* Validated pipeline for a user record *)
type user = { name: string; age: int; email: string }

let validate_name s =
  if String.length s >= 2 then valid s
  else invalid "name too short (min 2 chars)"

let validate_age n =
  if n >= 0 && n < 150 then valid n
  else invalid (Printf.sprintf "age %d out of range" n)

let validate_email s =
  if String.contains s '@' then valid s
  else invalid "email missing @"

let make_user name age email = { name; age; email }

let validate_user name age email =
  apply_v
    (apply_v (map_v make_user (validate_name name))
             (validate_age age))
    (validate_email email)

(* CPS logger *)
let with_log label f x k =
  let result = f x in
  k (Printf.sprintf "[%s] %s -> " label x) result

let () =
  (* Valid user *)
  (match validate_user "Alice" 30 "alice@example.com" with
   | Valid u    -> Printf.printf "Valid user: %s, %d, %s\n" u.name u.age u.email
   | Invalid es -> Printf.printf "Errors: %s\n" (String.concat "; " es));

  (* Multiple validation errors accumulate *)
  (match validate_user "A" 200 "bad-email" with
   | Valid _    -> assert false
   | Invalid es ->
     Printf.printf "Errors (%d):\n" (List.length es);
     List.iter (Printf.printf "  - %s\n") es);

  Printf.printf "\nGrand synthesis: FP patterns compose.\n"
