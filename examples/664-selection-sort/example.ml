(* Selection Sort in OCaml *)

let rec find_min = function
  | [] -> failwith "empty"
  | [x] -> (x, [])
  | x :: xs ->
      let (min, rest) = find_min xs in
      if x < min then (x, min :: rest)
      else (min, x :: rest)

let rec selection_sort = function
  | [] -> []
  | lst ->
      let (min, rest) = find_min lst in
      min :: selection_sort rest

let () =
  let arr = [64; 25; 12; 22; 11] in
  let sorted = selection_sort arr in
  Printf.printf "Sorted: [%s]\n"
    (String.concat "; " (List.map string_of_int sorted))
