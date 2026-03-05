(* Fragment specifiers concept in OCaml — ppx extensions *)
(* We show what each fragment would capture conceptually *)

(* expr — any expression *)
let eval_twice expr = expr + expr  (* conceptually *)
(* ty — type annotation *)
(* ident — identifier, e.g., field name *)
(* pat — pattern, e.g., Some(x) *)

(* Simulate: generate getters using ident-like patterns *)
type person = {
  name: string;
  age: int;
  email: string;
}

let get_name p = p.name
let get_age p = p.age
let get_email p = p.email

let () =
  let p = {name="Alice"; age=30; email="alice@example.com"} in
  Printf.printf "name=%s age=%d email=%s\n"
    (get_name p) (get_age p) (get_email p)
