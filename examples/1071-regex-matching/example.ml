(* 1071: Regex Matching — '.' and '*' — DP and memoization *)

(* Approach 1: Bottom-up DP *)
let is_match_dp s p =
  let s = Array.init (String.length s) (String.get s) in
  let p = Array.init (String.length p) (String.get p) in
  let m = Array.length s and n = Array.length p in
  (* dp.(i).(j) = s[0..i) matches p[0..j) *)
  let dp = Array.init (m + 1) (fun _ -> Array.make (n + 1) false) in
  dp.(0).(0) <- true;
  (* Pattern a* can match empty string *)
  for j = 2 to n do
    if p.(j - 1) = '*' then dp.(0).(j) <- dp.(0).(j - 2)
  done;
  for i = 1 to m do
    for j = 1 to n do
      if p.(j - 1) = '*' then begin
        (* zero occurrences of preceding element *)
        dp.(i).(j) <- dp.(i).(j - 2);
        (* one or more occurrences *)
        if p.(j - 2) = '.' || p.(j - 2) = s.(i - 1) then
          dp.(i).(j) <- dp.(i).(j) || dp.(i - 1).(j)
      end else if p.(j - 1) = '.' || p.(j - 1) = s.(i - 1) then
        dp.(i).(j) <- dp.(i - 1).(j - 1)
    done
  done;
  dp.(m).(n)

(* Approach 2: Recursive memoization with a Hashtbl *)
let is_match_memo s p =
  let s = Array.init (String.length s) (String.get s) in
  let p = Array.init (String.length p) (String.get p) in
  let cache = Hashtbl.create 64 in
  let rec solve i j =
    match Hashtbl.find_opt cache (i, j) with
    | Some v -> v
    | None ->
      let v =
        if j = Array.length p then i = Array.length s
        else begin
          let first_match = i < Array.length s && (p.(j) = '.' || p.(j) = s.(i)) in
          if j + 1 < Array.length p && p.(j + 1) = '*' then
            (* skip x* pair, or match one and stay *)
            solve i (j + 2) || (first_match && solve (i + 1) j)
          else
            first_match && solve (i + 1) (j + 1)
        end
      in
      Hashtbl.add cache (i, j) v;
      v
  in
  solve 0 0

let () =
  let cases = [
    ("aa", "a", false);
    ("aa", "a*", true);
    ("ab", ".*", true);
    ("aab", "c*a*b", true);
    ("", "a*b*", true);
  ] in
  List.iter (fun (s, p, expected) ->
    let dp_result = is_match_dp s p in
    let memo_result = is_match_memo s p in
    Printf.printf "is_match(%S, %S) = dp:%b memo:%b (expected:%b)\n"
      s p dp_result memo_result expected
  ) cases
