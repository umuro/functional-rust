(* 1071: Regex Matching — '.' and '*' — DP *)

(* Approach 1: Bottom-up DP *)
let is_match_dp s p =
  let m = String.length s and n = String.length p in
  let dp = Array.init (m + 1) (fun _ -> Array.make (n + 1) false) in
  dp.(0).(0) <- true;
  (* Pattern like a*, a*b*, a*b*c* can match empty string *)
  for j = 2 to n do
    if p.[j - 1] = '*' then dp.(0).(j) <- dp.(0).(j - 2)
  done;
  for i = 1 to m do
    for j = 1 to n do
      if p.[j - 1] = '*' then begin
        (* '*' means zero occurrences of preceding element *)
        dp.(i).(j) <- dp.(i).(j - 2);
        (* or one+ occurrences if preceding matches *)
        if p.[j - 2] = '.' || p.[j - 2] = s.[i - 1] then
          dp.(i).(j) <- dp.(i).(j) || dp.(i - 1).(j)
      end else if p.[j - 1] = '.' || p.[j - 1] = s.[i - 1] then
        dp.(i).(j) <- dp.(i - 1).(j - 1)
    done
  done;
  dp.(m).(n)

(* Approach 2: Recursive with memoization *)
let is_match_memo s p =
  let m = String.length s and n = String.length p in
  let cache = Hashtbl.create 64 in
  let rec solve i j =
    if j = n then i = m
    else
      match Hashtbl.find_opt cache (i, j) with
      | Some v -> v
      | None ->
        let first_match = i < m && (p.[j] = '.' || p.[j] = s.[i]) in
        let v =
          if j + 1 < n && p.[j + 1] = '*' then
            solve i (j + 2) || (first_match && solve (i + 1) j)
          else
            first_match && solve (i + 1) (j + 1)
        in
        Hashtbl.add cache (i, j) v;
        v
  in
  solve 0 0

let () =
  assert (is_match_dp "aa" "a" = false);
  assert (is_match_dp "aa" "a*" = true);
  assert (is_match_dp "ab" ".*" = true);
  assert (is_match_dp "aab" "c*a*b" = true);
  assert (is_match_dp "mississippi" "mis*is*p*." = false);
  assert (is_match_dp "" "a*b*" = true);

  assert (is_match_memo "aa" "a" = false);
  assert (is_match_memo "aa" "a*" = true);
  assert (is_match_memo "ab" ".*" = true);
  assert (is_match_memo "aab" "c*a*b" = true);

  Printf.printf "✓ All tests passed\n"
