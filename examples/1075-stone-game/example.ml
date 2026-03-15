(* 1075: Stone Game — Minimax DP *)

(* Approach 1: Bottom-up interval DP *)
let stone_game_dp piles =
  let n = Array.length piles in
  (* dp.(i).(j) = max score difference (current player - opponent) for piles[i..j] *)
  let dp = Array.init n (fun i -> Array.init n (fun j ->
    if i = j then piles.(i) else 0
  )) in
  for len = 2 to n do
    for i = 0 to n - len do
      let j = i + len - 1 in
      dp.(i).(j) <- max
        (piles.(i) - dp.(i + 1).(j))
        (piles.(j) - dp.(i).(j - 1))
    done
  done;
  dp.(0).(n - 1) > 0

(* Approach 2: Recursive with memoization *)
let stone_game_memo piles =
  let n = Array.length piles in
  let cache = Hashtbl.create 64 in
  let rec solve i j =
    if i > j then 0
    else if i = j then piles.(i)
    else
      match Hashtbl.find_opt cache (i, j) with
      | Some v -> v
      | None ->
        let v = max
          (piles.(i) - solve (i + 1) j)
          (piles.(j) - solve i (j - 1))
        in
        Hashtbl.add cache (i, j) v;
        v
  in
  solve 0 (n - 1) > 0

(* Approach 3: Mathematical insight — first player always wins with even piles *)
let stone_game_math _piles =
  (* With even number of piles, first player can always choose
     all odd-indexed or all even-indexed piles. One of those sums
     is strictly greater. So first player always wins. *)
  true

(* Generalized: compute actual scores *)
let stone_game_scores piles =
  let n = Array.length piles in
  let total = Array.fold_left ( + ) 0 piles in
  let dp = Array.init n (fun i -> Array.init n (fun j ->
    if i = j then piles.(i) else 0
  )) in
  for len = 2 to n do
    for i = 0 to n - len do
      let j = i + len - 1 in
      dp.(i).(j) <- max
        (piles.(i) - dp.(i + 1).(j))
        (piles.(j) - dp.(i).(j - 1))
    done
  done;
  let diff = dp.(0).(n - 1) in
  let p1 = (total + diff) / 2 in
  let p2 = total - p1 in
  (p1, p2)

let () =
  assert (stone_game_dp [|5; 3; 4; 5|] = true);
  assert (stone_game_dp [|3; 7; 2; 3|] = true);

  assert (stone_game_memo [|5; 3; 4; 5|] = true);
  assert (stone_game_memo [|3; 7; 2; 3|] = true);

  assert (stone_game_math [|5; 3; 4; 5|] = true);

  let (p1, p2) = stone_game_scores [|5; 3; 4; 5|] in
  assert (p1 > p2);
  assert (p1 + p2 = 17);

  Printf.printf "✓ All tests passed\n"
