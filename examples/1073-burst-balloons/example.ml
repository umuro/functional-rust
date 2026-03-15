(* 1073: Burst Balloons — Interval DP *)

(* Approach 1: Bottom-up interval DP *)
let max_coins_dp nums =
  (* Add virtual balloons with value 1 at both ends *)
  let n = Array.length nums in
  let balloons = Array.init (n + 2) (fun i ->
    if i = 0 || i = n + 1 then 1 else nums.(i - 1)
  ) in
  let len = n + 2 in
  let dp = Array.init len (fun _ -> Array.make len 0) in
  (* dp.(i).(j) = max coins from bursting all balloons between i and j (exclusive) *)
  for gap = 2 to len - 1 do
    for i = 0 to len - gap - 1 do
      let j = i + gap in
      for k = i + 1 to j - 1 do
        let coins = dp.(i).(k) + dp.(k).(j)
                    + balloons.(i) * balloons.(k) * balloons.(j) in
        dp.(i).(j) <- max dp.(i).(j) coins
      done
    done
  done;
  dp.(0).(len - 1)

(* Approach 2: Recursive with memoization *)
let max_coins_memo nums =
  let n = Array.length nums in
  let balloons = Array.init (n + 2) (fun i ->
    if i = 0 || i = n + 1 then 1 else nums.(i - 1)
  ) in
  let len = n + 2 in
  let cache = Hashtbl.create 64 in
  let rec solve left right =
    if right - left < 2 then 0
    else
      match Hashtbl.find_opt cache (left, right) with
      | Some v -> v
      | None ->
        let best = ref 0 in
        for k = left + 1 to right - 1 do
          let coins = solve left k + solve k right
                      + balloons.(left) * balloons.(k) * balloons.(right) in
          best := max !best coins
        done;
        Hashtbl.add cache (left, right) !best;
        !best
  in
  solve 0 (len - 1)

let () =
  assert (max_coins_dp [|3; 1; 5; 8|] = 167);
  assert (max_coins_dp [|1; 5|] = 10);
  assert (max_coins_dp [|1|] = 1);

  assert (max_coins_memo [|3; 1; 5; 8|] = 167);
  assert (max_coins_memo [|1; 5|] = 10);

  Printf.printf "✓ All tests passed\n"
