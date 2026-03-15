(* Pangram Check — Set-based string analysis *)

module CS = Set.Make(Char)

let alphabet =
  List.init 26 (fun i -> Char.chr (i + Char.code 'a'))
  |> CS.of_list

(* Idiomatic OCaml: filter to alpha chars, build set, check subset *)
let is_pangram s =
  let chars = s |> String.lowercase_ascii |> String.to_seq
    |> Seq.filter (fun c -> c >= 'a' && c <= 'z')
    |> CS.of_seq in
  CS.subset alphabet chars

(* Recursive: check each letter exists *)
let is_pangram_recursive s =
  let lower = String.lowercase_ascii s in
  let rec check c =
    if c > 'z' then true
    else String.contains lower c && check (Char.chr (Char.code c + 1))
  in
  check 'a'

let () =
  assert (is_pangram "The quick brown fox jumps over the lazy dog");
  assert (not (is_pangram "Hello world"));
  assert (is_pangram_recursive "The quick brown fox jumps over the lazy dog");
  assert (not (is_pangram_recursive "abc"));
  Printf.printf "All pangram tests passed!\n"
