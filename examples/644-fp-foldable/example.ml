(* Foldable in OCaml *)

module type FOLDABLE = sig
  type 'a t
  val fold_left : ('b -> 'a -> 'b) -> 'b -> 'a t -> 'b
  val fold_right : ('a -> 'b -> 'b) -> 'a t -> 'b -> 'b
end

module ListFoldable : FOLDABLE with type 'a t = 'a list = struct
  type 'a t = 'a list
  let fold_left = List.fold_left
  let fold_right f lst init = List.fold_right f lst init
end

module OptionFoldable : FOLDABLE with type 'a t = 'a option = struct
  type 'a t = 'a option
  let fold_left f init = function
    | None -> init
    | Some x -> f init x
  let fold_right f opt init = match opt with
    | None -> init
    | Some x -> f x init
end

let sum (type a) (module F : FOLDABLE with type 'x t = a) xs =
  F.fold_left ( + ) 0 xs

let () =
  let nums = [1; 2; 3; 4; 5] in
  Printf.printf "Sum: %d\n" (List.fold_left ( + ) 0 nums)
