(* 1059: Rod Cutting — Maximize Revenue
   Bottom-up DP, memoization, and cut reconstruction. *)

(* Approach 1: Bottom-up DP *)
let rod_cut_dp prices n =
  let dp = Array.make (n + 1) 0 in
  for i = 1 to n do
    for j = 1 to min i (Array.length prices) do
      if prices.(j-1) + dp.(i - j) > dp.(i) then
        dp.(i) <- prices.(j-1) + dp.(i - j)
    done
  done;
  dp.(n)

(* Approach 2: Top-down memoization *)
let rod_cut_memo prices n =
  let cache = Hashtbl.create 32 in
  let rec solve len =
    if len = 0 then 0
    else match Hashtbl.find_opt cache len with
    | Some v -> v
    | None ->
      let best = ref 0 in
      for j = 1 to min len (Array.length prices) do
        let v = prices.(j-1) + solve (len - j) in
        if v > !best then best := v
      done;
      Hashtbl.add cache len !best; !best
  in
  solve n

(* Approach 3: With cut sizes reconstruction *)
let rod_cut_with_cuts prices n =
  let dp   = Array.make (n + 1) 0 in
  let cuts = Array.make (n + 1) 0 in
  for i = 1 to n do
    for j = 1 to min i (Array.length prices) do
      let v = prices.(j-1) + dp.(i - j) in
      if v > dp.(i) then begin
        dp.(i)   <- v;
        cuts.(i) <- j
      end
    done
  done;
  let result = ref [] in
  let remaining = ref n in
  while !remaining > 0 do
    result := cuts.(!remaining) :: !result;
    remaining := !remaining - cuts.(!remaining)
  done;
  (dp.(n), List.rev !result)

let () =
  let prices = [|1;5;8;9;10;17;17;20|] in
  assert (rod_cut_dp   prices 8 = 22);
  assert (rod_cut_dp   prices 4 = 10);
  assert (rod_cut_dp   [|3;5;8;9;10;17;17;20|] 4 = 12);
  assert (rod_cut_memo prices 8 = 22);
  assert (rod_cut_memo prices 4 = 10);

  let (revenue, cuts) = rod_cut_with_cuts prices 8 in
  assert (revenue = 22);
  assert (List.fold_left ( + ) 0 cuts = 8);

  Printf.printf "All rod-cutting tests passed.\n"
