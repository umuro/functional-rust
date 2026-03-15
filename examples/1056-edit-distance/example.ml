(* 1056: Edit Distance (Levenshtein) — 2D DP, space-optimized, memoized *)

(* Approach 1: Full 2D DP table *)
let edit_distance s1 s2 =
  let a = Array.init (String.length s1) (String.get s1) in
  let b = Array.init (String.length s2) (String.get s2) in
  let m = Array.length a and n = Array.length b in
  let dp = Array.init (m + 1) (fun i ->
    Array.init (n + 1) (fun j -> if i = 0 then j else if j = 0 then i else 0))
  in
  for i = 1 to m do
    for j = 1 to n do
      dp.(i).(j) <-
        if a.(i-1) = b.(j-1) then dp.(i-1).(j-1)
        else 1 + min dp.(i-1).(j) (min dp.(i).(j-1) dp.(i-1).(j-1))
    done
  done;
  dp.(m).(n)

(* Approach 2: Space-optimized with two rows *)
let edit_distance_opt s1 s2 =
  let a = Array.init (String.length s1) (String.get s1) in
  let b = Array.init (String.length s2) (String.get s2) in
  let m = Array.length a and n = Array.length b in
  let prev = Array.init (n + 1) (fun j -> j) in
  let curr = Array.make (n + 1) 0 in
  for i = 1 to m do
    curr.(0) <- i;
    for j = 1 to n do
      curr.(j) <-
        if a.(i-1) = b.(j-1) then prev.(j-1)
        else 1 + min prev.(j) (min curr.(j-1) prev.(j-1))
    done;
    Array.blit curr 0 prev 0 (n + 1)
  done;
  prev.(n)

(* Approach 3: Recursive with memoization *)
let edit_distance_memo s1 s2 =
  let a = Array.init (String.length s1) (String.get s1) in
  let b = Array.init (String.length s2) (String.get s2) in
  let cache = Hashtbl.create 64 in
  let rec solve i j =
    if i = 0 then j
    else if j = 0 then i
    else match Hashtbl.find_opt cache (i, j) with
    | Some v -> v
    | None ->
      let v =
        if a.(i-1) = b.(j-1) then solve (i-1) (j-1)
        else 1 + min (solve (i-1) j) (min (solve i (j-1)) (solve (i-1) (j-1)))
      in
      Hashtbl.add cache (i, j) v; v
  in
  solve (Array.length a) (Array.length b)

let () =
  let cases = [
    ("kitten",   "sitting",  3);
    ("saturday", "sunday",   3);
    ("",         "abc",      3);
    ("abc",      "abc",      0);
  ] in
  List.iter (fun (s1, s2, expected) ->
    assert (edit_distance      s1 s2 = expected);
    assert (edit_distance_opt  s1 s2 = expected);
    assert (edit_distance_memo s1 s2 = expected)
  ) cases;
  Printf.printf "All edit-distance tests passed.\n"
