(* Parametric Polymorphism — Generic Pair *)
(* Generic types with type parameters *)

type ('a, 'b) either = Left of 'a | Right of 'b

let map_left f = function
  | Left x -> Left (f x)
  | Right y -> Right y

let map_right f = function
  | Left x -> Left x
  | Right y -> Right (f y)

let partition_either lst =
  List.fold_right (fun x (lefts, rights) -> match x with
    | Left l -> (l :: lefts, rights)
    | Right r -> (lefts, r :: rights)
  ) lst ([], [])

let items = [Left 1; Right "a"; Left 2; Right "b"; Left 3]
let (nums, strs) = partition_either items
let () = Printf.printf "Left: %s, Right: %s\n"
  (String.concat "," (List.map string_of_int nums))
  (String.concat "," strs)
