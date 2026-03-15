(* 1066: All Subsets (Power Set) — Backtracking vs Bitmasking *)

(* Approach 1: Backtracking *)
let subsets_backtrack nums =
  let n = Array.length nums in
  let results = ref [] in
  let current = ref [] in
  let rec build start =
    results := List.rev !current :: !results;
    for i = start to n - 1 do
      current := nums.(i) :: !current;
      build (i + 1);
      current := List.tl !current
    done
  in
  build 0;
  List.rev !results

(* Approach 2: Bitmasking *)
let subsets_bitmask nums =
  let n = Array.length nums in
  let total = 1 lsl n in
  List.init total (fun mask ->
    let subset = ref [] in
    for i = 0 to n - 1 do
      if mask land (1 lsl i) <> 0 then
        subset := nums.(i) :: !subset
    done;
    List.rev !subset
  )

(* Approach 3: Functional — iterative doubling *)
let subsets_fold nums =
  Array.fold_left (fun acc x ->
    acc @ List.map (fun subset -> subset @ [x]) acc
  ) [[]] nums

let () =
  let s1 = subsets_backtrack [|1; 2; 3|] in
  assert (List.length s1 = 8);
  assert (List.mem [] s1);
  assert (List.mem [1; 2; 3] s1);

  let s2 = subsets_bitmask [|1; 2; 3|] in
  assert (List.length s2 = 8);

  let s3 = subsets_fold [|1; 2; 3|] in
  assert (List.length s3 = 8);

  assert (List.length (subsets_backtrack [||]) = 1);

  Printf.printf "✓ All tests passed\n"
