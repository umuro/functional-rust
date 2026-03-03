(* Difference List — O(1) Append *)

(* A difference list is a function from list to list *)
type 'a dlist = 'a list -> 'a list

let empty : 'a dlist = Fun.id
let singleton x : 'a dlist = fun rest -> x :: rest
let append (a : 'a dlist) (b : 'a dlist) : 'a dlist = fun rest -> a (b rest)
let of_list lst : 'a dlist = fun rest -> lst @ rest
let to_list (dl : 'a dlist) = dl []

let () =
  let a = of_list [1;2;3] in
  let b = of_list [4;5;6] in
  let c = singleton 7 in
  let result = append (append a b) c |> to_list in
  assert (result = [1;2;3;4;5;6;7])
