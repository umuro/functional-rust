(* Quick Sort in OCaml *)

let rec quick_sort = function
  | [] -> []
  | pivot :: rest ->
      let left = List.filter (fun x -> x < pivot) rest in
      let right = List.filter (fun x -> x >= pivot) rest in
      quick_sort left @ [pivot] @ quick_sort right

let () =
  let arr = [10; 7; 8; 9; 1; 5] in
  let sorted = quick_sort arr in
  Printf.printf "Sorted: [%s]\n"
    (String.concat "; " (List.map string_of_int sorted))
