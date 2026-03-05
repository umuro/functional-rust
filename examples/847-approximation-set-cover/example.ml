(* Greedy Set Cover Approximation in OCaml *)

module IntSet = Set.Make(Int)

(* Greedy set cover: always pick the set covering the most uncovered elements *)
(* Returns list of chosen set indices *)
let greedy_set_cover (universe : int list) (sets : int list list) : int list =
  let uncovered = ref (IntSet.of_list universe) in
  let remaining_sets = ref (List.mapi (fun i s -> (i, IntSet.of_list s)) sets) in
  let chosen = ref [] in
  while not (IntSet.is_empty !uncovered) && not (List.is_empty !remaining_sets) do
    (* Find set with maximum coverage of uncovered elements *)
    let best = List.fold_left (fun best_opt (i, s) ->
      let covered = IntSet.cardinal (IntSet.inter s !uncovered) in
      match best_opt with
      | None -> Some (i, s, covered)
      | Some (_, _, best_cov) ->
        if covered > best_cov then Some (i, s, covered) else best_opt
    ) None !remaining_sets in
    match best with
    | None -> ()
    | Some (i, s, _) ->
      chosen := i :: !chosen;
      uncovered := IntSet.diff !uncovered s;
      remaining_sets := List.filter (fun (j, _) -> j <> i) !remaining_sets
  done;
  List.rev !chosen

(* Weighted set cover: each set has a cost; choose minimum-cost cover *)
(* Greedy: pick set with lowest cost per new element covered *)
let greedy_weighted_set_cover
    (universe : int list) (sets : (int list * float) list) : int list =
  let uncovered = ref (IntSet.of_list universe) in
  let indexed = List.mapi (fun i (s, cost) -> (i, IntSet.of_list s, cost)) sets in
  let remaining = ref indexed in
  let chosen = ref [] in
  while not (IntSet.is_empty !uncovered) && not (List.is_empty !remaining) do
    let best = List.fold_left (fun best_opt (i, s, cost) ->
      let new_covered = IntSet.cardinal (IntSet.inter s !uncovered) in
      if new_covered = 0 then best_opt
      else
        let ratio = cost /. float_of_int new_covered in
        match best_opt with
        | None -> Some (i, s, ratio)
        | Some (_, _, best_r) -> if ratio < best_r then Some (i, s, ratio) else best_opt
    ) None !remaining in
    match best with
    | None -> ()
    | Some (i, s, _) ->
      chosen := i :: !chosen;
      uncovered := IntSet.diff !uncovered s;
      remaining := List.filter (fun (j, _, _) -> j <> i) !remaining
  done;
  List.rev !chosen

let () =
  (* Classic example *)
  let universe = [1;2;3;4;5;6;7;8;9;10] in
  let sets = [
    [1;2;3;4;5];
    [4;5;6;7;8];
    [1;3;5;7;9];
    [2;4;6;8;10];
    [6;7;8;9;10];
  ] in
  let chosen = greedy_set_cover universe sets in
  Printf.printf "Greedy set cover (unweighted):\n";
  Printf.printf "  Chosen sets (0-indexed): [%s]\n"
    (String.concat "; " (List.map string_of_int chosen));
  Printf.printf "  Number of sets: %d\n" (List.length chosen)
