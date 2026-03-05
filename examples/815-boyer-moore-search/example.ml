(* Boyer-Moore-Horspool string search *)

let build_shift pattern =
  let m     = String.length pattern in
  let shift = Array.make 256 m in  (* default: shift by full pattern length *)
  for i = 0 to m - 2 do
    shift.(Char.code pattern.[i]) <- m - 1 - i
  done;
  shift

let bmh_search text pattern =
  let n = String.length text in
  let m = String.length pattern in
  if m = 0 then []
  else if m > n then []
  else begin
    let shift   = build_shift pattern in
    let matches = ref [] in
    let pos     = ref 0 in
    while !pos <= n - m do
      (* Compare right to left *)
      let j = ref (m - 1) in
      while !j >= 0 && pattern.[!j] = text.[!pos + !j] do
        decr j
      done;
      if !j < 0 then
        matches := !pos :: !matches;
      pos := !pos + shift.(Char.code text.[!pos + m - 1])
    done;
    List.rev !matches
  end

let () =
  let text = "ABAAABCDABABCABAB" in
  let pat  = "ABAB" in
  Printf.printf "Text:    %S\n" text;
  Printf.printf "Pattern: %S\n" pat;
  Printf.printf "Matches: [%s]\n"
    (String.concat "; " (List.map string_of_int (bmh_search text pat)));

  let text2 = "aaaaaaaaaa" in
  let pat2  = "aaa" in
  Printf.printf "\nText: %S  Pattern: %S\n" text2 pat2;
  Printf.printf "Matches: [%s]\n"
    (String.concat "; " (List.map string_of_int (bmh_search text2 pat2)))
