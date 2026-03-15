(* 1065: Combination Sum — Find Combos Summing to Target
   Candidates may be reused (Approach 1/2) or used once (Approach 3). *)

(* Approach 1: Backtracking — reuse allowed *)
let combination_sum candidates target =
  let sorted = List.sort compare candidates in
  let results = ref [] in
  let current = ref [] in
  let rec backtrack start remaining =
    if remaining = 0 then
      results := List.rev !current :: !results
    else
      let i = ref start in
      let stop = ref false in
      let cands = Array.of_list sorted in
      while !i < Array.length cands && not !stop do
        if cands.(!i) > remaining then stop := true
        else begin
          current := cands.(!i) :: !current;
          backtrack !i (remaining - cands.(!i));
          current := List.tl !current;
          incr i
        end
      done
  in
  backtrack 0 target;
  !results

(* Approach 2: Purely functional *)
let combination_sum_func candidates target =
  let sorted = List.sort compare candidates in
  let arr = Array.of_list sorted in
  let n = Array.length arr in
  let rec solve start remaining =
    if remaining = 0 then [[]]
    else if remaining < 0 then []
    else begin
      let results = ref [] in
      let i = ref start in
      while !i < n && arr.(!i) <= remaining do
        let combos = solve !i (remaining - arr.(!i)) in
        List.iter (fun combo ->
          results := (arr.(!i) :: combo) :: !results
        ) combos;
        incr i
      done;
      !results
    end
  in
  solve 0 target

(* Approach 3: Each element used at most once (with dedup skipping) *)
let combination_sum_unique candidates target =
  let sorted = List.sort compare candidates in
  let arr = Array.of_list sorted in
  let n = Array.length arr in
  let results = ref [] in
  let current = ref [] in
  let rec backtrack start remaining =
    if remaining = 0 then
      results := List.rev !current :: !results
    else
      for i = start to n - 1 do
        if arr.(i) <= remaining
           && (i = start || arr.(i) <> arr.(i-1)) then begin
          current := arr.(i) :: !current;
          backtrack (i + 1) (remaining - arr.(i));
          current := List.tl !current
        end
      done
  in
  backtrack 0 target;
  !results

let () =
  let r1 = combination_sum [2;3;6;7] 7 in
  assert (List.length r1 = 2);
  assert (List.mem [2;2;3] r1);
  assert (List.mem [7] r1);

  let r2 = combination_sum_func [2;3;5] 8 in
  assert (List.length r2 = 3);

  let r3 = combination_sum_unique [10;1;2;7;6;1;5] 8 in
  assert (List.mem [1;1;6] r3);
  assert (List.mem [1;2;5] r3);
  assert (List.mem [1;7] r3);
  assert (List.mem [2;6] r3);

  Printf.printf "All combination-sum tests passed.\n"
