(* 1056: Edit Distance (Levenshtein) — 2D DP Table *)

(* Approach 1: 2D DP table *)
let edit_distance s1 s2 =
  let m = String.length s1 and n = String.length s2 in
  let dp = Array.init (m + 1) (fun i ->
    Array.init (n + 1) (fun j ->
      if i = 0 then j else if j = 0 then i else 0
    )
  ) in
  for i = 1 to m do
    for j = 1 to n do
      if s1.[i - 1] = s2.[j - 1] then
        dp.(i).(j) <- dp.(i - 1).(j - 1)
      else
        dp.(i).(j) <- 1 + min (min dp.(i - 1).(j) dp.(i).(j - 1)) dp.(i - 1).(j - 1)
    done
  done;
  dp.(m).(n)

(* Approach 2: Space-optimized with two rows *)
let edit_distance_opt s1 s2 =
  let m = String.length s1 and n = String.length s2 in
  let prev = Array.init (n + 1) Fun.id in
  let curr = Array.make (n + 1) 0 in
  for i = 1 to m do
    curr.(0) <- i;
    for j = 1 to n do
      if s1.[i - 1] = s2.[j - 1] then
        curr.(j) <- prev.(j - 1)
      else
        curr.(j) <- 1 + min (min prev.(j) curr.(j - 1)) prev.(j - 1)
    done;
    Array.blit curr 0 prev 0 (n + 1)
  done;
  prev.(n)

(* Approach 3: Recursive with memoization *)
let edit_distance_memo s1 s2 =
  let cache = Hashtbl.create 128 in
  let rec solve i j =
    if i = 0 then j
    else if j = 0 then i
    else
      match Hashtbl.find_opt cache (i, j) with
      | Some v -> v
      | None ->
        let v =
          if s1.[i - 1] = s2.[j - 1] then solve (i - 1) (j - 1)
          else 1 + min (min (solve (i - 1) j) (solve i (j - 1))) (solve (i - 1) (j - 1))
        in
        Hashtbl.add cache (i, j) v;
        v
  in
  solve (String.length s1) (String.length s2)

let () =
  assert (edit_distance "kitten" "sitting" = 3);
  assert (edit_distance "saturday" "sunday" = 3);
  assert (edit_distance "" "abc" = 3);
  assert (edit_distance "abc" "" = 3);
  assert (edit_distance "abc" "abc" = 0);

  assert (edit_distance_opt "kitten" "sitting" = 3);
  assert (edit_distance_opt "saturday" "sunday" = 3);

  assert (edit_distance_memo "kitten" "sitting" = 3);
  assert (edit_distance_memo "saturday" "sunday" = 3);

  Printf.printf "✓ All tests passed\n"
