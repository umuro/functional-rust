(* 1059: Rod Cutting — Maximize Revenue *)

(* Approach 1: Bottom-up DP *)
let rod_cut_dp prices n =
  let dp = Array.make (n + 1) 0 in
  for i = 1 to n do
    for j = 1 to i do
      if j <= Array.length prices then
        dp.(i) <- max dp.(i) (prices.(j - 1) + dp.(i - j))
    done
  done;
  dp.(n)

(* Approach 2: Top-down with memoization *)
let rod_cut_memo prices n =
  let cache = Hashtbl.create 32 in
  let rec solve len =
    if len = 0 then 0
    else
      match Hashtbl.find_opt cache len with
      | Some v -> v
      | None ->
        let best = ref 0 in
        for j = 1 to min len (Array.length prices) do
          best := max !best (prices.(j - 1) + solve (len - j))
        done;
        Hashtbl.add cache len !best;
        !best
  in
  solve n

(* Approach 3: With cut reconstruction *)
let rod_cut_with_cuts prices n =
  let dp = Array.make (n + 1) 0 in
  let cuts = Array.make (n + 1) 0 in
  for i = 1 to n do
    for j = 1 to min i (Array.length prices) do
      let val_ = prices.(j - 1) + dp.(i - j) in
      if val_ > dp.(i) then begin
        dp.(i) <- val_;
        cuts.(i) <- j
      end
    done
  done;
  (* Reconstruct cuts *)
  let result = ref [] in
  let remaining = ref n in
  while !remaining > 0 do
    result := cuts.(!remaining) :: !result;
    remaining := !remaining - cuts.(!remaining)
  done;
  (dp.(n), List.rev !result)

let () =
  let prices = [|1; 5; 8; 9; 10; 17; 17; 20|] in
  assert (rod_cut_dp prices 8 = 22);
  assert (rod_cut_dp prices 4 = 10);
  assert (rod_cut_memo prices 8 = 22);
  assert (rod_cut_memo prices 4 = 10);
  let (revenue, _cuts) = rod_cut_with_cuts prices 8 in
  assert (revenue = 22);

  let prices2 = [|3; 5; 8; 9; 10; 17; 17; 20|] in
  assert (rod_cut_dp prices2 4 = 12);

  Printf.printf "✓ All tests passed\n"
