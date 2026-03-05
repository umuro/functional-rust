(* LCS: classic DP with backtracking in OCaml *)

(* ── Recursive with memoisation ───────────────────────────────────────────────── *)
let lcs_memo s1 s2 =
  let n = String.length s1 and m = String.length s2 in
  let memo = Hashtbl.create 256 in
  let rec dp i j =
    if i = 0 || j = 0 then 0
    else match Hashtbl.find_opt memo (i, j) with
    | Some v -> v
    | None ->
      let v =
        if s1.[i-1] = s2.[j-1] then 1 + dp (i-1) (j-1)
        else max (dp (i-1) j) (dp i (j-1))
      in
      Hashtbl.replace memo (i, j) v;
      v
  in
  dp n m

(* ── Tabulation + backtrack ────────────────────────────────────────────────────── *)
let lcs_tab s1 s2 =
  let n = String.length s1 and m = String.length s2 in
  let dp = Array.make_matrix (n+1) (m+1) 0 in
  for i = 1 to n do
    for j = 1 to m do
      dp.(i).(j) <-
        if s1.[i-1] = s2.[j-1] then dp.(i-1).(j-1) + 1
        else max dp.(i-1).(j) dp.(i).(j-1)
    done
  done;
  (* Backtrack *)
  let buf = Buffer.create 16 in
  let i = ref n and j = ref m in
  while !i > 0 && !j > 0 do
    if s1.[!i - 1] = s2.[!j - 1] then begin
      Buffer.add_char buf s1.[!i - 1];
      decr i; decr j
    end else if dp.(!i-1).(!j) > dp.(!i).(!j-1) then decr i
    else decr j
  done;
  let s = Buffer.contents buf in
  (* reverse *)
  String.init (String.length s) (fun i -> s.[String.length s - 1 - i])

let () =
  let s1 = "ABCBDAB" and s2 = "BDCAB" in
  Printf.printf "LCS length (memo): %d\n"  (lcs_memo s1 s2);
  Printf.printf "LCS string (tab):  %S\n"  (lcs_tab s1 s2);
  (* Another example *)
  let a = "AGGTAB" and b = "GXTXAYB" in
  Printf.printf "LCS(%S, %S) = %S\n" a b (lcs_tab a b)
