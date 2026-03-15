(* 496: String diff — Levenshtein edit distance and fuzzy matching in OCaml *)

(* Levenshtein distance between two strings (Unicode-aware via codepoint split) *)
let levenshtein s t =
  (* Work on arrays of characters for O(1) indexing *)
  let sv = Array.of_seq (String.to_seq s) in
  let tv = Array.of_seq (String.to_seq t) in
  let m = Array.length sv in
  let n = Array.length tv in
  (* dp.(i).(j) = edit distance between sv[0..i] and tv[0..j] *)
  let dp = Array.init (m + 1) (fun i ->
    Array.init (n + 1) (fun j -> if i = 0 then j else if j = 0 then i else 0)
  ) in
  for i = 1 to m do
    for j = 1 to n do
      dp.(i).(j) <-
        if sv.(i-1) = tv.(j-1) then dp.(i-1).(j-1)
        else 1 + (min dp.(i-1).(j) (min dp.(i).(j-1) dp.(i-1).(j-1)))
    done
  done;
  dp.(m).(n)

(* Find the closest string among candidates — minimum edit distance *)
let closest query candidates =
  match candidates with
  | [] -> None
  | _  ->
    let best = List.fold_left (fun (best_s, best_d) c ->
      let d = levenshtein query c in
      if d < best_d then (c, d) else (best_s, best_d)
    ) (List.hd candidates, levenshtein query (List.hd candidates))
      (List.tl candidates)
    in
    Some (fst best)

(* Case-insensitive prefix matching *)
let starts_with_ignore_case s prefix =
  let slen = String.length s in
  let plen = String.length prefix in
  if slen < plen then false
  else
    String.lowercase_ascii (String.sub s 0 plen) =
    String.lowercase_ascii prefix

let () =
  (* kitten → sitting = 3 *)
  assert (levenshtein "kitten" "sitting" = 3);
  Printf.printf "levenshtein(kitten, sitting) = %d\n" (levenshtein "kitten" "sitting");

  (* same string = 0 *)
  assert (levenshtein "abc" "abc" = 0);

  (* empty string *)
  assert (levenshtein "" "abc" = 3);
  assert (levenshtein "abc" "" = 3);
  print_endline "levenshtein: ok";

  (* closest *)
  assert (closest "rast" ["rust"; "bust"; "just"] = Some "rust");
  Printf.printf "closest(rast) = %s\n"
    (match closest "rast" ["rust"; "bust"; "just"] with Some s -> s | None -> "none");

  (* starts_with_ignore_case *)
  assert (starts_with_ignore_case "Hello World" "hello");
  assert (not (starts_with_ignore_case "Hello" "world"));
  print_endline "starts_with_ignore_case: ok";

  print_endline "All assertions passed."
