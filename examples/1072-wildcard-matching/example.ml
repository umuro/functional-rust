(* 1072: Wildcard Matching — '?' and '*' — DP *)

(* Approach 1: Bottom-up DP *)
let is_match_dp s p =
  let m = String.length s and n = String.length p in
  let dp = Array.init (m + 1) (fun _ -> Array.make (n + 1) false) in
  dp.(0).(0) <- true;
  for j = 1 to n do
    if p.[j - 1] = '*' then dp.(0).(j) <- dp.(0).(j - 1)
  done;
  for i = 1 to m do
    for j = 1 to n do
      if p.[j - 1] = '*' then
        dp.(i).(j) <- dp.(i - 1).(j) || dp.(i).(j - 1)
      else if p.[j - 1] = '?' || p.[j - 1] = s.[i - 1] then
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
    else if i = m then
      (* remaining pattern must be all '*' *)
      let all_star = ref true in
      for k = j to n - 1 do
        if p.[k] <> '*' then all_star := false
      done;
      !all_star
    else
      match Hashtbl.find_opt cache (i, j) with
      | Some v -> v
      | None ->
        let v =
          if p.[j] = '*' then
            solve i (j + 1) || solve (i + 1) j
          else if p.[j] = '?' || p.[j] = s.[i] then
            solve (i + 1) (j + 1)
          else
            false
        in
        Hashtbl.add cache (i, j) v;
        v
  in
  solve 0 0

(* Approach 3: Two-pointer greedy *)
let is_match_greedy s p =
  let m = String.length s and n = String.length p in
  let si = ref 0 and pi = ref 0 in
  let star_idx = ref (-1) and match_idx = ref 0 in
  while !si < m do
    if !pi < n && (p.[!pi] = '?' || p.[!pi] = s.[!si]) then begin
      incr si; incr pi
    end else if !pi < n && p.[!pi] = '*' then begin
      star_idx := !pi;
      match_idx := !si;
      incr pi
    end else if !star_idx >= 0 then begin
      pi := !star_idx + 1;
      incr match_idx;
      si := !match_idx
    end else
      (* Mismatch *)
      si := m + 1  (* force exit with failure *)
  done;
  while !pi < n && p.[!pi] = '*' do incr pi done;
  !si = m && !pi = n

let () =
  assert (is_match_dp "adceb" "*a*b" = true);
  assert (is_match_dp "acdcb" "a*c?b" = false);
  assert (is_match_dp "" "*" = true);
  assert (is_match_dp "cb" "?a" = false);
  assert (is_match_dp "aa" "a" = false);
  assert (is_match_dp "aa" "*" = true);

  assert (is_match_memo "adceb" "*a*b" = true);
  assert (is_match_memo "acdcb" "a*c?b" = false);

  assert (is_match_greedy "adceb" "*a*b" = true);
  assert (is_match_greedy "acdcb" "a*c?b" = false);
  assert (is_match_greedy "" "*" = true);

  Printf.printf "✓ All tests passed\n"
