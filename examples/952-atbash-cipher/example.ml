(* Atbash cipher — maps a<->z, b<->y, ..., digits pass through *)

(* Idiomatic OCaml: filter_map on a Seq, then chunk into groups of 5 *)
let atbash_char c =
  if c >= 'a' && c <= 'z' then
    Some (Char.chr (Char.code 'z' - (Char.code c - Char.code 'a')))
  else if c >= '0' && c <= '9' then Some c
  else None

let encode s =
  let chars = String.to_seq (String.lowercase_ascii s)
    |> Seq.filter_map atbash_char
    |> List.of_seq in
  let rec group = function
    | [] -> []
    | cs ->
      let chunk = List.filteri (fun j _ -> j < 5) cs in
      let rest  = List.filteri (fun j _ -> j >= 5) cs in
      String.init (List.length chunk) (List.nth chunk)
      :: group rest
  in
  String.concat " " (group chars)

(* Decode is the same operation — atbash is self-inverse *)
let decode s =
  String.to_seq s
  |> Seq.filter (fun c -> c <> ' ')
  |> Seq.filter_map atbash_char
  |> String.of_seq

let () =
  assert (encode "Testing, 1 2 3, testing." = "gvhgr mt123 gvhgr mt");
  assert (encode "Hello, World!" = "svool dliow");
  assert (encode "abcde" = "zyxwv");
  assert (encode "abcdefghij" = "zyxwv utsrq");
  assert (decode "svool" = "hello");
  assert (decode "gvhgr mt" = "testing");
  print_endline "ok"
