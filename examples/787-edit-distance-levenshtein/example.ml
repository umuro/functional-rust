(* Levenshtein edit distance in OCaml *)

let min3 a b c = min a (min b c)

(* ── Recursive with memoisation ───────────────────────────────────────────────── *)
let edit_memo s1 s2 =
  let n = String.length s1 and m = String.length s2 in
  let memo = Hashtbl.create 256 in
  let rec dp i j =
    if i = 0 then j
    else if j = 0 then i
    else match Hashtbl.find_opt memo (i, j) with
    | Some v -> v
    | None ->
      let v =
        if s1.[i-1] = s2.[j-1] then dp (i-1) (j-1)
        else 1 + min3 (dp (i-1) j) (dp i (j-1)) (dp (i-1) (j-1))
      in
      Hashtbl.replace memo (i, j) v;
      v
  in
  dp n m

(* ── Tabulation ────────────────────────────────────────────────────────────────── *)
let edit_tab s1 s2 =
  let n = String.length s1 and m = String.length s2 in
  let dp = Array.make_matrix (n+1) (m+1) 0 in
  for i = 0 to n do dp.(i).(0) <- i done;
  for j = 0 to m do dp.(0).(j) <- j done;
  for i = 1 to n do
    for j = 1 to m do
      dp.(i).(j) <-
        if s1.[i-1] = s2.[j-1] then dp.(i-1).(j-1)
        else 1 + min3 dp.(i-1).(j) dp.(i).(j-1) dp.(i-1).(j-1)
    done
  done;
  dp.(n).(m)

(* ── Space-optimised ───────────────────────────────────────────────────────────── *)
let edit_opt s1 s2 =
  let n = String.length s1 and m = String.length s2 in
  let prev = Array.init (m+1) Fun.id in
  let curr = Array.make (m+1) 0 in
  for i = 1 to n do
    curr.(0) <- i;
    for j = 1 to m do
      curr.(j) <-
        if s1.[i-1] = s2.[j-1] then prev.(j-1)
        else 1 + min3 prev.(j) curr.(j-1) prev.(j-1)
    done;
    Array.blit curr 0 prev 0 (m+1)
  done;
  prev.(m)

let () =
  let pairs = [
    ("kitten", "sitting");
    ("saturday", "sunday");
    ("", "abc");
    ("abc", "abc");
  ] in
  List.iter (fun (s1, s2) ->
    Printf.printf "edit(%S, %S) = %d (memo=%d opt=%d)\n"
      s1 s2 (edit_tab s1 s2) (edit_memo s1 s2) (edit_opt s1 s2)
  ) pairs
