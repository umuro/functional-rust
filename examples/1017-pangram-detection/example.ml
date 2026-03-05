(* Pangram Detection *)
(* Bit set for alphabet coverage check — early termination *)

let is_pangram s =
  let bits = ref 0 in
  let all_letters = (1 lsl 26) - 1 in
  String.iter (fun c ->
    let c = Char.lowercase_ascii c in
    if c >= 'a' && c <= 'z' then
      bits := !bits lor (1 lsl (Char.code c - Char.code 'a'))
  ) s;
  !bits = all_letters
