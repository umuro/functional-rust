(* Custom Set (Functor) *)
(* Functor-based set implementation with sorted lists *)

module type ELEMENT = sig
  type t
  val compare : t -> t -> int
end

module Make (El : ELEMENT) = struct
  type t = El.t list

  let is_empty = function [] -> true | _ -> false
  let is_member l n = List.exists (fun x -> El.compare x n = 0) l
  let of_list = List.sort_uniq El.compare
  let add l x = of_list (x :: l)
  let equal a b = of_list a = of_list b

  let is_subset x y = List.for_all (fun e -> is_member y e) x
  let is_disjoint x y = not (List.exists (fun e -> is_member y e) x)

  let union a b = of_list (a @ b)
  let intersect a b = List.filter (fun e -> is_member b e) a |> of_list
  let difference a b = List.filter (fun e -> not (is_member b e)) a |> of_list
end
