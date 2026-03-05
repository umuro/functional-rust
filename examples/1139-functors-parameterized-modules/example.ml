(* Functors — Parameterized Modules *)
(* Create modules parameterized by other modules *)

module type COMPARABLE = sig
  type t
  val compare : t -> t -> int
end

module MakeSortedList (C : COMPARABLE) = struct
  type t = C.t list
  let empty = []
  let rec insert x = function
    | [] -> [x]
    | hd :: tl ->
      if C.compare x hd <= 0 then x :: hd :: tl
      else hd :: insert x tl
  let to_list t = t
end

module IntSorted = MakeSortedList(Int)
let s = List.fold_left (fun acc x -> IntSorted.insert x acc) IntSorted.empty [5;3;7;1;4]
let () = List.iter (fun x -> Printf.printf "%d " x) (IntSorted.to_list s)
