(* KMP Pattern Matching — O(n+m) *)

let build_prefix pattern =
  let m  = String.length pattern in
  let pi = Array.make m 0 in
  let k  = ref 0 in
  for i = 1 to m - 1 do
    while !k > 0 && pattern.[!k] <> pattern.[i] do
      k := pi.(!k - 1)
    done;
    if pattern.[!k] = pattern.[i] then incr k;
    pi.(i) <- !k
  done;
  pi

let kmp_search text pattern =
  let n = String.length text in
  let m = String.length pattern in
  if m = 0 then []
  else begin
    let pi      = build_prefix pattern in
    let matches = ref [] in
    let q       = ref 0 in
    for i = 0 to n - 1 do
      while !q > 0 && pattern.[!q] <> text.[i] do
        q := pi.(!q - 1)
      done;
      if pattern.[!q] = text.[i] then incr q;
      if !q = m then begin
        matches := (i - m + 1) :: !matches;
        q       := pi.(m - 1)
      end
    done;
    List.rev !matches
  end

let () =
  let text    = "aabaacaadaabaaba" in
  let pattern = "aaba" in
  let matches = kmp_search text pattern in
  Printf.printf "Text:    %S\n" text;
  Printf.printf "Pattern: %S\n" pattern;
  Printf.printf "Matches at positions: [%s]\n"
    (String.concat "; " (List.map string_of_int matches));

  let matches2 = kmp_search "abcabcabc" "abc" in
  Printf.printf "\"abc\" in \"abcabcabc\": [%s]\n"
    (String.concat "; " (List.map string_of_int matches2))
