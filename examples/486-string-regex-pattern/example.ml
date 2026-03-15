(* 486: Regex-like matching without external libraries in OCaml *)
(* Demonstrates glob matching and SQL LIKE pattern matching using pure string logic *)

(* Glob matching: '*' matches any sequence of characters.
   Supports one '*' at most — split on '*' to find prefix/suffix. *)
let glob_match pattern s =
  match String.split_on_char '*' pattern with
  | [exact] -> s = exact
  | [pre; suf] ->
    let slen = String.length s in
    let plen = String.length pre in
    let sufflen = String.length suf in
    slen >= plen + sufflen
    && String.sub s 0 plen = pre
    && String.sub s (slen - sufflen) sufflen = suf
  | _ -> false (* multiple wildcards: not supported in this minimal version *)

(* SQL LIKE matching: '%' = any chars, '_' = exactly one char.
   Recursive implementation mirrors the Rust pattern. *)
let like_match s pattern =
  (* Work on char arrays for O(1) indexing *)
  let sv = Array.of_seq (String.to_seq s) in
  let pv = Array.of_seq (String.to_seq pattern) in
  let sn = Array.length sv in
  let pn = Array.length pv in
  (* rec si pi — match sv[si..] against pv[pi..] *)
  let rec rec_ si pi =
    if pi = pn then si = sn                  (* pattern exhausted *)
    else match pv.(pi) with
    | '%' ->
      (* consume zero or more chars from s *)
      let rec try_pos k =
        if k > sn then false
        else if rec_ k (pi + 1) then true
        else try_pos (k + 1)
      in
      try_pos si
    | '_' ->
      si < sn && rec_ (si + 1) (pi + 1)
    | pc ->
      si < sn && sv.(si) = pc && rec_ (si + 1) (pi + 1)
  in
  rec_ 0 0

let () =
  (* glob *)
  assert (glob_match "*.txt" "hello.txt");
  assert (not (glob_match "*.txt" "hello.rs"));
  assert (glob_match "exact" "exact");
  assert (not (glob_match "exact" "notexact"));
  print_endline "glob_match: ok";

  (* like *)
  assert (like_match "hello" "h%o");
  assert (like_match "hello" "he_lo");
  assert (not (like_match "hello" "world"));
  assert (like_match "hello" "%");
  assert (like_match "" "%");
  assert (like_match "a" "_");
  assert (not (like_match "" "_"));
  print_endline "like_match: ok";

  print_endline "All assertions passed."
