(* 1054: 0/1 Knapsack — 2D DP, 1D rolling array, and memoization *)

(* Approach 1: 2D DP table *)
let knapsack_2d weights values capacity =
  let n = Array.length weights in
  let dp = Array.init (n + 1) (fun _ -> Array.make (capacity + 1) 0) in
  for i = 1 to n do
    for w = 0 to capacity do
      dp.(i).(w) <- dp.(i-1).(w);
      if weights.(i-1) <= w then
        dp.(i).(w) <- max dp.(i).(w) (dp.(i-1).(w - weights.(i-1)) + values.(i-1))
    done
  done;
  dp.(n).(capacity)

(* Approach 2: 1D rolling array — process weights in reverse to avoid reuse *)
let knapsack_1d weights values capacity =
  let dp = Array.make (capacity + 1) 0 in
  let n = Array.length weights in
  for i = 0 to n - 1 do
    (* Traverse capacity in reverse to avoid using item i twice *)
    for w = capacity downto weights.(i) do
      dp.(w) <- max dp.(w) (dp.(w - weights.(i)) + values.(i))
    done
  done;
  dp.(capacity)

(* Approach 3: Top-down memoization *)
let knapsack_memo weights values capacity =
  let n = Array.length weights in
  let cache = Hashtbl.create 64 in
  let rec solve i w =
    if i = 0 || w = 0 then 0
    else match Hashtbl.find_opt cache (i, w) with
    | Some v -> v
    | None ->
      let skip = solve (i - 1) w in
      let take =
        if weights.(i-1) <= w
        then solve (i - 1) (w - weights.(i-1)) + values.(i-1)
        else 0
      in
      let result = max skip take in
      Hashtbl.add cache (i, w) result; result
  in
  solve n capacity

(* Purely functional fold-based 1D knapsack *)
let knapsack_fold weights values capacity =
  let n = Array.length weights in
  let init_dp = Array.make (capacity + 1) 0 in
  let dp =
    Array.fold_left
      (fun dp i ->
        let dp' = Array.copy dp in
        for w = capacity downto weights.(i) do
          dp'.(w) <- max dp'.(w) (dp'.(w - weights.(i)) + values.(i))
        done;
        dp')
      init_dp
      (Array.init n (fun i -> i))
  in
  dp.(capacity)

let () =
  let cases = [
    ([|2;3;4;5|], [|3;4;5;6|], 5, 7);
    ([|1;2;3|],   [|6;10;12|], 5, 22);
  ] in
  List.iter (fun (w, v, cap, expected) ->
    assert (knapsack_2d  w v cap = expected);
    assert (knapsack_1d  w v cap = expected);
    assert (knapsack_memo w v cap = expected);
    assert (knapsack_fold w v cap = expected)
  ) cases;
  Printf.printf "All knapsack tests passed.\n"
