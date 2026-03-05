(* Polynomial Rolling Hash in OCaml *)

let base = 31
let modp = 1_000_000_007

(* Build prefix hash array and powers for string s *)
(* h.(i) = hash of s[0..i-1], h.(0) = 0 *)
let build_hash (s : string) : int array * int array =
  let n = String.length s in
  let h = Array.make (n + 1) 0 in
  let pw = Array.make (n + 1) 1 in
  for i = 0 to n - 1 do
    let c = Char.code s.[i] - Char.code 'a' + 1 in
    h.(i + 1) <- (h.(i) * base + c) mod modp;
    pw.(i + 1) <- pw.(i) * base mod modp
  done;
  (h, pw)

(* Hash of s[l..r) (0-indexed, exclusive r) *)
let substring_hash (h : int array) (pw : int array) (l : int) (r : int) : int =
  (h.(r) - h.(l) * pw.(r - l) mod modp + modp * modp) mod modp

(* Rabin-Karp: find all occurrences of pattern in text *)
let rabin_karp (pattern : string) (text : string) : int list =
  let m = String.length pattern and n = String.length text in
  if m > n then []
  else begin
    let (ph, ppw) = build_hash pattern in
    let (th, tpw) = build_hash text in
    let pat_hash = substring_hash ph ppw 0 m in
    let results = ref [] in
    for i = 0 to n - m do
      let win_hash = substring_hash th tpw i (i + m) in
      (* Hash match: verify to avoid collisions *)
      if win_hash = pat_hash && String.sub text i m = pattern then
        results := i :: !results
    done;
    List.rev !results
  end

let () =
  let text = "abcabcabc" in
  let pattern = "abc" in
  let matches = rabin_karp pattern text in
  Printf.printf "rabin_karp(%S, %S) = [%s]\n" pattern text
    (String.concat "; " (List.map string_of_int matches));

  (* Show substring hashes equal for same content *)
  let s = "abcabc" in
  let (h, pw) = build_hash s in
  let h1 = substring_hash h pw 0 3 in
  let h2 = substring_hash h pw 3 6 in
  Printf.printf "hash('abc' at 0) = %d, hash('abc' at 3) = %d, equal=%b\n"
    h1 h2 (h1 = h2)
