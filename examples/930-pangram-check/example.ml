(* 930: Pangram Check — a sentence using every letter of the alphabet

   Three approaches:
   1. Set-based (idiomatic OCaml with Char.Set or Hashtbl)
   2. Bitflag approach — u32 bitmask for 26 letters
   3. Recursive approach — check each letter a..z *)

module CharSet = Set.Make(Char)

(* ── Approach 1: Set-based ────────────────────────────────────────────────── *)

let is_pangram sentence =
  let lower = String.lowercase_ascii sentence in
  let unique =
    String.fold_left (fun acc c ->
      if c >= 'a' && c <= 'z' then CharSet.add c acc else acc
    ) CharSet.empty lower
  in
  CharSet.cardinal unique = 26

(* ── Approach 2: Bitflag — 26-bit integer ─────────────────────────────────── *)

let is_pangram_bitflag sentence =
  let all_26 = (1 lsl 26) - 1 in
  let seen =
    String.fold_left (fun acc c ->
      let c = Char.lowercase_ascii c in
      if c >= 'a' && c <= 'z' then
        let bit = Char.code c - Char.code 'a' in
        acc lor (1 lsl bit)
      else acc
    ) 0 sentence
  in
  seen = all_26

(* ── Approach 3: Recursive check ─────────────────────────────────────────── *)

let is_pangram_recursive sentence =
  let lower = String.lowercase_ascii sentence in
  let rec has_all c =
    if c > 'z' then true
    else
      let found = String.contains lower c in
      found && has_all (Char.chr (Char.code c + 1))
  in
  has_all 'a'

(* ── Approach 4: Functional with List.for_all ─────────────────────────────── *)

let is_pangram_functional sentence =
  let lower = String.lowercase_ascii sentence in
  let letters = List.init 26 (fun i -> Char.chr (Char.code 'a' + i)) in
  List.for_all (fun c -> String.contains lower c) letters

let () =
  let pangram = "The quick brown fox jumps over the lazy dog" in
  let not_pangram = "Hello world" in

  assert (is_pangram pangram);
  assert (not (is_pangram not_pangram));
  assert (not (is_pangram ""));
  assert (not (is_pangram "The quick brown fo jumps over the lazy dog"));
  assert (is_pangram "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG");
  assert (is_pangram "The 1 quick brown fox jumps! over the 2 lazy dogs.");

  assert (is_pangram_bitflag pangram);
  assert (not (is_pangram_bitflag not_pangram));

  assert (is_pangram_recursive pangram);
  assert (not (is_pangram_recursive "abc"));

  assert (is_pangram_functional pangram);
  assert (not (is_pangram_functional not_pangram));

  (* All four methods agree *)
  let sentences = [pangram; not_pangram; ""; "Pack my box with five dozen liquor jugs"] in
  List.iter (fun s ->
    let r1 = is_pangram s in
    assert (is_pangram_bitflag s = r1);
    assert (is_pangram_recursive s = r1);
    assert (is_pangram_functional s = r1)
  ) sentences;

  print_endline "930-pangram-check: all tests passed"
