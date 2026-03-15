(* Example 099: Group By Iterator *)
(* Group consecutive equal elements without external crates *)

(* Approach 1: Group consecutive equal elements *)
let group_consecutive lst =
  match lst with
  | [] -> []
  | first :: rest ->
    let groups, current = List.fold_left (fun (groups, current) x ->
      match current with
      | [] -> (groups, [x])
      | hd :: _ when hd = x -> (groups, x :: current)
      | _ -> (List.rev current :: groups, [x])
    ) ([], [first]) rest in
    List.rev (List.rev current :: groups)

(* Approach 2: Group by key function *)
let group_by_key key lst =
  match lst with
  | [] -> []
  | first :: rest ->
    let groups, current_key, current = List.fold_left (fun (groups, k, current) x ->
      let new_key = key x in
      if new_key = k then (groups, k, x :: current)
      else ((k, List.rev current) :: groups, new_key, [x])
    ) ([], key first, [first]) rest in
    List.rev ((current_key, List.rev current) :: groups)

(* Approach 3: Run-length encoding *)
let run_length_encode lst =
  group_consecutive lst
  |> List.map (fun group ->
    (List.hd group, List.length group))

let run_length_decode encoded =
  List.concat_map (fun (value, count) ->
    List.init count (fun _ -> value)) encoded

(* Dedup consecutive *)
let dedup_consecutive lst =
  group_consecutive lst |> List.map List.hd

(* Tests *)
let () =
  assert (group_consecutive [1;1;2;2;2;3;1;1] =
          [[1;1]; [2;2;2]; [3]; [1;1]]);

  assert (group_consecutive [] = []);
  assert (group_consecutive [1] = [[1]]);

  let groups = group_by_key (fun x -> x mod 2) [2;4;1;3;6;8] in
  assert (List.length groups = 3);
  assert (fst (List.hd groups) = 0);

  assert (run_length_encode [1;1;1;2;2;3;3;3;3] =
          [(1, 3); (2, 2); (3, 4)]);

  assert (run_length_decode [(1, 3); (2, 2); (3, 4)] =
          [1;1;1;2;2;3;3;3;3]);

  assert (dedup_consecutive [1;1;2;2;3;3;1;1] = [1;2;3;1]);

  Printf.printf "✓ All tests passed\n"
