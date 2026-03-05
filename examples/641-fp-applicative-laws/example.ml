(* Applicative Functor Laws in OCaml *)

module type APPLICATIVE = sig
  type 'a t
  val pure : 'a -> 'a t
  val ap : ('a -> 'b) t -> 'a t -> 'b t
end

(* Option as Applicative *)
module OptionApplicative : APPLICATIVE with type 'a t = 'a option = struct
  type 'a t = 'a option
  
  let pure x = Some x
  
  let ap f_opt x_opt =
    match f_opt, x_opt with
    | Some f, Some x -> Some (f x)
    | _ -> None
end

(* Verify laws *)
let () =
  let open OptionApplicative in
  
  (* Identity: pure id <*> v = v *)
  let v = Some 42 in
  let id_law = ap (pure Fun.id) v = v in
  Printf.printf "Identity law: %b\n" id_law;
  
  (* Homomorphism: pure f <*> pure x = pure (f x) *)
  let f = fun x -> x * 2 in
  let x = 21 in
  let homo_law = ap (pure f) (pure x) = pure (f x) in
  Printf.printf "Homomorphism law: %b\n" homo_law
