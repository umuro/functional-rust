(* Burrows-Wheeler Transform in OCaml *)

(* Compare two rotations of s: rotation starting at i vs rotation starting at j *)
let compare_rotations (s : string) (n : int) (i : int) (j : int) : int =
  let rec cmp k =
    if k = n then 0
    else
      let ci = s.[(i + k) mod n] and cj = s.[(j + k) mod n] in
      if ci < cj then -1
      else if ci > cj then 1
      else cmp (k + 1)
  in
  cmp 0

(* Forward BWT: returns (transformed_string, index_of_original_row) *)
let bwt (input : string) : string * int =
  let s = input ^ "$" in
  let n = String.length s in
  (* Sort rotation indices *)
  let indices = Array.init n (fun i -> i) in
  Array.sort (compare_rotations s n) indices;
  (* Last column = character before the start of each sorted rotation *)
  let transformed = String.init n (fun i -> s.[(indices.(i) + n - 1) mod n]) in
  (* Find the row corresponding to the original string *)
  let original_row =
    let found = ref 0 in
    Array.iteri (fun row i -> if i = 0 then found := row) indices;
    !found
  in
  (transformed, original_row)

(* Inverse BWT using the LF-mapping *)
let ibwt (bwt_str : string) (original_row : int) : string =
  let n = String.length bwt_str in
  let l = Array.init n (String.get bwt_str) in
  (* First column F = sorted last column L *)
  let f = Array.copy l in
  Array.sort Char.compare f;
  (* Count occurrences of each char in F (prefix counts) *)
  (* LF-mapping: next.(i) = j where l.(i) maps to f.(j) *)
  (* Build rank array: rank.(i) = how many times l.(i) appeared before i *)
  let rank = Array.make n 0 in
  let seen = Hashtbl.create 26 in
  Array.iteri (fun i c ->
    let cnt = match Hashtbl.find_opt seen c with None -> 0 | Some v -> v in
    rank.(i) <- cnt;
    Hashtbl.replace seen c (cnt + 1)
  ) l;
  (* For each char c, first_occ.(c) = first position of c in f *)
  let first_occ = Hashtbl.create 26 in
  Array.iteri (fun i c ->
    if not (Hashtbl.mem first_occ c) then
      Hashtbl.add first_occ c i
  ) f;
  (* Recover original string by following LF-mapping n-1 times *)
  let result = Buffer.create (n - 1) in
  let row = ref original_row in
  for _ = 0 to n - 2 do
    let c = l.(!row) in
    Buffer.add_char result c;
    row := (Hashtbl.find first_occ c) + rank.(!row)
  done;
  (* The recovered string is reversed and includes '$', strip it *)
  let s = Buffer.contents result in
  let reversed = String.init (String.length s) (fun i -> s.[String.length s - 1 - i]) in
  (* Remove trailing '$' *)
  String.sub reversed 0 (String.length reversed - 1)

let () =
  let tests = ["banana"; "abracadabra"; "mississippi"; "hello"] in
  List.iter (fun s ->
    let (t, row) = bwt s in
    let recovered = ibwt t row in
    Printf.printf "BWT(%S) = %S (row=%d), inverse = %S, ok=%b\n"
      s t row recovered (recovered = s)
  ) tests
