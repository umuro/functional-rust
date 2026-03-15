(* 1060: Partition Equal Subset Sum — Boolean DP
   Can we split the array into two halves with equal sum? *)

(* Approach 1: Bottom-up boolean DP *)
let can_partition nums =
  let total = Array.fold_left ( + ) 0 nums in
  if total mod 2 <> 0 then false
  else begin
    let target = total / 2 in
    let dp = Array.make (target + 1) false in
    dp.(0) <- true;
    Array.iter (fun num ->
      (* Traverse in reverse to avoid reusing the same element *)
      for j = target downto num do
        if dp.(j - num) then dp.(j) <- true
      done
    ) nums;
    dp.(target)
  end

(* Approach 2: Using a set of reachable sums *)
module IntSet = Set.Make(Int)

let can_partition_set nums =
  let total = Array.fold_left ( + ) 0 nums in
  if total mod 2 <> 0 then false
  else begin
    let target = total / 2 in
    let reachable = ref (IntSet.singleton 0) in
    Array.iter (fun num ->
      (* filter_map not available in 4.10; use fold instead *)
      let new_sums = IntSet.fold (fun s acc ->
        let s' = s + num in
        if s' <= target then IntSet.add s' acc else acc
      ) !reachable IntSet.empty in
      reachable := IntSet.union !reachable new_sums
    ) nums;
    IntSet.mem target !reachable
  end

(* Approach 3: Recursive with memoization *)
let can_partition_memo nums =
  let total = Array.fold_left ( + ) 0 nums in
  if total mod 2 <> 0 then false
  else begin
    let target = total / 2 in
    let n = Array.length nums in
    let cache = Hashtbl.create 64 in
    let rec solve i remaining =
      if remaining = 0 then true
      else if i >= n || remaining < 0 then false
      else match Hashtbl.find_opt cache (i, remaining) with
      | Some v -> v
      | None ->
        let v = solve (i + 1) (remaining - nums.(i))
             || solve (i + 1) remaining in
        Hashtbl.add cache (i, remaining) v; v
    in
    solve 0 target
  end

let () =
  assert (can_partition      [|1;5;11;5|] = true);
  assert (can_partition      [|1;2;3;5|]  = false);
  assert (can_partition      [|1;1|]       = true);

  assert (can_partition_set  [|1;5;11;5|] = true);
  assert (can_partition_set  [|1;2;3;5|]  = false);

  assert (can_partition_memo [|1;5;11;5|] = true);
  assert (can_partition_memo [|1;2;3;5|]  = false);

  Printf.printf "All partition-equal-subset tests passed.\n"
