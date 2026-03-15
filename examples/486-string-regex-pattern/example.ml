(* 486. Pattern matching without regex – OCaml *)
(* Standard Str module provides basic regex *)
let starts_with s pre =
  String.length s >= String.length pre &&
  String.sub s 0 (String.length pre) = pre

let ends_with s suf =
  let ls=String.length s and lsuf=String.length suf in
  ls >= lsuf && String.sub s (ls-lsuf) lsuf = suf

let matches_glob pattern s =
  (* simple: only * wildcard *)
  match String.split_on_char '*' pattern with
  | [prefix; suffix] ->
    starts_with s prefix && ends_with s suffix &&
    String.length s >= String.length prefix + String.length suffix
  | [exact] -> s = exact
  | _ -> false  (* multiple * not supported *)

let () =
  let words = ["hello.txt";"world.rs";"README.md";"test.txt"] in
  let matches = List.filter (matches_glob "*.txt") words in
  Printf.printf "*.txt: %s\n" (String.concat " " matches);

  let has_digit s = String.exists (fun c -> c >= '0' && c <= '9') s in
  List.iter (fun s -> Printf.printf "%s has_digit=%b\n" s (has_digit s))
    ["abc";"abc123";"xyz"]
