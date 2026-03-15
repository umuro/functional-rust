(* 1054: 0/1 Knapsack — 2D DP Table *)

(* Approach 1: 2D array DP *)
let knapsack_2d weights values capacity =
  let n = Array.length weights in
  let dp = Array.init (n + 1) (fun _ -> Array.make (capacity + 1) 0) in
  for i = 1 to n do
    for w = 0 to capacity do
      dp.(i).(w) <- dp.(i - 1).(w);
      if weights.(i - 1) <= w then
        dp.(i).(w) <- max dp.(i).(w)
          (dp.(i - 1).(w - weights.(i - 1)) + values.(i - 1))
    done
  done;
  dp.(n).(capacity)

(* Approach 2: 1D rolling array optimization *)
let knapsack_1d weights values capacity =
  let n = Array.length weights in
  let dp = Array.make (capacity + 1) 0 in
  for i = 0 to n - 1 do
    for w = capacity downto weights.(i) do
      dp.(w) <- max dp.(w) (dp.(w - weights.(i)) + values.(i))
    done
  done;
  dp.(capacity)

(* Approach 3: Recursive with memoization *)
let knapsack_memo weights values capacity =
  let n = Array.length weights in
  let cache = Hashtbl.create 128 in
  let rec solve i w =
    if i = 0 || w = 0 then 0
    else
      match Hashtbl.find_opt cache (i, w) with
      | Some v -> v
      | None ->
        let skip = solve (i - 1) w in
        let take =
          if weights.(i - 1) <= w then
            solve (i - 1) (w - weights.(i - 1)) + values.(i - 1)
          else 0
        in
        let result = max skip take in
        Hashtbl.add cache (i, w) result;
        result
  in
  solve n capacity

let () =
  let weights = [|2; 3; 4; 5|] in
  let values = [|3; 4; 5; 6|] in
  assert (knapsack_2d weights values 5 = 7);
  assert (knapsack_1d weights values 5 = 7);
  assert (knapsack_memo weights values 5 = 7);

  let w2 = [|1; 2; 3|] in
  let v2 = [|6; 10; 12|] in
  assert (knapsack_2d w2 v2 5 = 22);
  assert (knapsack_1d w2 v2 5 = 22);
  assert (knapsack_memo w2 v2 5 = 22);

  Printf.printf "✓ All tests passed\n"
