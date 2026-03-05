(* Functor laws verification in OCaml *)
module type FUNCTOR = sig
  type 'a t
  val map : ('a -> 'b) -> 'a t -> 'b t
end

module OptionFunctor : FUNCTOR with type 'a t = 'a option = struct
  type 'a t = 'a option
  let map = Option.map
end

module ListFunctor : FUNCTOR with type 'a t = 'a list = struct
  type 'a t = 'a list
  let map = List.map
end

(* Law verification *)
let identity_law fa =
  List.map Fun.id fa = fa

let composition_law f g fa =
  List.map (fun x -> g (f x)) fa = List.map g (List.map f fa)

let () =
  let xs = [1;2;3;4;5] in
  Printf.printf "identity law: %b\n" (identity_law xs);
  let f x = x * 2 in
  let g x = x + 1 in
  Printf.printf "composition law: %b\n" (composition_law f g xs)
