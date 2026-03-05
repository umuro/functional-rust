(* Paramorphism in OCaml *)
(* Para on lists: f receives (head, tail, folded_tail) *)

let para_list nil cons xs =
  let rec go = function
    | []      -> nil
    | x :: xs -> cons x xs (go xs)
  in go xs

(* Insert into sorted list *)
let insert_sorted x xs =
  para_list
    [x]
    (fun h t folded_t -> if x <= h then x :: h :: t else h :: folded_t)
    xs

let insertion_sort xs =
  List.fold_left (fun acc x -> insert_sorted x acc) [] xs

(* Get suffix context *)
let tails xs = para_list [[]] (fun x _ rest -> (x :: List.hd rest) :: rest) xs

let () =
  Printf.printf "insert 3 into [1;2;4;5]: %s\n"
    (String.concat "," (List.map string_of_int (insert_sorted 3 [1;2;4;5])));
  Printf.printf "insertion_sort [3;1;4;1;5]: %s\n"
    (String.concat "," (List.map string_of_int (insertion_sort [3;1;4;1;5])));
  Printf.printf "tails [1;2;3]: %d sublists\n" (List.length (tails [1;2;3]))
