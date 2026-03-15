(* 099: Group Consecutive Equal Elements
   In OCaml we implement group_by using recursive pattern matching.
   Consecutive equal elements are collected into inner lists. *)

(* Group consecutive equal elements — pure recursive approach *)
let group_by lst =
  let rec aux acc cur rest =
    match rest with
    | [] -> List.rev (List.rev cur :: acc)
    | x :: xs ->
      (match cur with
       | [] -> aux acc [x] xs
       | h :: _ when h = x -> aux acc (x :: cur) xs
       | _ -> aux (List.rev cur :: acc) [x] xs)
  in
  match lst with
  | [] -> []
  | _ -> aux [] [] lst

let () =
  assert (group_by [1; 1; 2; 2; 2; 3; 1; 1] =
          [[1; 1]; [2; 2; 2]; [3]; [1; 1]]);
  assert (group_by ([] : int list) = []);
  assert (group_by [1] = [[1]]);
  assert (group_by [5; 5; 5] = [[5; 5; 5]]);

  (* Print a group *)
  let groups = group_by [1; 1; 2; 2; 2; 3; 1; 1] in
  List.iteri (fun i g ->
    Printf.printf "group %d: [%s]\n" i
      (String.concat "; " (List.map string_of_int g))
  ) groups
