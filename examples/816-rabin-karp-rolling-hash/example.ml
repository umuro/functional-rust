(* Rabin-Karp Rolling Hash — O(n+m) expected *)

let base  = 256
let prime = 1_000_000_007

let rabin_karp text pattern =
  let n = String.length text in
  let m = String.length pattern in
  if m = 0 then []
  else if m > n then []
  else begin
    (* Compute base^(m-1) mod prime *)
    let pow = ref 1 in
    for _ = 1 to m - 1 do
      pow := (!pow * base) mod prime
    done;

    (* Compute initial hashes *)
    let hash_p = ref 0 in
    let hash_t = ref 0 in
    for i = 0 to m - 1 do
      hash_p := (!hash_p * base + Char.code pattern.[i]) mod prime;
      hash_t := (!hash_t * base + Char.code text.[i])    mod prime
    done;

    let matches = ref [] in
    for i = 0 to n - m do
      if !hash_t = !hash_p then begin
        (* Verify (avoid false positives) *)
        let ok = ref true in
        for j = 0 to m - 1 do
          if text.[i+j] <> pattern.[j] then ok := false
        done;
        if !ok then matches := i :: !matches
      end;
      (* Roll hash *)
      if i < n - m then begin
        hash_t := ((!hash_t - Char.code text.[i] * !pow mod prime + prime) * base
                   + Char.code text.[i + m]) mod prime
      end
    done;
    List.rev !matches
  end

let () =
  let text = "ABCDABABCDABCDAB" in
  let pat  = "ABCD" in
  Printf.printf "Text:    %S\n" text;
  Printf.printf "Pattern: %S\n" pat;
  Printf.printf "Matches: [%s]\n"
    (String.concat "; " (List.map string_of_int (rabin_karp text pat)));

  let m2 = rabin_karp "aaabaaabaa" "aab" in
  Printf.printf "\"aab\" in \"aaabaaabaa\": [%s]\n"
    (String.concat "; " (List.map string_of_int m2))
