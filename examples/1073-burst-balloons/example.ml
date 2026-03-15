(* 1073: Burst Balloons — Interval DP *)

(* Build augmented array with sentinel 1s at each end *)
let make_balloons nums =
  let n = Array.length nums in
  let b = Array.make (n + 2) 1 in
  Array.blit nums 0 b 1 n;
  b

(* Approach 1: Bottom-up interval DP *)
let max_coins_dp nums =
  let b = make_balloons nums in
  let len = Array.length b in
  (* dp.(i).(j) = max coins from bursting all balloons strictly between i and j *)
  let dp = Array.init len (fun _ -> Array.make len 0) in
  (* gap = number of elements in the open interval (i, j) *)
  for gap = 2 to len - 1 do
    for i = 0 to len - 1 - gap do
      let j = i + gap in
      for k = i + 1 to j - 1 do
        let coins = dp.(i).(k) + dp.(k).(j) + b.(i) * b.(k) * b.(j) in
        if coins > dp.(i).(j) then dp.(i).(j) <- coins
      done
    done
  done;
  dp.(0).(len - 1)

(* Approach 2: Recursive memoization *)
let max_coins_memo nums =
  let b = make_balloons nums in
  let len = Array.length b in
  let cache = Hashtbl.create 64 in
  let rec solve left right =
    if right - left < 2 then 0
    else match Hashtbl.find_opt cache (left, right) with
    | Some v -> v
    | None ->
      let best = ref 0 in
      for k = left + 1 to right - 1 do
        let coins = solve left k + solve k right + b.(left) * b.(k) * b.(right) in
        if coins > !best then best := coins
      done;
      Hashtbl.add cache (left, right) !best;
      !best
  in
  solve 0 (len - 1)

let () =
  let cases = [([|3;1;5;8|], 167); ([|1;5|], 10); ([|1|], 1)] in
  List.iter (fun (nums, expected) ->
    let r1 = max_coins_dp nums in
    let r2 = max_coins_memo nums in
    Printf.printf "max_coins([%s]) = dp:%d memo:%d (expected:%d)\n"
      (Array.to_list nums |> List.map string_of_int |> String.concat ",")
      r1 r2 expected
  ) cases
