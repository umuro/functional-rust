(* Curry-Howard: propositions are types, proofs are programs.
   A -> B corresponds to a function a -> b.
   A /\ B corresponds to a pair (a, b).
   A \/ B corresponds to a sum type (Left a | Right b). *)

(* Conjunction = product *)
type ('a, 'b) conj = And of 'a * 'b

let and_intro a b = And (a, b)
let and_elim_left  (And (a, _)) = a
let and_elim_right (And (_, b)) = b

(* Disjunction = sum *)
type ('a, 'b) disj = Left of 'a | Right of 'b

let or_intro_left  a = Left a
let or_intro_right b = Right b
let or_elim f g = function Left a -> f a | Right b -> g b

(* Implication = function *)
let modus_ponens f a = f a  (* A -> B, A |- B *)

(* Negation = A -> False (using unit as False placeholder) *)
type 'a neg = 'a -> unit

(* Double negation intro *)
let dne : 'a -> 'a neg neg = fun a k -> k a

let () =
  (* Proof: A /\ B -> B /\ A (and is commutative) *)
  let and_comm (And (a, b)) = And (b, a) in
  let proof = and_comm (And (1, "hello")) in
  Printf.printf "and_comm: (%s, %d)\n" (and_elim_left proof) (and_elim_right proof);

  (* Proof: A -> A (identity) *)
  let id_proof : 'a -> 'a = modus_ponens (fun x -> x) in
  Printf.printf "identity proof: %d\n" (id_proof 42);

  Printf.printf "Curry-Howard: types are propositions, functions are proofs\n"
