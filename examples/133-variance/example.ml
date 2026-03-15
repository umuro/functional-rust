(* Example 133: Variance — Covariance, Contravariance, Invariance *)

(* Approach 1: Covariance in OCaml *)
(* OCaml infers variance for type parameters *)
type +'a producer = { produce : unit -> 'a }
type -'a consumer = { consume : 'a -> unit }
type 'a invariant_ref = { mutable contents : 'a }

let int_producer : int producer = { produce = fun () -> 42 }

(* Covariance: if int is a subtype via polymorphism, producer is covariant *)
(* OCaml uses structural subtyping with objects/polymorphic variants *)

(* Approach 2: Polymorphic variants show variance *)
type base = [ `A | `B ]
type extended = [ `A | `B | `C ]

(* extended is a subtype of base for covariant positions *)
let use_base (x : base) = match x with `A -> "a" | `B -> "b"
let extended_val : extended = `C

(* Approach 3: Functor variance *)
module type COVARIANT = sig
  type +'a t
  val map : ('a -> 'b) -> 'a t -> 'b t
end

module ListCov : COVARIANT with type 'a t = 'a list = struct
  type 'a t = 'a list
  let map = List.map
end

module type CONTRAVARIANT = sig
  type -'a t
  val contramap : ('b -> 'a) -> 'a t -> 'b t
end

module Predicate : CONTRAVARIANT = struct
  type 'a t = 'a -> bool
  let contramap f pred = fun x -> pred (f x)
end

(* Tests *)
let () =
  assert (int_producer.produce () = 42);
  assert (use_base `A = "a");
  let doubled = ListCov.map (fun x -> x * 2) [1; 2; 3] in
  assert (doubled = [2; 4; 6]);
  let is_positive = fun x -> x > 0 in
  let string_len_positive = Predicate.contramap String.length is_positive in
  assert (string_len_positive "hello" = true);
  assert (string_len_positive "" = false);
  Printf.printf "✓ All tests passed\n"
