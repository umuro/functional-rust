(* 1060: Partition Equal Subset Sum — Boolean DP *)

(* Approach 1: Bottom-up boolean DP *)
let can_partition nums =
  let total = Array.fold_left ( + ) 0 nums in
  if total mod 2 <> 0 then false
  else begin
    let target = total / 2 in
    let dp = Array.make (target + 1) false in
    dp.(0) <- true;
    Array.iter (fun num ->
      for j = target downto num do
        if dp.(j - num) then dp.(j) <- true
      done
    ) nums;
    dp.(target)
  end

(* Approach 2: Using a set for reachable sums *)
module IntSet = Set.Make(Int)

let can_partition_set nums =
  let total = Array.fold_left ( + ) 0 nums in
  if total mod 2 <> 0 then false
  else begin
    let target = total / 2 in
    let reachable = Array.fold_left (fun acc num ->
      IntSet.fold (fun s new_acc ->
        let sum = s + num in
        if sum <= target then IntSet.add sum new_acc else new_acc
      ) acc acc
    ) (IntSet.singleton 0) nums in
    IntSet.mem target reachable
  end

(* Approach 3: Recursive with memoization *)
let can_partition_memo nums =
  let total = Array.fold_left ( + ) 0 nums in
  if total mod 2 <> 0 then false
  else begin
    let target = total / 2 in
    let n = Array.length nums in
    let cache = Hashtbl.create 128 in
    let rec solve i remaining =
      if remaining = 0 then true
      else if i >= n || remaining < 0 then false
      else
        match Hashtbl.find_opt cache (i, remaining) with
        | Some v -> v
        | None ->
          let v = solve (i + 1) (remaining - nums.(i)) || solve (i + 1) remaining in
          Hashtbl.add cache (i, remaining) v;
          v
    in
    solve 0 target
  end

let () =
  assert (can_partition [|1; 5; 11; 5|] = true);
  assert (can_partition [|1; 2; 3; 5|] = false);
  assert (can_partition [|1; 1|] = true);
  assert (can_partition [|2; 2; 3; 5|] = false);

  assert (can_partition_set [|1; 5; 11; 5|] = true);
  assert (can_partition_set [|1; 2; 3; 5|] = false);

  assert (can_partition_memo [|1; 5; 11; 5|] = true);
  assert (can_partition_memo [|1; 2; 3; 5|] = false);

  Printf.printf "✓ All tests passed\n"
