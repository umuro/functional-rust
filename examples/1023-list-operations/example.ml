(* List Operations *)
(* Implement map, filter, fold, reverse, append, concat from scratch *)

let length l =
  let rec go acc = function [] -> acc | _ :: t -> go (acc + 1) t in
  go 0 l

let reverse l =
  let rec go acc = function [] -> acc | h :: t -> go (h :: acc) t in
  go [] l

let map ~f l =
  let rec go acc = function
    | [] -> acc | h :: t -> go (f h :: acc) t in
  go [] l |> reverse

let filter ~f l =
  let rec go acc = function
    | [] -> acc
    | h :: t when f h -> go (h :: acc) t
    | _ :: t -> go acc t in
  go [] l |> reverse

let rec fold ~init:acc ~f = function
  | [] -> acc | h :: t -> fold ~init:(f acc h) ~f t

let append a b = List.fold_right (fun x acc -> x :: acc) a b

let concat ll = List.fold_right append ll []
