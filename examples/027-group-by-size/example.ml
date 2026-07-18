(* Group By Size *)
(* OCaml 99 Problems #27 *)

let rec combinations k lst =
  match k, lst with
  | 0, _ -> [[]]
  | _, [] -> []
  | k, x :: xs -> List.map (fun c -> x :: c) (combinations (k - 1) xs) @ combinations k xs

let rec group lst sizes =
  match sizes with
  | [] -> [[]]
  | k :: rest ->
    List.concat_map
      (fun combo ->
         let remaining = List.filter (fun x -> not (List.mem x combo)) lst in
         List.map (fun groups -> combo :: groups) (group remaining rest))
      (combinations k lst)

(* Tests *)
let () =
  assert (List.length (group [1; 2; 3] [1; 2]) = 3);

  let groupings = group [1; 2; 3; 4] [2; 2] in
  List.iter
    (fun grouping ->
       assert (List.length grouping = 2);
       assert (List.sort compare (List.concat grouping) = [1; 2; 3; 4]))
    groupings;

  let list9 = List.init 9 (fun i -> i + 1) in
  assert (List.length (group list9 [2; 3; 4]) = 1260);

  assert (group [1; 2; 3] [] = [[]]);

  print_endline "✓ OCaml tests passed"
