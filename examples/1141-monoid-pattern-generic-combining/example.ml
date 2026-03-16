module type MONOID = sig
  type t
  val empty : t
  val combine : t -> t -> t
end

let concat_all (type a) (module M : MONOID with type t = a) (lst : a list) =
  List.fold_left M.combine M.empty lst

module Sum = struct type t = int let empty = 0 let combine = (+) end
module Product = struct type t = int let empty = 1 let combine = ( * ) end
module Concat = struct type t = string let empty = "" let combine = (^) end
module All = struct type t = bool let empty = true let combine = (&&) end

let () =
  Printf.printf "sum: %d\n" (concat_all (module Sum) [1;2;3;4;5]);
  Printf.printf "product: %d\n" (concat_all (module Product) [1;2;3;4;5]);
  Printf.printf "concat: %s\n" (concat_all (module Concat) ["hello";" ";"world"]);
  Printf.printf "all: %b\n" (concat_all (module All) [true; true; false])

