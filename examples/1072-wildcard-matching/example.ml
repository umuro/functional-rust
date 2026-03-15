(* 1072: Wildcard Matching — '?' and '*' — DP, memoization, greedy *)

(* Approach 1: Bottom-up DP *)
let is_match_dp s p =
  let s = Array.init (String.length s) (String.get s) in
  let p = Array.init (String.length p) (String.get p) in
  let m = Array.length s and n = Array.length p in
  let dp = Array.init (m + 1) (fun _ -> Array.make (n + 1) false) in
  dp.(0).(0) <- true;
  (* Leading '*' characters match empty string *)
  for j = 1 to n do
    if p.(j - 1) = '*' then dp.(0).(j) <- dp.(0).(j - 1)
  done;
  for i = 1 to m do
    for j = 1 to n do
      if p.(j - 1) = '*' then
        (* '*' matches zero chars (dp[i][j-1]) or one more char (dp[i-1][j]) *)
        dp.(i).(j) <- dp.(i - 1).(j) || dp.(i).(j - 1)
      else if p.(j - 1) = '?' || p.(j - 1) = s.(i - 1) then
        dp.(i).(j) <- dp.(i - 1).(j - 1)
    done
  done;
  dp.(m).(n)

(* Approach 2: Recursive memoization *)
let is_match_memo s p =
  let s = Array.init (String.length s) (String.get s) in
  let p = Array.init (String.length p) (String.get p) in
  let sn = Array.length s and pn = Array.length p in
  let cache = Hashtbl.create 64 in
  let rec solve i j =
    match Hashtbl.find_opt cache (i, j) with
    | Some v -> v
    | None ->
      let v =
        if j = pn then i = sn
        else if i = sn then
          (* remaining pattern must be all '*' *)
          let all_star = ref true in
          for k = j to pn - 1 do
            if p.(k) <> '*' then all_star := false
          done;
          !all_star
        else if p.(j) = '*' then
          solve i (j + 1) || solve (i + 1) j
        else if p.(j) = '?' || p.(j) = s.(i) then
          solve (i + 1) (j + 1)
        else false
      in
      Hashtbl.add cache (i, j) v;
      v
  in
  solve 0 0

(* Approach 3: Greedy two-pointer — O(m+n) average *)
let is_match_greedy s p =
  let s = Array.init (String.length s) (String.get s) in
  let p = Array.init (String.length p) (String.get p) in
  let m = Array.length s and n = Array.length p in
  let si = ref 0 and pi = ref 0 in
  let star_pi = ref (-1) and match_si = ref 0 in
  while !si < m do
    if !pi < n && (p.(!pi) = '?' || p.(!pi) = s.(!si)) then begin
      incr si; incr pi
    end else if !pi < n && p.(!pi) = '*' then begin
      star_pi := !pi;
      match_si := !si;
      incr pi
    end else if !star_pi >= 0 then begin
      pi := !star_pi + 1;
      incr match_si;
      si := !match_si
    end else
      (* mismatch and no star to fall back to *)
      si := m + 1  (* signal failure *)
  done;
  (* consume trailing stars *)
  while !pi < n && p.(!pi) = '*' do incr pi done;
  !si = m && !pi = n

let () =
  let cases = [
    ("adceb", "*a*b", true);
    ("acdcb", "a*c?b", false);
    ("", "*", true);
    ("cb", "?a", false);
    ("aa", "*", true);
  ] in
  List.iter (fun (s, p, expected) ->
    let r1 = is_match_dp s p in
    let r2 = is_match_memo s p in
    let r3 = is_match_greedy s p in
    Printf.printf "is_match(%S, %S): dp=%b memo=%b greedy=%b (expected=%b)\n"
      s p r1 r2 r3 expected
  ) cases
