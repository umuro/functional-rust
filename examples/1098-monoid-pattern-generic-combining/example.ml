(* Example 1098: Monoid Pattern — Generic Combining *)
(* A monoid has an identity element (empty) and an associative binary operation (combine). *)

module type MONOID = sig
  type t
  val empty : t
  val combine : t -> t -> t
end

(* concat_all: fold a list using any monoid — first-class module as argument *)
let concat_all (type a) (module M : MONOID with type t = a) (lst : a list) =
  List.fold_left M.combine M.empty lst

(* Concrete monoid modules *)
module Sum = struct type t = int let empty = 0 let combine = (+) end
module Product = struct type t = int let empty = 1 let combine = ( * ) end
module Concat = struct type t = string let empty = "" let combine = (^) end
module All = struct type t = bool let empty = true let combine = (&&) end
module Any = struct type t = bool let empty = false let combine = (||) end

(* Tests *)
let () =
  (* Sum monoid *)
  assert (concat_all (module Sum) [1;2;3;4;5] = 15);
  assert (concat_all (module Sum) [] = 0);
  assert (concat_all (module Sum) [42] = 42);

  (* Product monoid *)
  assert (concat_all (module Product) [1;2;3;4;5] = 120);
  assert (concat_all (module Product) [] = 1);
  assert (concat_all (module Product) [5;0;3] = 0);

  (* Concat monoid *)
  assert (concat_all (module Concat) ["hello";" ";"world"] = "hello world");
  assert (concat_all (module Concat) [] = "");

  (* All (boolean AND) monoid *)
  assert (concat_all (module All) [true; true; true] = true);
  assert (concat_all (module All) [true; true; false] = false);
  assert (concat_all (module All) [] = true);

  (* Any (boolean OR) monoid *)
  assert (concat_all (module Any) [false; false] = false);
  assert (concat_all (module Any) [false; true; false] = true);
  assert (concat_all (module Any) [] = false);

  print_endline "ok"
