(* Monoid Laws in OCaml *)

module type MONOID = sig
  type t
  val empty : t
  val combine : t -> t -> t
end

module SumMonoid : MONOID with type t = int = struct
  type t = int
  let empty = 0
  let combine = ( + )
end

module StringMonoid : MONOID with type t = string = struct
  type t = string
  let empty = ""
  let combine = ( ^ )
end

let mconcat (type a) (module M : MONOID with type t = a) items =
  List.fold_left M.combine M.empty items

let () =
  let open SumMonoid in
  let x = 42 in
  Printf.printf "Left identity: %b\n" (combine empty x = x);
  Printf.printf "Right identity: %b\n" (combine x empty = x);
  
  let result = mconcat (module SumMonoid) [1; 2; 3; 4] in
  Printf.printf "mconcat [1;2;3;4] = %d\n" result
