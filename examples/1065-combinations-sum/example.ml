(* 1065: Combination Sum — Find Combos Summing to Target *)

(* Approach 1: Backtracking with reuse allowed *)
let combination_sum candidates target =
  let sorted = List.sort compare candidates in
  let results = ref [] in
  let rec backtrack start current remaining =
    if remaining = 0 then
      results := List.rev current :: !results
    else if remaining > 0 then
      List.iteri (fun idx c ->
        if idx >= start && c <= remaining then
          backtrack idx (c :: current) (remaining - c)
      ) sorted
  in
  backtrack 0 [] target;
  List.rev !results

(* Approach 2: Functional with list accumulation *)
let combination_sum_func candidates target =
  let sorted = List.sort compare candidates in
  let rec solve start remaining =
    if remaining = 0 then [[]]
    else if remaining < 0 then []
    else
      List.filteri (fun i _ -> i >= start) sorted
      |> List.concat_map (fun (c : int) ->
        if c > remaining then []
        else
          let idx = List.filteri (fun i _ -> i >= start) sorted
                    |> List.mapi (fun i x -> (i + start, x))
                    |> List.find (fun (_, x) -> x = c)
                    |> fst in
          List.map (fun rest -> c :: rest) (solve idx (remaining - c))
      )
  in
  solve 0 target

(* Approach 3: Array-based for efficiency *)
let combination_sum_arr candidates target =
  let arr = Array.of_list (List.sort compare candidates) in
  let n = Array.length arr in
  let results = ref [] in
  let current = ref [] in
  let rec backtrack start remaining =
    if remaining = 0 then
      results := List.rev !current :: !results
    else
      for i = start to n - 1 do
        if arr.(i) <= remaining then begin
          current := arr.(i) :: !current;
          backtrack i (remaining - arr.(i));
          current := List.tl !current
        end
      done
  in
  backtrack 0 target;
  List.rev !results

let () =
  let r1 = combination_sum [2; 3; 6; 7] 7 in
  assert (List.length r1 = 2);  (* [2;2;3] and [7] *)

  let r2 = combination_sum [2; 3; 5] 8 in
  assert (List.length r2 = 3);  (* [2;2;2;2], [2;3;3], [3;5] *)

  let r3 = combination_sum_arr [2; 3; 6; 7] 7 in
  assert (List.length r3 = 2);

  Printf.printf "✓ All tests passed\n"
