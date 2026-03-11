(* 1109: Monoid Pattern — Generic Combining
   OCaml uses a module type (MONOID) as a typeclass.
   Modules satisfying the signature are passed first-class to functions. *)

module type MONOID = sig
  type t
  val empty : t
  val combine : t -> t -> t
end

(* concat_all is polymorphic over any MONOID module.
   The module is passed explicitly as a first-class value. *)
let concat_all (type a) (module M : MONOID with type t = a) (lst : a list) =
  List.fold_left M.combine M.empty lst

(* Recursive version — explicit structural recursion *)
let rec concat_all_rec (type a) (module M : MONOID with type t = a) = function
  | []      -> M.empty
  | [x]     -> x
  | x :: xs -> M.combine x (concat_all_rec (module M) xs)

(* Four module instances — same module type, different semantics *)
module Sum = struct
  type t = int
  let empty = 0
  let combine = ( + )
end

module Product = struct
  type t = int
  let empty = 1
  let combine = ( * )
end

module Concat = struct
  type t = string
  let empty = ""
  let combine = ( ^ )
end

module All = struct
  type t = bool
  let empty = true
  let combine = ( && )
end

let () =
  (* Iterative via fold *)
  assert (concat_all (module Sum)     [1;2;3;4;5] = 15);
  assert (concat_all (module Product) [1;2;3;4;5] = 120);
  assert (concat_all (module Concat)  ["hello";" ";"world"] = "hello world");
  assert (concat_all (module All)     [true; true; false] = false);
  (* Identity elements *)
  assert (concat_all (module Sum)     [] = 0);
  assert (concat_all (module Product) [] = 1);
  assert (concat_all (module All)     [] = true);
  (* Recursive version agrees *)
  assert (concat_all_rec (module Sum) [1;2;3;4;5] = 15);
  Printf.printf "sum: %d\n"     (concat_all (module Sum)     [1;2;3;4;5]);
  Printf.printf "product: %d\n" (concat_all (module Product) [1;2;3;4;5]);
  Printf.printf "concat: %s\n"  (concat_all (module Concat)  ["hello";" ";"world"]);
  Printf.printf "all: %b\n"     (concat_all (module All)     [true; true; false]);
  print_endline "ok"
