(* Merge Sort in OCaml *)

let rec merge = function
  | [], ys -> ys
  | xs, [] -> xs
  | (x :: xs as xxs), (y :: ys as yys) ->
      if x <= y then x :: merge (xs, yys)
      else y :: merge (xxs, ys)

let rec split = function
  | [] -> ([], [])
  | [x] -> ([x], [])
  | x :: y :: rest ->
      let (left, right) = split rest in
      (x :: left, y :: right)

let rec merge_sort = function
  | [] | [_] as lst -> lst
  | lst ->
      let (left, right) = split lst in
      merge (merge_sort left, merge_sort right)

let () =
  let arr = [38; 27; 43; 3; 9; 82; 10] in
  let sorted = merge_sort arr in
  Printf.printf "Sorted: [%s]\n"
    (String.concat "; " (List.map string_of_int sorted))
