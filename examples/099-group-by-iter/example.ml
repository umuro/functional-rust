(* 099: Group Consecutive Equal Elements *)

let group_by lst =
  match lst with
  | [] -> []
  | x :: xs ->
    let rec aux current group acc = function
      | [] -> List.rev (List.rev (current :: group) :: acc)
      | h :: t ->
        if h = current then aux current (h :: group) acc t
        else aux h [h] (List.rev (current :: group) :: acc) t
    in
    aux x [] [] xs

(* Tests *)
let () =
  assert (group_by [1;1;2;2;2;3;1;1] = [[1;1];[2;2;2];[3];[1;1]]);
  assert (group_by [] = []);
  assert (group_by [1] = [[1]]);
  Printf.printf "✓ All tests passed\n"
