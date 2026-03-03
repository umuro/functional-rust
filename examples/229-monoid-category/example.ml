(* A monoid viewed as a single-object category:
   - One object: ()
   - Morphisms: the monoid elements
   - Composition: monoid append (associative)
   - Identity: monoid empty *)

module type MONOID = sig
  type t
  val empty  : t
  val append : t -> t -> t
end

(* Verify monoid laws *)
module MonoidLaws (M : MONOID) = struct
  let check_identity x =
    M.append M.empty x = x &&
    M.append x M.empty = x

  let check_associativity x y z =
    M.append (M.append x y) z = M.append x (M.append y z)
end

(* String monoid *)
module StringMonoid : MONOID with type t = string = struct
  type t = string
  let empty  = ""
  let append = ( ^ )
end

(* List monoid *)
module ListMonoid : MONOID with type t = int list = struct
  type t = int list
  let empty  = []
  let append = ( @ )
end

(* Sum monoid *)
module SumMonoid : MONOID with type t = int = struct
  type t = int
  let empty  = 0
  let append = ( + )
end

let () =
  let module SL = MonoidLaws (StringMonoid) in
  let module LL = MonoidLaws (ListMonoid) in
  let module NL = MonoidLaws (SumMonoid) in

  assert (SL.check_identity "hello");
  assert (SL.check_associativity "a" "b" "c");
  assert (LL.check_identity [1;2;3]);
  assert (NL.check_identity 42);

  Printf.printf "All monoid laws verified\n";

  (* Monoid as category: compose morphisms *)
  let compose_morphisms ms =
    List.fold_left StringMonoid.append StringMonoid.empty ms
  in
  Printf.printf "Composed: %s\n" (compose_morphisms ["hello"; ", "; "world"; "!"])
