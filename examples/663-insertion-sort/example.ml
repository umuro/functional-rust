(* Insertion Sort in OCaml - Functional *)

let rec insert x = function
  | [] -> [x]
  | h :: t -> if x <= h then x :: h :: t else h :: insert x t

let insertion_sort lst = List.fold_right insert lst []

let () =
  let arr = [12; 11; 13; 5; 6] in
  let sorted = insertion_sort arr in
  Printf.printf "Sorted: [%s]\n"
    (String.concat "; " (List.map string_of_int sorted))
