(* 970: Rope String
   A tree-based string representation for efficient concatenation and splitting.
   concat is O(1); indexing and split are O(log n).
   OCaml algebraic types make the recursive structure natural. *)

type rope =
  | Leaf   of string                  (* actual text *)
  | Branch of rope * rope * int       (* left, right, total length *)

let length = function
  | Leaf s         -> String.length s
  | Branch (_,_,n) -> n

let of_string s = Leaf s

let concat a b =
  if length a = 0 then b
  else if length b = 0 then a
  else Branch (a, b, length a + length b)

(* O(log n) character access *)
let rec get rope i =
  match rope with
  | Leaf s -> s.[i]
  | Branch (left, right, _) ->
    let ln = length left in
    if i < ln then get left i
    else          get right (i - ln)

(* Flatten to a string — O(n) *)
let rec to_string = function
  | Leaf s              -> s
  | Branch (l, r, _)    -> to_string l ^ to_string r

(* Split rope into (left [0,i), right [i,n)) — O(log n) *)
let rec split rope i =
  match rope with
  | Leaf s ->
    (Leaf (String.sub s 0 i), Leaf (String.sub s i (String.length s - i)))
  | Branch (left, right, _) ->
    let ln = length left in
    if i = ln then (left, right)
    else if i < ln then
      let (ll, lr) = split left i in
      (ll, concat lr right)
    else
      let (rl, rr) = split right (i - ln) in
      (concat left rl, rr)

(* Insert string at position i *)
let insert rope i s =
  let (l, r) = split rope i in
  concat (concat l (of_string s)) r

(* Delete characters [i, i+len) *)
let delete rope i len =
  let (l, rest) = split rope i in
  let (_, r) = split rest len in
  concat l r

(* Substring [i, i+len) *)
let substring rope i len =
  let (_, rest) = split rope i in
  let (s, _)   = split rest len in
  to_string s

(* Collect all leaf strings in order — useful for inspecting tree shape *)
let rec leaves = function
  | Leaf s           -> [s]
  | Branch (l, r, _) -> leaves l @ leaves r

(* Re-balance by flattening and rebuilding as a balanced tree *)
let rebalance rope =
  let s = to_string rope in
  let rec build str lo hi =
    if hi - lo <= 16 then Leaf (String.sub str lo (hi - lo))
    else
      let mid = (lo + hi) / 2 in
      let l = build str lo mid and r = build str mid hi in
      Branch (l, r, hi - lo)
  in
  build s 0 (String.length s)

let () =
  let r1 = of_string "Hello, " in
  let r2 = of_string "world" in
  let r3 = of_string "!" in
  let rope = concat (concat r1 r2) r3 in

  Printf.printf "to_string: %s\n" (to_string rope);
  Printf.printf "length: %d\n" (length rope);
  Printf.printf "get[0] = '%c', get[7] = '%c'\n" (get rope 0) (get rope 7);

  let (left, right) = split rope 7 in
  Printf.printf "split at 7: \"%s\" | \"%s\"\n"
    (to_string left) (to_string right);

  let rope2 = insert rope 7 "beautiful " in
  Printf.printf "insert \"beautiful \" at 7: %s\n" (to_string rope2);

  let rope3 = delete rope2 7 10 in
  Printf.printf "delete [7,17): %s\n" (to_string rope3);

  Printf.printf "substring [7,12): %s\n" (substring rope2 7 5);

  Printf.printf "leaves: [%s]\n"
    (String.concat "; " (List.map (Printf.sprintf "\"%s\"") (leaves rope)));

  (* Large concatenation chain — stays O(1) per concat *)
  let big = List.fold_left (fun acc s -> concat acc (of_string s))
    (of_string "") ["The "; "quick "; "brown "; "fox "; "jumps"] in
  Printf.printf "big rope: \"%s\"\n" (to_string big);
  let balanced = rebalance big in
  Printf.printf "rebalanced length: %d\n" (length balanced)
