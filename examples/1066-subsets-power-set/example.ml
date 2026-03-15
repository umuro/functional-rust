(* 1066: All Subsets (Power Set) — Backtracking, Bitmasking, Fold *)

(* Approach 1: Backtracking *)
let subsets_backtrack nums =
  let n = List.length nums in
  let arr = Array.of_list nums in
  let results = ref [] in
  let current = ref [] in
  let rec build start =
    results := List.rev !current :: !results;
    for i = start to n - 1 do
      current := arr.(i) :: !current;
      build (i + 1);
      current := List.tl !current
    done
  in
  build 0;
  !results

(* Approach 2: Bitmask — each bit selects an element *)
let subsets_bitmask nums =
  let arr = Array.of_list nums in
  let n = Array.length arr in
  let total = 1 lsl n in
  List.init total (fun mask ->
    List.filter_map (fun i ->
      if mask land (1 lsl i) <> 0 then Some arr.(i) else None
    ) (List.init n (fun i -> i))
  )

(* Approach 3: Iterative doubling via fold — idiomatic OCaml *)
let subsets_fold nums =
  List.fold_left
    (fun acc x ->
      acc @ List.map (fun subset -> subset @ [x]) acc)
    [[]]
    nums

let () =
  let s1 = subsets_backtrack [1;2;3] in
  assert (List.length s1 = 8);
  assert (List.mem [] s1);
  assert (List.mem [1;2;3] s1);

  let s2 = subsets_bitmask [1;2;3] in
  assert (List.length s2 = 8);

  let s3 = subsets_fold [1;2;3] in
  assert (List.length s3 = 8);

  (* Empty input *)
  assert (List.length (subsets_backtrack []) = 1);

  Printf.printf "All subsets/power-set tests passed.\n"
