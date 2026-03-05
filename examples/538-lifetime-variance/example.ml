(* Variance in OCaml — handled by type system automatically *)
(* Covariant type parameter: *)
type 'a box_ = Box of 'a  (* covariant in 'a — safe to use Child where Parent expected *)

(* Contravariant: function argument *)
(* If Animal :> Dog, then (Dog -> unit) :> (Animal -> unit) in OCaml subtyping *)

(* Invariant: mutable references *)
(* ('a ref) is invariant — can't substitute *)

let () =
  (* Covariant example: option is covariant *)
  let x : int option = Some 42 in
  let _ = x in
  Printf.printf "Variance concept: covariant containers are safe for reading\n";
  Printf.printf "Invariant containers (mutable refs) must match exactly\n";
  Printf.printf "Contravariant: functions that ACCEPT base types work where derived expected\n"
