(* Propositions as types (PAT), also called BHK interpretation:
   - Proposition = Type
   - Proof = inhabitant (value)
   - Implication A -> B = function type
   - Conjunction A and B = product type A * B
   - Disjunction A or B = sum type A + B
   - True = unit, False = empty type *)

(* True: always provable — unit *)
let proof_of_true : unit = ()

(* Implication: A -> B — a function *)
let impl_proof : int -> string = string_of_int

(* Conjunction: A and B — a pair *)
let conj_proof : int * string = (42, "hello")
let elim_left  (a, _) = a
let elim_right (_, b) = b

(* Disjunction: A or B — Either *)
type ('a, 'b) either = Left of 'a | Right of 'b
let disj_left  a = Left  a
let disj_right b = Right b
let case f g = function Left a -> f a | Right b -> g b

(* Universal: forall a. P a — polymorphic function *)
let id_proof : 'a -> 'a = fun x -> x

(* De Morgan: not (A or B) -> not A and not B *)
let de_morgan f =
  (fun a -> f (Left a), fun b -> f (Right b))

let () =
  Printf.printf "True proved: %s\n" (if proof_of_true = () then "yes" else "no");
  Printf.printf "impl: %s\n" (impl_proof 42);
  Printf.printf "conj left: %d\n" (elim_left conj_proof);
  Printf.printf "disj case: %s\n" (case string_of_int (fun s -> s) (Left 42));
  Printf.printf "PAT: proofs are programs, propositions are types\n"
