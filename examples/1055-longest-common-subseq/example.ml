(* 1055: Longest Common Subsequence — 2D DP + Backtrack *)

(* Approach 1: 2D DP table for length *)
let lcs_length s1 s2 =
  let m = String.length s1 and n = String.length s2 in
  let dp = Array.init (m + 1) (fun _ -> Array.make (n + 1) 0) in
  for i = 1 to m do
    for j = 1 to n do
      if s1.[i - 1] = s2.[j - 1] then
        dp.(i).(j) <- dp.(i - 1).(j - 1) + 1
      else
        dp.(i).(j) <- max dp.(i - 1).(j) dp.(i).(j - 1)
    done
  done;
  dp.(m).(n)

(* Approach 2: DP + backtrack to reconstruct the subsequence *)
let lcs_string s1 s2 =
  let m = String.length s1 and n = String.length s2 in
  let dp = Array.init (m + 1) (fun _ -> Array.make (n + 1) 0) in
  for i = 1 to m do
    for j = 1 to n do
      if s1.[i - 1] = s2.[j - 1] then
        dp.(i).(j) <- dp.(i - 1).(j - 1) + 1
      else
        dp.(i).(j) <- max dp.(i - 1).(j) dp.(i).(j - 1)
    done
  done;
  (* Backtrack to find the actual subsequence *)
  let buf = Buffer.create 16 in
  let i = ref m and j = ref n in
  while !i > 0 && !j > 0 do
    if s1.[!i - 1] = s2.[!j - 1] then begin
      Buffer.add_char buf s1.[!i - 1];
      decr i; decr j
    end else if dp.(!i - 1).(!j) > dp.(!i).(!j - 1) then
      decr i
    else
      decr j
  done;
  let s = Buffer.contents buf in
  String.init (String.length s) (fun i -> s.[String.length s - 1 - i])

(* Approach 3: Recursive with memoization *)
let lcs_memo s1 s2 =
  let cache = Hashtbl.create 128 in
  let rec solve i j =
    if i = 0 || j = 0 then 0
    else
      match Hashtbl.find_opt cache (i, j) with
      | Some v -> v
      | None ->
        let v =
          if s1.[i - 1] = s2.[j - 1] then solve (i - 1) (j - 1) + 1
          else max (solve (i - 1) j) (solve i (j - 1))
        in
        Hashtbl.add cache (i, j) v;
        v
  in
  solve (String.length s1) (String.length s2)

let () =
  assert (lcs_length "abcde" "ace" = 3);
  assert (lcs_length "abc" "abc" = 3);
  assert (lcs_length "abc" "def" = 0);
  assert (lcs_string "abcde" "ace" = "ace");
  assert (lcs_string "AGGTAB" "GXTXAYB" = "GTAB");
  assert (lcs_memo "abcde" "ace" = 3);
  assert (lcs_memo "AGGTAB" "GXTXAYB" = 4);
  Printf.printf "✓ All tests passed\n"
