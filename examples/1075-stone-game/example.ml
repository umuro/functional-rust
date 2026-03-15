(* 1075: Stone Game — Minimax DP
   Players alternate picking from either end of a pile array.
   dp.(i).(j) = max score difference (current player - opponent) for piles[i..j] *)

(* Approach 1: Bottom-up interval DP *)
let stone_game_dp piles =
  let n = Array.length piles in
  let dp = Array.init n (fun _ -> Array.make n 0) in
  for i = 0 to n - 1 do dp.(i).(i) <- piles.(i) done;
  for len = 2 to n do
    for i = 0 to n - len do
      let j = i + len - 1 in
      dp.(i).(j) <- max (piles.(i) - dp.(i + 1).(j))
                        (piles.(j) - dp.(i).(j - 1))
    done
  done;
  dp.(0).(n - 1) > 0

(* Approach 2: Recursive memoization *)
let stone_game_memo piles =
  let n = Array.length piles in
  let cache = Hashtbl.create 64 in
  let rec solve i j =
    if i > j then 0
    else if i = j then piles.(i)
    else match Hashtbl.find_opt cache (i, j) with
    | Some v -> v
    | None ->
      let v = max (piles.(i) - solve (i + 1) j)
                  (piles.(j) - solve i (j - 1)) in
      Hashtbl.add cache (i, j) v;
      v
  in
  solve 0 (n - 1) > 0

(* Approach 3: Mathematical — first player always wins when pile count is even.
   With an even number of piles, player 1 can always guarantee the better of
   "all odd-indexed" vs "all even-indexed" sums by choosing first/last. *)
let stone_game_math _piles = true

(* Bonus: compute actual scores for both players *)
let stone_game_scores piles =
  let n = Array.length piles in
  let total = Array.fold_left ( + ) 0 piles in
  let dp = Array.init n (fun _ -> Array.make n 0) in
  for i = 0 to n - 1 do dp.(i).(i) <- piles.(i) done;
  for len = 2 to n do
    for i = 0 to n - len do
      let j = i + len - 1 in
      dp.(i).(j) <- max (piles.(i) - dp.(i + 1).(j))
                        (piles.(j) - dp.(i).(j - 1))
    done
  done;
  let diff = dp.(0).(n - 1) in
  let p1 = (total + diff) / 2 in
  (p1, total - p1)

let () =
  let cases = [[|5;3;4;5|]; [|3;7;2;3|]] in
  List.iter (fun piles ->
    let dp = stone_game_dp piles in
    let memo = stone_game_memo piles in
    let (p1, p2) = stone_game_scores piles in
    Printf.printf "piles=[%s]: dp=%b memo=%b math=%b scores=(%d,%d)\n"
      (Array.to_list piles |> List.map string_of_int |> String.concat ",")
      dp memo (stone_game_math piles) p1 p2
  ) cases
