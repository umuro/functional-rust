(* 354: Binary Heap / Priority Queue
   OCaml stdlib has no built-in heap, but we can implement a simple
   max-heap, or use a sorted module. Here we implement a functional
   leftist heap for pure-functional priority queues. *)

(* Leftist min-heap (negate values for max-heap behaviour) *)
type 'a heap =
  | Empty
  | Node of int * 'a * 'a heap * 'a heap  (* rank, value, left, right *)

let rank = function Empty -> 0 | Node (r,_,_,_) -> r

let make_node v l r =
  (* Leftist property: left rank >= right rank *)
  if rank l >= rank r
  then Node (rank r + 1, v, l, r)
  else Node (rank l + 1, v, r, l)

let rec merge h1 h2 =
  match h1, h2 with
  | Empty, h | h, Empty -> h
  | Node (_,v1,l1,r1), Node (_,v2,_,_) ->
    if v1 <= v2
    then make_node v1 l1 (merge r1 h2)
    else merge h2 h1

let push heap v = merge heap (Node (1, v, Empty, Empty))

let pop = function
  | Empty -> None
  | Node (_,v,l,r) -> Some (v, merge l r)

(* Heap from list *)
let of_list items = List.fold_left push Empty items

(* Top N smallest elements *)
let bottom_n items n =
  let h = of_list items in
  let rec go h acc k =
    if k = 0 then List.rev acc
    else match pop h with
      | None        -> List.rev acc
      | Some (v, h) -> go h (v::acc) (k-1)
  in
  go h [] n

(* Top N largest: negate, take bottom N, negate back *)
let top_n items n =
  let neg_items = List.map (~-) items in
  bottom_n neg_items n |> List.map (~-)

(* Heap sort (ascending) *)
let heap_sort items =
  let h = of_list items in
  let rec drain h acc =
    match pop h with
    | None        -> List.rev acc
    | Some (v, h) -> drain h (v::acc)
  in
  drain h []

let () =
  let data = [3;1;4;1;5;9;2;6] in

  let top = top_n data 3 in
  assert (top = [9;6;5]);
  Printf.printf "top 3: %s\n%!"
    (top |> List.map string_of_int |> String.concat ", ");

  let bottom = bottom_n data 3 in
  assert (bottom = [1;1;2]);
  Printf.printf "bottom 3: %s\n%!"
    (bottom |> List.map string_of_int |> String.concat ", ");

  let sorted = heap_sort [3;1;4;1;5] in
  assert (sorted = [1;1;3;4;5]);
  Printf.printf "heap sort: %s\n%!"
    (sorted |> List.map string_of_int |> String.concat ", ")
