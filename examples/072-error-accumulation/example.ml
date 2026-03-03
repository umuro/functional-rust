(* Error accumulation: collect ALL errors, not just the first.
   Unlike Result which short-circuits, this gathers a list of errors. *)

type ('a, 'e) validation =
  | Ok of 'a
  | Errors of 'e list

let return x = Ok x

let map f = function
  | Ok x     -> Ok (f x)
  | Errors es -> Errors es

(* Apply: combine errors from both sides *)
let apply f_v x_v = match f_v, x_v with
  | Ok f, Ok x         -> Ok (f x)
  | Ok _, Errors es    -> Errors es
  | Errors es, Ok _    -> Errors es
  | Errors e1, Errors e2 -> Errors (e1 @ e2)

let (<*>) = apply

(* Validate a name (non-empty) *)
let validate_name s =
  if String.length s > 0 then Ok s
  else Errors ["name cannot be empty"]

(* Validate an age (18-120) *)
let validate_age n =
  if n >= 18 && n <= 120 then Ok n
  else Errors [Printf.sprintf "age %d out of range (18-120)" n]

(* Validate email (has @) *)
let validate_email s =
  if String.contains s '@' then Ok s
  else Errors ["email must contain @"]

let () =
  (* All valid *)
  let r1 = return (fun n a e -> (n, a, e))
           <*> validate_name "Alice"
           <*> validate_age 30
           <*> validate_email "alice@example.com"
  in
  (match r1 with
   | Ok (n, a, e) -> Printf.printf "Valid: %s, %d, %s\n" n a e
   | Errors _ -> assert false);

  (* Multiple errors accumulate *)
  let r2 = return (fun n a e -> (n, a, e))
           <*> validate_name ""
           <*> validate_age 15
           <*> validate_email "bad-email"
  in
  (match r2 with
   | Ok _ -> assert false
   | Errors es ->
     Printf.printf "Errors (%d):\n" (List.length es);
     List.iter (Printf.printf "  - %s\n") es)
